//! Mechanically lifted functions (scripts/lift_run.py), module h002. Each one is checked against the original
//! in the world harness (tests/world_h002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x682b0 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000682b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x682b0;
    loop {
        match pc {
        0x682b0 => {
            // block 682b0
            #[cfg(feature = "trace")] eprintln!("f_000682b0 pc 682b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 682b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 682b1 mov rbp,rsp
            rbp = rsp;
            // 682b4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 682b6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 682b8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 682ba push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 682bc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 682bd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 682be mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 682c1 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // 682c4 mov rbx,rdi
            rbx = rdi;
            // 682c7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 682ca mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 682cf mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 682d2 call QWORD PTR [rax+0xbd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 682d8 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 682db mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 682e2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 682e5 mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // 682e8 lea rcx,[rip+0x99a01] # 0x101cf0
            rcx = m.env.data_va(91325);
            // 682ef mov esi,DWORD PTR [rcx+r13*4]
            rsi = (m.r32(rcx.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 682f3 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 682f7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 682fc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 682ff mov rdi,rbx
            rdi = rbx;
            // 68302 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68305 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 68308 lea rcx,[rip+0x99a11] # 0x101d20
            rcx = m.env.data_va(91373);
            // 6830f mov cl,BYTE PTR [rcx+r13*4]
            rcx = (rcx & !0xff) | ((m.r8(rcx.wrapping_add(r13.wrapping_mul(4))) as u64) as u8 as u64);
            // 68313 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 68315 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 68318 cmp eax,r12d
            fl = sub::<32>((rax as u32 as u64), (r12 as u32 as u64)).1;
            // 6831b je 0x68334
            if fl.e() { pc = 0x68334; continue; }
            pc = 0x6831d;
        }
        0x6831d => {
            // block 6831d
            #[cfg(feature = "trace")] eprintln!("f_000682b0 pc 6831d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6831d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 68320 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 68325 mov rdi,rbx
            rdi = rbx;
            // 68328 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 6832b call QWORD PTR [rax+0xbd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68331 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x68334;
        }
        0x68334 => {
            // block 68334
            #[cfg(feature = "trace")] eprintln!("f_000682b0 pc 68334 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68334 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 68337 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6833b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6833c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6833e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 68340 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68342 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 68344 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68345 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a71a size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001a71a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a71a;
    loop {
        match pc {
        0x1a71a => {
            // block 1a71a
            #[cfg(feature = "trace")] eprintln!("f_0001a71a pc 1a71a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a71a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a71b mov rbp,rsp
            rbp = rsp;
            // 1a71e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1a720 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a722 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1a724 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1a726 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a727 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a72b mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1a72f mov r12,rdi
            r12 = rdi;
            // 1a732 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1a736 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a73b sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1a73e sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1a742 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1a745 mov rax,rbx
            rax = rbx;
            // 1a748 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1a74c je 0x1a75a
            if fl.e() { pc = 0x1a75a; continue; }
            pc = 0x1a74e;
        }
        0x1a74e => {
            // block 1a74e
            #[cfg(feature = "trace")] eprintln!("f_0001a71a pc 1a74e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a74e mov rdi,r12
            rdi = r12;
            // 1a751 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x1a75a => {
            // block 1a75a
            #[cfg(feature = "trace")] eprintln!("f_0001a71a pc 1a75a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a75a lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1a75f mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1a764 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1a768 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1a76b mov rcx,r15
            rcx = r15;
            // 1a76e sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1a772 movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1a77c cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1a77f ja 0x1a78d
            if fl.a() { pc = 0x1a78d; continue; }
            pc = 0x1a781;
        }
        0x1a781 => {
            // block 1a781
            #[cfg(feature = "trace")] eprintln!("f_0001a71a pc 1a781 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a781 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1a784 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1a787 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1a78b jmp 0x1a797
            pc = 0x1a797; continue;
        }
        0x1a78d => {
            // block 1a78d
            #[cfg(feature = "trace")] eprintln!("f_0001a71a pc 1a78d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a78d movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1a797;
        }
        0x1a797 => {
            // block 1a797
            #[cfg(feature = "trace")] eprintln!("f_0001a71a pc 1a797 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a797 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a79c mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1a7a0 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1a7a3 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1a7a7 mov r13,r14
            r13 = r14;
            // 1a7aa sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1a7ae mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1a7b6 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1a7ba xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1a7bc test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1a7bf je 0x1a7d1
            if fl.e() { pc = 0x1a7d1; continue; }
            pc = 0x1a7c1;
        }
        0x1a7c1 => {
            // block 1a7c1
            #[cfg(feature = "trace")] eprintln!("f_0001a71a pc 1a7c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a7c1 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1a7c9 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a7ce mov rbx,rax
            rbx = rax;
            pc = 0x1a7d1;
        }
        0x1a7d1 => {
            // block 1a7d1
            #[cfg(feature = "trace")] eprintln!("f_0001a71a pc 1a7d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a7d1 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1a7d5 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1a7d9 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1a7dd mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1a7df mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1a7e3 lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1a7e8 mov rdi,rbx
            rdi = rbx;
            // 1a7eb mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1a7ef mov rsi,r15
            rsi = r15;
            // 1a7f2 mov rdx,r14
            rdx = r14;
            // 1a7f5 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a7fa mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1a7fe mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1a802 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1a807 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1a80b mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1a80f mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1a813 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1a818 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1a81c mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1a820 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1a824 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1a828 call 0xeb97e
            { let r = { let t = m.env.stub_va(0x14b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a82d add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a831 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a832 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1a834 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1a836 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a838 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a83a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a83b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68106 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068106<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68106;
    loop {
        match pc {
        0x68106 => {
            // block 68106
            #[cfg(feature = "trace")] eprintln!("f_00068106 pc 68106 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68106 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68107 mov rbp,rsp
            rbp = rsp;
            // 6810a mov rax,rsi
            rax = rsi;
            // 6810d mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 68110 mov rcx,QWORD PTR [rcx+0xb8]
            rcx = m.r64(rcx.wrapping_add(0xb8)) as u64;
            // 68117 mov edx,edx
            rdx = ((rdx as u32 as u64)) as u32 as u64;
            // 68119 lea rsi,[rip+0x99b70] # 0x101c90
            rsi = m.env.data_va(91229);
            // 68120 mov esi,DWORD PTR [rsi+rdx*4]
            rsi = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 68123 mov rdx,rax
            rdx = rax;
            // 68126 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68127 jmp rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1afb0 size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001afb0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1afb0;
    loop {
        match pc {
        0x1afb0 => {
            // block 1afb0
            #[cfg(feature = "trace")] eprintln!("f_0001afb0 pc 1afb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1afb0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1afb1 mov rbp,rsp
            rbp = rsp;
            // 1afb4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1afb6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1afb8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1afba push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1afbc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1afbd sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1afc1 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1afc5 mov r12,rdi
            r12 = rdi;
            // 1afc8 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1afcc mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1afd1 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1afd4 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1afd8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1afdb mov rax,rbx
            rax = rbx;
            // 1afde shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1afe2 je 0x1aff0
            if fl.e() { pc = 0x1aff0; continue; }
            pc = 0x1afe4;
        }
        0x1afe4 => {
            // block 1afe4
            #[cfg(feature = "trace")] eprintln!("f_0001afb0 pc 1afe4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1afe4 mov rdi,r12
            rdi = r12;
            // 1afe7 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x1aff0 => {
            // block 1aff0
            #[cfg(feature = "trace")] eprintln!("f_0001afb0 pc 1aff0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1aff0 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1aff5 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1affa mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1affe sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1b001 mov rcx,r15
            rcx = r15;
            // 1b004 sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1b008 movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1b012 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1b015 ja 0x1b023
            if fl.a() { pc = 0x1b023; continue; }
            pc = 0x1b017;
        }
        0x1b017 => {
            // block 1b017
            #[cfg(feature = "trace")] eprintln!("f_0001afb0 pc 1b017 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b017 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1b01a cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1b01d cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1b021 jmp 0x1b02d
            pc = 0x1b02d; continue;
        }
        0x1b023 => {
            // block 1b023
            #[cfg(feature = "trace")] eprintln!("f_0001afb0 pc 1b023 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b023 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1b02d;
        }
        0x1b02d => {
            // block 1b02d
            #[cfg(feature = "trace")] eprintln!("f_0001afb0 pc 1b02d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b02d mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b032 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1b036 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1b039 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1b03d mov r13,r14
            r13 = r14;
            // 1b040 sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1b044 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1b04c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1b050 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1b052 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1b055 je 0x1b067
            if fl.e() { pc = 0x1b067; continue; }
            pc = 0x1b057;
        }
        0x1b057 => {
            // block 1b057
            #[cfg(feature = "trace")] eprintln!("f_0001afb0 pc 1b057 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b057 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1b05f call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b064 mov rbx,rax
            rbx = rax;
            pc = 0x1b067;
        }
        0x1b067 => {
            // block 1b067
            #[cfg(feature = "trace")] eprintln!("f_0001afb0 pc 1b067 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b067 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1b06b mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1b06f mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1b073 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1b075 mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1b079 lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1b07e mov rdi,rbx
            rdi = rbx;
            // 1b081 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1b085 mov rsi,r15
            rsi = r15;
            // 1b088 mov rdx,r14
            rdx = r14;
            // 1b08b call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b090 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1b094 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1b098 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1b09d mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1b0a1 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1b0a5 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1b0a9 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1b0ae mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1b0b2 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1b0b6 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1b0ba lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1b0be call 0xeb978
            { let r = { let t = m.env.stub_va(0x145); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b0c3 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b0c7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1b0c8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1b0ca pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1b0cc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1b0ce pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1b0d0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1b0d1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42f68 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042f68<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42f68;
    loop {
        match pc {
        0x42f68 => {
            // block 42f68
            #[cfg(feature = "trace")] eprintln!("f_00042f68 pc 42f68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42f68 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42f69 mov rbp,rsp
            rbp = rsp;
            // 42f6c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42f6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42f70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42f71 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42f72 mov r14,rdx
            r14 = rdx;
            // 42f75 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 42f78 mov rbx,rdi
            rbx = rdi;
            // 42f7b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f7e call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42f84 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 42f86 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 42f88 cmp r15d,0xf
            fl = sub::<32>((r15 as u32 as u64), 0xfu64).1;
            // 42f8c ja 0x42fd5
            if fl.a() { pc = 0x42fd5; continue; }
            pc = 0x42f8e;
        }
        0x42f8e => {
            // block 42f8e
            #[cfg(feature = "trace")] eprintln!("f_00042f68 pc 42f8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42f8e test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 42f90 je 0x42fd5
            if fl.e() { pc = 0x42fd5; continue; }
            pc = 0x42f92;
        }
        0x42f92 => {
            // block 42f92
            #[cfg(feature = "trace")] eprintln!("f_00042f68 pc 42f92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42f92 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 42f99 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f9c mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 42f9f lea rcx,[rip+0xbd52a] # 0x1004d0
            rcx = m.env.data_va(85149);
            // 42fa6 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42fa9 lea rsi,[rip+0xbd570] # 0x100520
            rsi = m.env.data_va(85229);
            // 42fb0 mov r8d,DWORD PTR [rsi+rdx*4]
            r8 = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42fb4 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 42fb8 mov esi,0x905
            rsi = (0x905u64) as u32 as u64;
            // 42fbd mov rdi,rbx
            rdi = rbx;
            // 42fc0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42fc3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42fc5 je 0x42fd3
            if fl.e() { pc = 0x42fd3; continue; }
            pc = 0x42fc7;
        }
        0x42fc7 => {
            // block 42fc7
            #[cfg(feature = "trace")] eprintln!("f_00042f68 pc 42fc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42fc7 cmp DWORD PTR [rbp-0x1c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x1u64).1;
            // 42fcb sete BYTE PTR [r14]
            m.w8(r14, ((fl.e() as u64)) as u8);
            // 42fcf mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 42fd1 jmp 0x42fd5
            pc = 0x42fd5; continue;
        }
        0x42fd3 => {
            // block 42fd3
            #[cfg(feature = "trace")] eprintln!("f_00042f68 pc 42fd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42fd3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42fd5;
        }
        0x42fd5 => {
            // block 42fd5
            #[cfg(feature = "trace")] eprintln!("f_00042f68 pc 42fd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42fd5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42fd9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42fda pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42fdc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42fde pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42fdf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x57d10 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00057d10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x57d10;
    loop {
        match pc {
        0x57d10 => {
            // block 57d10
            #[cfg(feature = "trace")] eprintln!("f_00057d10 pc 57d10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57d10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 57d11 mov rbp,rsp
            rbp = rsp;
            // 57d14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 57d16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 57d17 mov rbx,rsi
            rbx = rsi;
            // 57d1a mov r14,rdi
            r14 = rdi;
            // 57d1d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 57d20 je 0x57d45
            if fl.e() { pc = 0x57d45; continue; }
            pc = 0x57d22;
        }
        0x57d22 => {
            // block 57d22
            #[cfg(feature = "trace")] eprintln!("f_00057d10 pc 57d22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57d22 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 57d25 mov rdi,r14
            rdi = r14;
            // 57d28 call 0x57d10
            { let r = { let t = m.env.code_va(0x57d10); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57d2d mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 57d31 mov rdi,r14
            rdi = r14;
            // 57d34 call 0x57d10
            { let r = { let t = m.env.code_va(0x57d10); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57d39 mov rdi,rbx
            rdi = rbx;
            // 57d3c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 57d3d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 57d3f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 57d40 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x57d45 => {
            // block 57d45
            #[cfg(feature = "trace")] eprintln!("f_00057d10 pc 57d45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57d45 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 57d46 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 57d48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 57d49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dc9c size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dc9c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2dc9c;
    loop {
        match pc {
        0x2dc9c => {
            // block 2dc9c
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dc9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dc9c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dc9d mov rbp,rsp
            rbp = rsp;
            // 2dca0 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2dca4 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dca7 je 0x2dcda
            if fl.e() { pc = 0x2dcda; continue; }
            pc = 0x2dca9;
        }
        0x2dca9 => {
            // block 2dca9
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dca9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dca9 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 2dcab jmp 0x2dcb0
            pc = 0x2dcb0; continue;
        }
        0x2dcad => {
            // block 2dcad
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcad mov rax,rdx
            rax = rdx;
            pc = 0x2dcb0;
        }
        0x2dcb0 => {
            // block 2dcb0
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcb0 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 2dcb3 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 2dcb5 jae 0x2dcc1
            if fl.ae() { pc = 0x2dcc1; continue; }
            pc = 0x2dcb7;
        }
        0x2dcb7 => {
            // block 2dcb7
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcb7 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 2dcba test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2dcbd jne 0x2dcad
            if fl.ne() { pc = 0x2dcad; continue; }
            pc = 0x2dcbf;
        }
        0x2dcbf => {
            // block 2dcbf
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcbf jmp 0x2dce6
            pc = 0x2dce6; continue;
        }
        0x2dcc1 => {
            // block 2dcc1
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcc1 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 2dcc3 jae 0x2dcee
            if fl.ae() { pc = 0x2dcee; continue; }
            pc = 0x2dcc5;
        }
        0x2dcc5 => {
            // block 2dcc5
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcc5 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2dcc9 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2dccc jne 0x2dcad
            if fl.ne() { pc = 0x2dcad; continue; }
            pc = 0x2dcce;
        }
        0x2dcce => {
            // block 2dcce
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcce mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2dcd1 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 2dcd5 mov rsi,rax
            rsi = rax;
            // 2dcd8 jmp 0x2dcf1
            pc = 0x2dcf1; continue;
        }
        0x2dcda => {
            // block 2dcda
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcda add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 2dcde mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 2dce1 mov rsi,rdi
            rsi = rdi;
            // 2dce4 jmp 0x2dcf1
            pc = 0x2dcf1; continue;
        }
        0x2dce6 => {
            // block 2dce6
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dce6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dce6 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2dce9 mov rsi,rax
            rsi = rax;
            // 2dcec jmp 0x2dcf1
            pc = 0x2dcf1; continue;
        }
        0x2dcee => {
            // block 2dcee
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcee mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x2dcf1;
        }
        0x2dcf1 => {
            // block 2dcf1
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcf1 mov rax,rsi
            rax = rsi;
            // 2dcf4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dcf5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x31ccc size=1046
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f, stub:0x961
pub fn f_00031ccc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x31ccc;
    loop {
        match pc {
        0x31ccc => {
            // block 31ccc
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31ccc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31ccc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 31ccd mov rbp,rsp
            rbp = rsp;
            // 31cd0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 31cd2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31cd4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 31cd6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 31cd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31cd9 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 31ce0 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 31ce3 mov r15,rsi
            r15 = rsi;
            // 31ce6 mov r13,rdi
            r13 = rdi;
            // 31ce9 mov r14,QWORD PTR [rip+0x10e6e8] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // 31cf0 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // 31cf3 mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 31cf7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 31cfa mov rdi,r15
            rdi = r15;
            // 31cfd call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31d03 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31d05 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31d07 je 0x3208c
            if fl.e() { pc = 0x3208c; continue; }
            pc = 0x31d0d;
        }
        0x31d0d => {
            // block 31d0d
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31d0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31d0d mov QWORD PTR [rbp-0xc0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r13) as u64);
            // 31d14 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 31d17 lea rcx,[rip+0xce142] # 0xffe60
            rcx = m.env.data_va(83501);
            // 31d1e mov r14d,DWORD PTR [rcx+rax*4]
            r14 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 31d22 lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 31d29 mov QWORD PTR [rbp-0x88],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0x0u64) as u64);
            // 31d34 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 31d3f mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 31d46 lea r12,[rbp-0x98]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 31d4d lea r13,[rbp-0x9c]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff64);
            pc = 0x31d54;
        }
        0x31d54 => {
            // block 31d54
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31d54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31d54 lea eax,[r14+rbx*1]
            rax = (r14.wrapping_add(rbx)) as u32 as u64;
            // 31d58 mov DWORD PTR [rbp-0x9c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff64), ((rax as u32 as u64)) as u32);
            // 31d5e mov rdi,r12
            rdi = r12;
            // 31d61 mov rsi,r13
            rsi = r13;
            // 31d64 call 0xebb22
            { let r = { let t = m.env.stub_va(0x2ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31d69 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 31d6b cmp ebx,0x11
            fl = sub::<32>((rbx as u32 as u64), 0x11u64).1;
            // 31d6e jl 0x31d54
            if fl.l() { pc = 0x31d54; continue; }
            pc = 0x31d70;
        }
        0x31d70 => {
            // block 31d70
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31d70 lea rbx,[rbp-0xb0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31d77 mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 31d82 mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // 31d8d mov QWORD PTR [rbp-0xb8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rbx) as u64);
            // 31d94 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 31d97 mov rax,QWORD PTR [rax+0xc90]
            rax = m.r64(rax.wrapping_add(0xc90)) as u64;
            // 31d9e lea rsi,[rbp-0x98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 31da5 lea rdx,[rbp-0xb8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 31dac mov rdi,r15
            rdi = r15;
            // 31daf call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31db1 mov r14,QWORD PTR [rip+0x10e620] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // 31db8 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // 31dbb mov r15,QWORD PTR [rbp-0xc0]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // 31dc2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31dc4 je 0x31e4f
            if fl.e() { pc = 0x31e4f; continue; }
            pc = 0x31dca;
        }
        0x31dca => {
            // block 31dca
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dca cmp QWORD PTR [rbp-0xa8],0x11
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, 0x11u64).1;
            // 31dd2 jne 0x31e4f
            if fl.ne() { pc = 0x31e4f; continue; }
            pc = 0x31dd4;
        }
        0x31dd4 => {
            // block 31dd4
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dd4 mov rcx,QWORD PTR [rbp-0xb8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // 31ddb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31ddd cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 31de0 je 0x31e56
            if fl.e() { pc = 0x31e56; continue; }
            pc = 0x31de2;
        }
        0x31de2 => {
            // block 31de2
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31de2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31de2 mov edx,DWORD PTR [rcx+0x20]
            rdx = (m.r32(rcx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 31de5 mov DWORD PTR [rbp+rax*4-0x80],edx
            m.w32(rbp.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff80), ((rdx as u32 as u64)) as u32);
            // 31de9 mov rdx,QWORD PTR [rcx+0x8]
            rdx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 31ded test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 31df0 je 0x31dff
            if fl.e() { pc = 0x31dff; continue; }
            pc = 0x31df2;
        }
        0x31df2 => {
            // block 31df2
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31df2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31df2 mov rcx,rdx
            rcx = rdx;
            // 31df5 mov rdx,QWORD PTR [rcx]
            rdx = m.r64(rcx) as u64;
            // 31df8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 31dfb jne 0x31df2
            if fl.ne() { pc = 0x31df2; continue; }
            pc = 0x31dfd;
        }
        0x31dfd => {
            // block 31dfd
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dfd jmp 0x31e0e
            pc = 0x31e0e; continue;
        }
        0x31dff => {
            // block 31dff
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dff mov rdx,rcx
            rdx = rcx;
            pc = 0x31e02;
        }
        0x31e02 => {
            // block 31e02
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e02 mov rcx,QWORD PTR [rdx+0x10]
            rcx = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 31e06 cmp QWORD PTR [rcx],rdx
            fl = sub::<64>(m.r64(rcx) as u64, rdx).1;
            // 31e09 mov rdx,rcx
            rdx = rcx;
            // 31e0c jne 0x31e02
            if fl.ne() { pc = 0x31e02; continue; }
            pc = 0x31e0e;
        }
        0x31e0e => {
            // block 31e0e
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e0e inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 31e11 cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 31e14 jne 0x31de2
            if fl.ne() { pc = 0x31de2; continue; }
            pc = 0x31e16;
        }
        0x31e16 => {
            // block 31e16
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e16 mov ecx,DWORD PTR [rbp-0x80]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            // 31e19 mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // 31e1c mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 31e1e shr edx,0x4
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x4u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 31e21 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 31e23 shr esi,0xc
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xcu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 31e26 movd xmm0,ecx
            x0 = ((rcx as u32 as u64)) as u32 as u128;
            // 31e2a shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 31e2d movd xmm1,ecx
            x1 = ((rcx as u32 as u64)) as u32 as u128;
            // 31e31 movd xmm2,edx
            x2 = ((rdx as u32 as u64)) as u32 as u128;
            // 31e35 punpckldq xmm2,xmm1
            { let a = x2; let b = x1; x2 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e39 movd xmm1,esi
            x1 = ((rsi as u32 as u64)) as u32 as u128;
            // 31e3d punpckldq xmm0,xmm1
            { let a = x0; let b = x1; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e41 punpckldq xmm0,xmm2
            { let a = x0; let b = x2; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e45 pand xmm0,XMMWORD PTR [rip+0xce003] # 0xffe50
            x0 &= m.r128(m.env.data_va(83485));
            // 31e4d jmp 0x31e5d
            pc = 0x31e5d; continue;
        }
        0x31e4f => {
            // block 31e4f
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e4f xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31e51 jmp 0x32066
            pc = 0x32066; continue;
        }
        0x31e56 => {
            // block 31e56
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e56 pxor xmm0,xmm0
            x0 = 0;
            // 31e5a mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            pc = 0x31e5d;
        }
        0x31e5d => {
            // block 31e5d
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e5d movdqu XMMWORD PTR [r15+0x8],xmm0
            m.w128(r15.wrapping_add(0x8), x0);
            // 31e63 lea rbx,[r15+0x30]
            rbx = r15.wrapping_add(0x30);
            // 31e67 mov rcx,QWORD PTR [r15+0x30]
            rcx = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31e6b mov rcx,QWORD PTR [rcx+0x30]
            rcx = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 31e6f movsx edx,ax
            rdx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // 31e72 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31e74 mov rdi,rbx
            rdi = rbx;
            // 31e77 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31e79 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31e7c mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31e80 movsx edx,WORD PTR [rbp-0x7a]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff86)) as u64) as u64) as u32 as u64;
            // 31e84 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31e89 mov rdi,rbx
            rdi = rbx;
            // 31e8c call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31e8e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31e91 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31e95 movsx edx,WORD PTR [rbp-0x78]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u64) as u32 as u64;
            // 31e99 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31e9e mov rdi,rbx
            rdi = rbx;
            // 31ea1 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ea3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ea6 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31eaa mov ecx,DWORD PTR [rbp-0x74]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // 31ead cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31eb2 mulsd xmm0,QWORD PTR [rip+0xcdf86] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31eba xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31ebc mov rdi,rbx
            rdi = rbx;
            // 31ebf call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ec1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ec4 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31ec8 mov ecx,DWORD PTR [rbp-0x70]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // 31ecb xorps xmm0,xmm0
            x0 = 0;
            // 31ece cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31ed3 mulsd xmm0,QWORD PTR [rip+0xcdf65] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31edb mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31ee0 mov rdi,rbx
            rdi = rbx;
            // 31ee3 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ee5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ee8 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31eec mov ecx,DWORD PTR [rbp-0x6c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // 31eef xorps xmm0,xmm0
            x0 = 0;
            // 31ef2 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31ef7 mulsd xmm0,QWORD PTR [rip+0xcdf41] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31eff mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 31f04 mov rdi,rbx
            rdi = rbx;
            // 31f07 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f09 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f0c mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f10 mov ecx,DWORD PTR [rbp-0x68]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 31f13 xorps xmm0,xmm0
            x0 = 0;
            // 31f16 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f1b mulsd xmm0,QWORD PTR [rip+0xcdf1d] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f23 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31f28 mov rdi,rbx
            rdi = rbx;
            // 31f2b call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f2d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f30 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f34 mov ecx,DWORD PTR [rbp-0x64]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 31f37 xorps xmm0,xmm0
            x0 = 0;
            // 31f3a cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f3f mulsd xmm0,QWORD PTR [rip+0xcdef9] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f47 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 31f4c mov rdi,rbx
            rdi = rbx;
            // 31f4f call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f51 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f54 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f58 mov ecx,DWORD PTR [rbp-0x60]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 31f5b xorps xmm0,xmm0
            x0 = 0;
            // 31f5e cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f63 mulsd xmm0,QWORD PTR [rip+0xcded5] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f6b mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 31f70 mov rdi,rbx
            rdi = rbx;
            // 31f73 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f75 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f78 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f7c mov ecx,DWORD PTR [rbp-0x5c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // 31f7f xorps xmm0,xmm0
            x0 = 0;
            // 31f82 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f87 mulsd xmm0,QWORD PTR [rip+0xcdeb1] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f8f mov esi,0x6
            rsi = (0x6u64) as u32 as u64;
            // 31f94 mov rdi,rbx
            rdi = rbx;
            // 31f97 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f99 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f9c mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31fa0 mov ecx,DWORD PTR [rbp-0x58]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 31fa3 xorps xmm0,xmm0
            x0 = 0;
            // 31fa6 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31fab mulsd xmm0,QWORD PTR [rip+0xcde8d] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31fb3 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 31fb8 mov rdi,rbx
            rdi = rbx;
            // 31fbb call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31fbd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31fc0 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31fc4 mov ecx,DWORD PTR [rbp-0x54]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64) as u32 as u64;
            // 31fc7 xorps xmm0,xmm0
            x0 = 0;
            // 31fca cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31fcf mulsd xmm0,QWORD PTR [rip+0xcde69] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31fd7 mov esi,0x8
            rsi = (0x8u64) as u32 as u64;
            // 31fdc mov rdi,rbx
            rdi = rbx;
            // 31fdf call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31fe1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31fe4 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31fe8 movsx edx,WORD PTR [rbp-0x50]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64) as u64) as u32 as u64;
            // 31fec mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31ff1 mov rdi,rbx
            rdi = rbx;
            // 31ff4 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ff6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ff9 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31ffd movsx edx,WORD PTR [rbp-0x4e]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb2)) as u64) as u64) as u32 as u64;
            // 32001 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 32006 mov rdi,rbx
            rdi = rbx;
            // 32009 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 3200b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3200e mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 32012 movsx edx,WORD PTR [rbp-0x4c]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u64) as u32 as u64;
            // 32016 mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 3201b mov rdi,rbx
            rdi = rbx;
            // 3201e call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 32020 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 32024 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32026 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 32029 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3202c mov DWORD PTR [r15+0x1c],ecx
            m.w32(r15.wrapping_add(0x1c), ((rcx as u32 as u64)) as u32);
            // 32030 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32032 and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 32035 mov DWORD PTR [r15+0x18],ecx
            m.w32(r15.wrapping_add(0x18), ((rcx as u32 as u64)) as u32);
            // 32039 mov rcx,rax
            rcx = rax;
            // 3203c shr rcx,0x20
            { let (r, f) = shr::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // 32040 mov WORD PTR [r15+0x20],cx
            m.w16(r15.wrapping_add(0x20), ((rcx as u16 as u64)) as u16);
            // 32045 shr rax,0x30
            { let (r, f) = shr::<64>(rax, 0x30u64, fl); rax = r; fl = f; }
            // 32049 mov WORD PTR [r15+0x22],ax
            m.w16(r15.wrapping_add(0x22), ((rax as u16 as u64)) as u16);
            // 3204e mov eax,DWORD PTR [rbp-0x40]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 32051 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 32056 mulsd xmm0,QWORD PTR [rip+0xcddea] # 0xffe48
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83477)) as u128))).to_bits());
            // 3205e movsd QWORD PTR [r15+0x28],xmm0
            m.w64(r15.wrapping_add(0x28), x0 as u64);
            // 32064 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x32066;
        }
        0x32066 => {
            // block 32066
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 32066 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32066 mov rsi,QWORD PTR [rbp-0xb0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 3206d lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 32074 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 32079 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 32080 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 32087 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            pc = 0x3208c;
        }
        0x3208c => {
            // block 3208c
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 3208c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3208c cmp r14,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 32090 jne 0x320dc
            if fl.ne() { pc = 0x320dc; continue; }
            pc = 0x32092;
        }
        0x32092 => {
            // block 32092
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 32092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32092 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 32094 add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 3209b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3209c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 3209e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 320a0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 320a2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 320a4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 320a5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x320a6 => {
            // block 320a6
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 320a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320a6 mov rbx,rax
            rbx = rax;
            pc = 0x320a9;
        }
        0x320a9 => {
            // block 320a9
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 320a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320a9 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 320b0 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 320b7 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 320bc mov rdi,rbx
            rdi = rbx;
            // 320bf call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
        0x320dc => {
            // block 320dc
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 320dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320dc call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10de4 size=372
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010de4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x10de4;
    loop {
        match pc {
        0x10de4 => {
            // block 10de4
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10de4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10de4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10de5 mov rbp,rsp
            rbp = rsp;
            // 10de8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10dea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10dec push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10dee push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10df0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10df1 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10df5 mov r15,rdi
            r15 = rdi;
            // 10df8 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10dfc mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10e00 cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 10e03 jne 0x10f3d
            if fl.ne() { pc = 0x10f3d; continue; }
            pc = 0x10e09;
        }
        0x10e09 => {
            // block 10e09
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10e09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e09 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 10e0d mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 10e10 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10e14 mov rax,rbx
            rax = rbx;
            // 10e17 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10e1a jbe 0x10e6f
            if fl.be() { pc = 0x10e6f; continue; }
            pc = 0x10e1c;
        }
        0x10e1c => {
            // block 10e1c
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10e1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e1c sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10e20 lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 10e24 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 10e28 lea r14,[rax+rcx*1+0x1]
            r14 = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 10e2d sar r14,1
            { let (r, f) = sar::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 10e30 lea rax,[r14*8+0x0]
            rax = r14.wrapping_mul(8);
            // 10e38 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10e3c mov rdi,rbx
            rdi = rbx;
            // 10e3f sub rdi,rax
            { let (r, f) = sub::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 10e42 sub r12,rbx
            { let (r, f) = sub::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // 10e45 mov r13,r12
            r13 = r12;
            // 10e48 shr r13,0x3
            { let (r, f) = shr::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // 10e4c mov rsi,rbx
            rsi = rbx;
            // 10e4f mov rdx,r12
            rdx = r12;
            // 10e52 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10e57 sub r13,r14
            { let (r, f) = sub::<64>(r13, r14);
              r13 = r;
              fl = f; }
            // 10e5a lea r12,[rbx+r13*8]
            r12 = rbx.wrapping_add(r13.wrapping_mul(8));
            // 10e5e mov QWORD PTR [r15+0x10],r12
            m.w64(r15.wrapping_add(0x10), (r12) as u64);
            // 10e62 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10e66 sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10e6a jmp 0x10f39
            pc = 0x10f39; continue;
        }
        0x10e6f => {
            // block 10e6f
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10e6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e6f mov rax,r12
            rax = r12;
            // 10e72 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10e75 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10e79 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10e7f cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10e83 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 10e8d and rax,r14
            { let r = rax & r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 10e90 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10e94 mov rax,QWORD PTR [r15+0x20]
            rax = m.r64(r15.wrapping_add(0x20)) as u64;
            // 10e98 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10e9c mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 10ea0 lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 10ea8 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10ead mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10eb1 mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10eb5 lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 10eb9 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10ebd mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 10ec1 lea rdx,[rax+r14*8]
            rdx = rax.wrapping_add(r14.wrapping_mul(8));
            // 10ec5 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 10ec9 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 10ecc je 0x10f06
            if fl.e() { pc = 0x10f06; continue; }
            pc = 0x10ece;
        }
        0x10ece => {
            // block 10ece
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10ece rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ece mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 10ed1 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 10ed4 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 10ed8 add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 10edc mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10ee0 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 10ee4 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 10ee7 jne 0x10ece
            if fl.ne() { pc = 0x10ece; continue; }
            pc = 0x10ee9;
        }
        0x10ee9 => {
            // block 10ee9
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ee9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 10eec mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10ef0 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10ef4 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10ef8 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10efc mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10f00 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10f04 jmp 0x10f0d
            pc = 0x10f0d; continue;
        }
        0x10f06 => {
            // block 10f06
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f06 mov rsi,rcx
            rsi = rcx;
            // 10f09 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0x10f0d;
        }
        0x10f0d => {
            // block 10f0d
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f0d mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10f10 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 10f14 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 10f18 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 10f1c mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 10f20 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 10f24 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 10f28 mov QWORD PTR [rbp-0x38],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r13) as u64);
            // 10f2c lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10f30 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10f35 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x10f39;
        }
        0x10f39 => {
            // block 10f39
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f39 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x10f3d;
        }
        0x10f3d => {
            // block 10f3d
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f3d mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 10f40 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 10f44 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10f49 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10f4d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10f4e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10f50 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 10f52 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10f54 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10f56 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10f57 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4ae0 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4ae0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4ae0;
    loop {
        match pc {
        0xe4ae0 => {
            // block e4ae0
            #[cfg(feature = "trace")] eprintln!("f_000e4ae0 pc e4ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4ae0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4ae1 mov rbp,rsp
            rbp = rsp;
            // e4ae4 movzx eax,BYTE PTR [rip+0xe3af5] # 0x1c85e0
            rax = (m.r8(m.env.data_va(904621)) as u64) as u32 as u64;
            // e4aeb cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e4aee jne 0xe4af9
            if fl.ne() { pc = 0xe4af9; continue; }
            pc = 0xe4af0;
        }
        0xe4af0 => {
            // block e4af0
            #[cfg(feature = "trace")] eprintln!("f_000e4ae0 pc e4af0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4af0 mov rax,QWORD PTR [rip+0xe3af9] # 0x1c85f0
            rax = m.r64(m.env.data_va(904637)) as u64;
            // e4af7 jmp 0xe4b49
            pc = 0xe4b49; continue;
        }
        0xe4af9 => {
            // block e4af9
            #[cfg(feature = "trace")] eprintln!("f_000e4ae0 pc e4af9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4af9 mov ecx,DWORD PTR [rip+0xe3ae9] # 0x1c85e8
            rcx = (m.r32(m.env.data_va(904629)) as u64) as u32 as u64;
            // e4aff test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // e4b01 jne 0xe4b15
            if fl.ne() { pc = 0xe4b15; continue; }
            pc = 0xe4b03;
        }
        0xe4b03 => {
            // block e4b03
            #[cfg(feature = "trace")] eprintln!("f_000e4ae0 pc e4b03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4b03 lea rdi,[rip+0xe3ada] # 0x1c85e4
            rdi = m.env.data_va(904625);
            // e4b0a call 0xec206
            { let r = { let t = m.env.stub_va(0x9d3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4b0f mov ecx,DWORD PTR [rip+0xe3ad3] # 0x1c85e8
            rcx = (m.r32(m.env.data_va(904629)) as u64) as u32 as u64;
            pc = 0xe4b15;
        }
        0xe4b15 => {
            // block e4b15
            #[cfg(feature = "trace")] eprintln!("f_000e4ae0 pc e4b15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4b15 mov eax,DWORD PTR [rip+0xe3ac9] # 0x1c85e4
            rax = (m.r32(m.env.data_va(904625)) as u64) as u32 as u64;
            // e4b1b imul rax,rax,0x3b9aca00
            { let (r, f) = imul::<64>(rax, 0x3b9aca00u64); rax = r; fl = f; }
            // e4b22 mov ecx,ecx
            rcx = ((rcx as u32 as u64)) as u32 as u64;
            // e4b24 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4b26 div rcx
            { let (q, r) = div::<64>(rdx, rax, rcx); rax = q; rdx = r; }
            // e4b29 mov rcx,rax
            rcx = rax;
            // e4b2c movabs rax,0xde0b6b3a7640000
            rax = 0xde0_b6b3_a764_0000u64;
            // e4b36 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4b38 div rcx
            { let (q, r) = div::<64>(rdx, rax, rcx); rax = q; rdx = r; }
            // e4b3b mov QWORD PTR [rip+0xe3aae],rax # 0x1c85f0
            m.w64(m.env.data_va(904637), (rax) as u64);
            // e4b42 mov BYTE PTR [rip+0xe3a97],0x1 # 0x1c85e0
            m.w8(m.env.data_va(904621), (0x1u64) as u8);
            pc = 0xe4b49;
        }
        0xe4b49 => {
            // block e4b49
            #[cfg(feature = "trace")] eprintln!("f_000e4ae0 pc e4b49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4b49 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4b4a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfb8 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000fb8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfb8;
    loop {
        match pc {
        0xfb8 => {
            // block fb8
            #[cfg(feature = "trace")] eprintln!("f_00000fb8 pc fb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fb8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fb9 mov rbp,rsp
            rbp = rsp;
            // fbc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // fbe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fc0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fc1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // fc2 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // fc5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // fc8 mov rbx,rdi
            rbx = rdi;
            // fcb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fce call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd4 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // fd7 mov r9,QWORD PTR [rcx+0x30]
            r9 = m.r64(rcx.wrapping_add(0x30)) as u64;
            // fdb mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // fde xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // fe0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // fe2 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // fe6 lea rax,[rip+0xfe003] # 0xfeff0
            rax = m.env.data_va(79805);
            // fed mov esi,DWORD PTR [rax+rdx*4]
            rsi = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // ff0 mov ecx,0x380
            rcx = (0x380u64) as u32 as u64;
            // ff5 mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // ffb mov rdi,rbx
            rdi = rbx;
            // ffe mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 1001 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1005 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1006 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1008 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 100a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 100b jmp r9
            { let r = { let t = r9; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15158 size=360
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_00015158<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x15158;
    loop {
        match pc {
        0x15158 => {
            // block 15158
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15158 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15158 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15159 mov rbp,rsp
            rbp = rsp;
            // 1515c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1515e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15160 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15162 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15164 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15165 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15166 mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // 15169 mov r12,rdx
            r12 = rdx;
            // 1516c mov r14,rsi
            r14 = rsi;
            // 1516f mov rbx,rdi
            rbx = rdi;
            // 15172 mov rdi,QWORD PTR [r14+0x80]
            rdi = m.r64(r14.wrapping_add(0x80)) as u64;
            // 15179 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1517c je 0x15294
            if fl.e() { pc = 0x15294; continue; }
            pc = 0x15182;
        }
        0x15182 => {
            // block 15182
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15182 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 15185 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15188 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 1518b cmp QWORD PTR [r14+0x78],0x0
            fl = sub::<64>(m.r64(r14.wrapping_add(0x78)) as u64, 0x0u64).1;
            // 15190 je 0x151d1
            if fl.e() { pc = 0x151d1; continue; }
            pc = 0x15192;
        }
        0x15192 => {
            // block 15192
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15192 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15192 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 15195 je 0x1519c
            if fl.e() { pc = 0x1519c; continue; }
            pc = 0x15197;
        }
        0x15197 => {
            // block 15197
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15197 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15197 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 1519a jle 0x151d1
            if fl.le() { pc = 0x151d1; continue; }
            pc = 0x1519c;
        }
        0x1519c => {
            // block 1519c
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 1519c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1519c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1519f mov rdi,r14
            rdi = r14;
            // 151a2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 151a5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 151a7 jne 0x151d1
            if fl.ne() { pc = 0x151d1; continue; }
            pc = 0x151a9;
        }
        0x151a9 => {
            // block 151a9
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151a9 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 151ad jae 0x151d1
            if fl.ae() { pc = 0x151d1; continue; }
            pc = 0x151af;
        }
        0x151af => {
            // block 151af
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151af mov rdi,QWORD PTR [r14+0x78]
            rdi = m.r64(r14.wrapping_add(0x78)) as u64;
            // 151b3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 151b5 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 151b8 jle 0x151c1
            if fl.le() { pc = 0x151c1; continue; }
            pc = 0x151ba;
        }
        0x151ba => {
            // block 151ba
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151ba movsxd rsi,r13d
            rsi = sx::<32>((r13 as u32 as u64)) as u64;
            // 151bd imul rsi,r12
            { let (r, f) = imul::<64>(rsi, r12); rsi = r; fl = f; }
            pc = 0x151c1;
        }
        0x151c1 => {
            // block 151c1
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151c1 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 151c4 call 0xec1dc
            { let r = { let t = m.env.stub_va(0x9a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 151c9 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 151cb je 0x1526d
            if fl.e() { pc = 0x1526d; continue; }
            pc = 0x151d1;
        }
        0x151d1 => {
            // block 151d1
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151d1 mov QWORD PTR [rbx+0x78],0x0
            m.w64(rbx.wrapping_add(0x78), (0x0u64) as u64);
            // 151d9 mov QWORD PTR [rbx+0x70],0x0
            m.w64(rbx.wrapping_add(0x70), (0x0u64) as u64);
            // 151e1 mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 151e9 mov QWORD PTR [rbx+0x60],0x0
            m.w64(rbx.wrapping_add(0x60), (0x0u64) as u64);
            // 151f1 mov QWORD PTR [rbx+0x58],0x0
            m.w64(rbx.wrapping_add(0x58), (0x0u64) as u64);
            // 151f9 mov QWORD PTR [rbx+0x50],0x0
            m.w64(rbx.wrapping_add(0x50), (0x0u64) as u64);
            // 15201 mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // 15209 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 15211 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 15219 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 15221 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 15229 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 15231 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 15239 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 15241 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 15249 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 15250 mov QWORD PTR [rbx+0x80],0xffffffffffffffff
            m.w64(rbx.wrapping_add(0x80), (0xffff_ffff_ffff_ffffu64) as u64);
            pc = 0x1525b;
        }
        0x1525b => {
            // block 1525b
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 1525b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1525b mov rax,rbx
            rax = rbx;
            // 1525e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 15262 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15263 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15265 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15267 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15269 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1526b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1526c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1526d => {
            // block 1526d
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 1526d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1526d mov rdi,QWORD PTR [r14+0x78]
            rdi = m.r64(r14.wrapping_add(0x78)) as u64;
            // 15271 call 0xec1e2
            { let r = { let t = m.env.stub_va(0x9af); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15276 add r14,0x88
            { let (r, f) = add::<64>(r14, 0x88u64);
              r14 = r;
              fl = f; }
            // 1527d mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // 15282 mov rdi,rbx
            rdi = rbx;
            // 15285 mov rsi,r14
            rsi = r14;
            // 15288 rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            // 1528b mov QWORD PTR [rbx+0x80],rax
            m.w64(rbx.wrapping_add(0x80), (rax) as u64);
            // 15292 jmp 0x1525b
            pc = 0x1525b; continue;
        }
        0x15294 => {
            // block 15294
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15294 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15294 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // 15299 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1529e mov rbx,rax
            rbx = rax;
            // 152a1 mov rdi,rbx
            rdi = rbx;
            // 152a4 call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 152a9 mov rsi,QWORD PTR [rip+0x12af30] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // 152b0 mov rdx,QWORD PTR [rip+0x12aef1] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // 152b7 mov rdi,rbx
            rdi = rbx;
            // 152ba call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd96c size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d96c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd96c;
    loop {
        match pc {
        0xd96c => {
            // block d96c
            #[cfg(feature = "trace")] eprintln!("f_0000d96c pc d96c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d96c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d96d mov rbp,rsp
            rbp = rsp;
            // d970 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d972 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d974 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d975 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d976 mov r14,rdx
            r14 = rdx;
            // d979 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d97c mov rbx,rdi
            rbx = rdi;
            // d97f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d982 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d988 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d98a je 0xd990
            if fl.e() { pc = 0xd990; continue; }
            pc = 0xd98c;
        }
        0xd98c => {
            // block d98c
            #[cfg(feature = "trace")] eprintln!("f_0000d96c pc d98c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d98c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d98e jmp 0xd9c4
            pc = 0xd9c4; continue;
        }
        0xd990 => {
            // block d990
            #[cfg(feature = "trace")] eprintln!("f_0000d96c pc d990 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d990 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // d997 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d99a mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // d99d lea rdx,[rip+0xf1a1c] # 0xff3c0
            rdx = m.env.data_va(80781);
            // d9a4 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // d9a7 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d9ab mov ecx,0x1000000
            rcx = (0x1000000u64) as u32 as u64;
            // d9b0 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // d9b6 mov rdi,rbx
            rdi = rbx;
            // d9b9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9bc cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // d9c0 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0xd9c4;
        }
        0xd9c4 => {
            // block d9c4
            #[cfg(feature = "trace")] eprintln!("f_0000d96c pc d9c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9c4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d9c8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d9c9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d9cb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d9cd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d9ce ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e45e size=202
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e45e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e45e;
    loop {
        match pc {
        0x2e45e => {
            // block 2e45e
            #[cfg(feature = "trace")] eprintln!("f_0002e45e pc 2e45e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e45e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e45f mov rbp,rsp
            rbp = rsp;
            // 2e462 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e464 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e466 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e468 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e469 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 2e46d mov r14,rsi
            r14 = rsi;
            // 2e470 mov r12,rdi
            r12 = rdi;
            // 2e473 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2e477 call QWORD PTR [rax+0x70]
            { let r = { let t = m.r64(rax.wrapping_add(0x70)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e47a mov rcx,QWORD PTR [r12+0x18]
            rcx = m.r64(r12.wrapping_add(0x18)) as u64;
            // 2e47f sub rcx,QWORD PTR [r12+0x10]
            { let (r, f) = sub::<64>(rcx, m.r64(r12.wrapping_add(0x10)) as u64);
              rcx = r;
              fl = f; }
            // 2e484 mov rax,rcx
            rax = rcx;
            // 2e487 shr rax,0x3
            { let (r, f) = shr::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 2e48b mov DWORD PTR [rbp-0x3c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((rax as u32 as u64)) as u32);
            // 2e48e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e490 jle 0x2e51b
            if fl.le() { pc = 0x2e51b; continue; }
            pc = 0x2e496;
        }
        0x2e496 => {
            // block 2e496
            #[cfg(feature = "trace")] eprintln!("f_0002e45e pc 2e496 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e496 shl rcx,0x1d
            { let (r, f) = shl::<64>(rcx, 0x1du64, fl); rcx = r; fl = f; }
            // 2e49a movabs r15,0x100000000
            r15 = 0x0_0001_0000_0000u64;
            // 2e4a4 add r15,rcx
            { let (r, f) = add::<64>(r15, rcx);
              r15 = r;
              fl = f; }
            // 2e4a7 mov rax,r15
            rax = r15;
            // 2e4aa sar rax,0x20
            { let (r, f) = sar::<64>(rax, 0x20u64, fl); rax = r; fl = f; }
            // 2e4ae mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 2e4b3 mul rcx
            { let (lo, hi, f) = mul1::<64>(rax, rcx); rax = lo; rdx = hi; fl = f; }
            // 2e4b6 mov rdi,0xffffffffffffffff
            rdi = 0xffff_ffff_ffff_ffffu64;
            // 2e4bd cmovno rdi,rax
            if fl.no() { rdi = rax; }
            // 2e4c1 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e4c6 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 2e4c9 sar r15,0x1d
            { let (r, f) = sar::<64>(r15, 0x1du64, fl); r15 = r; fl = f; }
            // 2e4cd mov rdi,rax
            rdi = rax;
            // 2e4d0 mov rsi,r15
            rsi = r15;
            // 2e4d3 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e4d8 mov DWORD PTR [rbp-0x3c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0x0u64) as u32);
            // 2e4df mov rbx,QWORD PTR [r12+0x10]
            rbx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2e4e4 mov r15,QWORD PTR [r12+0x18]
            r15 = m.r64(r12.wrapping_add(0x18)) as u64;
            // 2e4e9 mov QWORD PTR [rbp-0x38],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r12) as u64);
            // 2e4ed mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 2e4f1 lea rax,[rbp-0x3c]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // 2e4f5 mov QWORD PTR [rbp-0x28],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rax) as u64);
            // 2e4f9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e4fb cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 2e4fe je 0x2e51b
            if fl.e() { pc = 0x2e51b; continue; }
            pc = 0x2e500;
        }
        0x2e500 => {
            // block 2e500
            #[cfg(feature = "trace")] eprintln!("f_0002e45e pc 2e500 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e500 lea r14,[rbp-0x38]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0x2e504;
        }
        0x2e504 => {
            // block 2e504
            #[cfg(feature = "trace")] eprintln!("f_0002e45e pc 2e504 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e504 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 2e507 mov rdi,r14
            rdi = r14;
            // 2e50a call 0x2eea0
            { let r = { let t = m.env.code_va(0x2eea0); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e50f add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2e513 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 2e516 jne 0x2e504
            if fl.ne() { pc = 0x2e504; continue; }
            pc = 0x2e518;
        }
        0x2e518 => {
            // block 2e518
            #[cfg(feature = "trace")] eprintln!("f_0002e45e pc 2e518 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e518 mov eax,DWORD PTR [rbp-0x3c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64) as u32 as u64;
            pc = 0x2e51b;
        }
        0x2e51b => {
            // block 2e51b
            #[cfg(feature = "trace")] eprintln!("f_0002e45e pc 2e51b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e51b add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 2e51f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e520 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e522 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e524 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e526 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e527 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4247e size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004247e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4247e;
    loop {
        match pc {
        0x4247e => {
            // block 4247e
            #[cfg(feature = "trace")] eprintln!("f_0004247e pc 4247e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4247e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4247f mov rbp,rsp
            rbp = rsp;
            // 42482 mov r9,rsi
            r9 = rsi;
            // 42485 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 42488 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 4248c mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 4248e lea rdx,[rip+0xbdf8b] # 0x100420
            rdx = m.env.data_va(84973);
            // 42495 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42498 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4249d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 424a0 mov rdx,r9
            rdx = r9;
            // 424a3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 424a4 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x100e size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000100e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x100e;
    loop {
        match pc {
        0x100e => {
            // block 100e
            #[cfg(feature = "trace")] eprintln!("f_0000100e pc 100e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 100e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 100f mov rbp,rsp
            rbp = rsp;
            // 1012 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1014 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1016 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1017 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1018 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 101b mov r14,rsi
            r14 = rsi;
            // 101e mov rbx,rdi
            rbx = rdi;
            // 1021 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1024 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 102a mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 102d mov r9,QWORD PTR [rcx+0x38]
            r9 = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 1031 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 1034 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1036 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1038 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // 103c lea rax,[rip+0xfdfad] # 0xfeff0
            rax = m.env.data_va(79805);
            // 1043 mov esi,DWORD PTR [rax+rdx*4]
            rsi = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 1046 mov ecx,0x380
            rcx = (0x380u64) as u32 as u64;
            // 104b mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // 1051 mov rdi,rbx
            rdi = rbx;
            // 1054 mov rdx,r14
            rdx = r14;
            // 1057 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 105b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 105c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 105e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1060 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1061 jmp r9
            { let r = { let t = r9; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3ef4 size=188
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003ef4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3ef4;
    loop {
        match pc {
        0x3ef4 => {
            // block 3ef4
            #[cfg(feature = "trace")] eprintln!("f_00003ef4 pc 3ef4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3ef4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3ef5 mov rbp,rsp
            rbp = rsp;
            // 3ef8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 3efa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3efc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3efd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3efe mov r14,rcx
            r14 = rcx;
            // 3f01 mov r15,rdx
            r15 = rdx;
            // 3f04 mov rbx,rsi
            rbx = rsi;
            // 3f07 mov WORD PTR [r14],0x0
            m.w16(r14, (0x0u64) as u16);
            // 3f0d mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 3f13 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 3f18 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 3f1f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 3f22 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 3f26 mov esi,0x59
            rsi = (0x59u64) as u32 as u64;
            // 3f2b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3f30 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3f33 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3f36 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3f38 je 0x3fa3
            if fl.e() { pc = 0x3fa3; continue; }
            pc = 0x3f3a;
        }
        0x3f3a => {
            // block 3f3a
            #[cfg(feature = "trace")] eprintln!("f_00003ef4 pc 3f3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3f3a mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 3f3d mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 3f3f shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 3f42 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 3f44 shr edx,0x8
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x8u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 3f47 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 3f49 shr esi,0x14
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x14u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 3f4c and esi,0xf
            { let r = (rsi as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 3f4f lea esi,[rsi+rsi*4]
            rsi = (rsi.wrapping_add(rsi.wrapping_mul(4))) as u32 as u64;
            // 3f52 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3f55 lea ecx,[rcx+rsi*2]
            rcx = (rcx.wrapping_add(rsi.wrapping_mul(2))) as u32 as u64;
            // 3f58 mov WORD PTR [rbx],cx
            m.w16(rbx, ((rcx as u16 as u64)) as u16);
            // 3f5b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 3f5d shr ecx,0xc
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0xcu64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 3f60 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3f63 lea ecx,[rcx+rcx*4]
            rcx = (rcx.wrapping_add(rcx.wrapping_mul(4))) as u32 as u64;
            // 3f66 and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3f69 lea ecx,[rdx+rcx*2]
            rcx = (rdx.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // 3f6c mov WORD PTR [r15],cx
            m.w16(r15, ((rcx as u16 as u64)) as u16);
            // 3f70 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 3f72 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 3f75 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3f78 lea ecx,[rcx+rcx*4]
            rcx = (rcx.wrapping_add(rcx.wrapping_mul(4))) as u32 as u64;
            // 3f7b and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3f7e lea eax,[rax+rcx*2]
            rax = (rax.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // 3f81 mov WORD PTR [r14],ax
            m.w16(r14, ((rax as u16 as u64)) as u16);
            // 3f85 movzx ecx,WORD PTR [rbx]
            rcx = (m.r16(rbx) as u64) as u32 as u64;
            // 3f88 cmp ecx,0x18
            fl = sub::<32>((rcx as u32 as u64), 0x18u64).1;
            // 3f8b sbb cl,cl
            { let (r, f) = sbb::<8>((rcx as u8 as u64), (rcx as u8 as u64), fl.cf);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f8d movzx edx,WORD PTR [r15]
            rdx = (m.r16(r15) as u64) as u32 as u64;
            // 3f91 cmp edx,0x3c
            fl = sub::<32>((rdx as u32 as u64), 0x3cu64).1;
            // 3f94 sbb dl,dl
            { let (r, f) = sbb::<8>((rdx as u8 as u64), (rdx as u8 as u64), fl.cf);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f96 and dl,cl
            { let r = (rdx as u8 as u64) & (rcx as u8 as u64); let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f98 cmp eax,0x3c
            fl = sub::<32>((rax as u32 as u64), 0x3cu64).1;
            // 3f9b sbb al,al
            { let (r, f) = sbb::<8>((rax as u8 as u64), (rax as u8 as u64), fl.cf);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f9d and al,dl
            { let r = (rax as u8 as u64) & (rdx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f9f and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3fa1 jmp 0x3fa5
            pc = 0x3fa5; continue;
        }
        0x3fa3 => {
            // block 3fa3
            #[cfg(feature = "trace")] eprintln!("f_00003ef4 pc 3fa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3fa3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x3fa5;
        }
        0x3fa5 => {
            // block 3fa5
            #[cfg(feature = "trace")] eprintln!("f_00003ef4 pc 3fa5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3fa5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 3fa9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3faa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3fac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3fae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3faf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x975a size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000975a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x975a;
    loop {
        match pc {
        0x975a => {
            // block 975a
            #[cfg(feature = "trace")] eprintln!("f_0000975a pc 975a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 975a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 975b mov rbp,rsp
            rbp = rsp;
            // 975e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9760 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9762 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9763 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 9764 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 9767 mov r14,rsi
            r14 = rsi;
            // 976a mov rbx,rdi
            rbx = rdi;
            // 976d mov DWORD PTR [r14],0x2
            m.w32(r14, (0x2u64) as u32);
            // 9774 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 9777 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 977a call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9780 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9782 jne 0x97be
            if fl.ne() { pc = 0x97be; continue; }
            pc = 0x9784;
        }
        0x9784 => {
            // block 9784
            #[cfg(feature = "trace")] eprintln!("f_0000975a pc 9784 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9784 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 978b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 978e mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 9791 lea rdx,[rip+0xf5f38] # 0xff6d0
            rdx = m.env.data_va(81565);
            // 9798 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 979b lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 979f mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 97a4 mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 97aa mov rdi,rbx
            rdi = rbx;
            // 97ad call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 97b0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 97b2 je 0x97be
            if fl.e() { pc = 0x97be; continue; }
            pc = 0x97b4;
        }
        0x97b4 => {
            // block 97b4
            #[cfg(feature = "trace")] eprintln!("f_0000975a pc 97b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97b4 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 97b7 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 97ba mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 97bc jmp 0x97c0
            pc = 0x97c0; continue;
        }
        0x97be => {
            // block 97be
            #[cfg(feature = "trace")] eprintln!("f_0000975a pc 97be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97be xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x97c0;
        }
        0x97c0 => {
            // block 97c0
            #[cfg(feature = "trace")] eprintln!("f_0000975a pc 97c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97c0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 97c4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 97c5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 97c7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 97c9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 97ca ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51d9a size=26
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051d9a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51d9a;
    loop {
        match pc {
        0x51d9a => {
            // block 51d9a
            #[cfg(feature = "trace")] eprintln!("f_00051d9a pc 51d9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51d9a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51d9b mov rbp,rsp
            rbp = rsp;
            // 51d9e test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 51da1 je 0x51daf
            if fl.e() { pc = 0x51daf; continue; }
            pc = 0x51da3;
        }
        0x51da3 => {
            // block 51da3
            #[cfg(feature = "trace")] eprintln!("f_00051d9a pc 51da3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51da3 mov eax,DWORD PTR [rip+0x1764df] # 0x1c8288
            rax = (m.r32(m.env.data_va(903765)) as u64) as u32 as u64;
            // 51da9 mov DWORD PTR [rsi],eax
            m.w32(rsi, ((rax as u32 as u64)) as u32);
            // 51dab mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 51dad jmp 0x51db1
            pc = 0x51db1; continue;
        }
        0x51daf => {
            // block 51daf
            #[cfg(feature = "trace")] eprintln!("f_00051d9a pc 51daf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51daf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x51db1;
        }
        0x51db1 => {
            // block 51db1
            #[cfg(feature = "trace")] eprintln!("f_00051d9a pc 51db1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51db1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51db2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1613e size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001613e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1613e;
    loop {
        match pc {
        0x1613e => {
            // block 1613e
            #[cfg(feature = "trace")] eprintln!("f_0001613e pc 1613e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1613e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1613f mov rbp,rsp
            rbp = rsp;
            // 16142 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 16143 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 16144 mov rbx,rdi
            rbx = rdi;
            // 16147 cmp rbx,rsi
            fl = sub::<64>(rbx, rsi).1;
            // 1614a je 0x16155
            if fl.e() { pc = 0x16155; continue; }
            pc = 0x1614c;
        }
        0x1614c => {
            // block 1614c
            #[cfg(feature = "trace")] eprintln!("f_0001613e pc 1614c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1614c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1614f mov rdi,rbx
            rdi = rbx;
            // 16152 call QWORD PTR [rax+0x68]
            { let r = { let t = m.r64(rax.wrapping_add(0x68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16155;
        }
        0x16155 => {
            // block 16155
            #[cfg(feature = "trace")] eprintln!("f_0001613e pc 16155 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16155 mov rax,rbx
            rax = rbx;
            // 16158 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1615c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1615d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1615e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
