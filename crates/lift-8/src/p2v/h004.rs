//! Mechanically lifted functions (scripts/lift_run.py), module h004. Each one is checked against the original
//! in the world harness (tests/world_h004.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

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
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x12edd; continue; }
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
            { let r = { let t = m.env.stub_va(0x2d7); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0x12ebc; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x90)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f0d test BYTE PTR [r13+0x2c8],0x1
            fl = logic::<8>(m.r8(r13.wrapping_add(0x2c8)) as u64 & 0x1u64);
            // 12f15 jne 0x12f1f
            if fl.ne() { pc = 0x12f1f; continue; }
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
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f6d movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 12f71 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 12f73 jne 0x12f7a
            if fl.ne() { pc = 0x12f7a; continue; }
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
            if fl.ne() { pc = 0x12f8e; continue; }
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
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0xdfcd0; continue; }
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
            if fl.a() { pc = 0xdfd04; continue; }
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
            if fl.e() { pc = 0xdfd48; continue; }
            pc = 0xdfd38;
        }
        0xdfd38 => {
            // block dfd38
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfd38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfd38 lea rdi,[r15*8+0x0]
            rdi = r15.wrapping_mul(8);
            // dfd40 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x6a9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x14f08 size=288
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014f08<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x14f08;
    loop {
        match pc {
        0x14f08 => {
            // block 14f08
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f08 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14f09 mov rbp,rsp
            rbp = rsp;
            // 14f0c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 14f0e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14f0f mov r14,rsi
            r14 = rsi;
            // 14f12 mov rbx,rdi
            rbx = rdi;
            // 14f15 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 14f18 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14f1b mov rsi,QWORD PTR [rip+0x12b276] # 0x140198
            rsi = m.r64(m.env.data_va(346469)) as u64;
            // 14f22 mov rdi,r14
            rdi = r14;
            // 14f25 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14f2a mov QWORD PTR [rbx+0x80],rax
            m.w64(rbx.wrapping_add(0x80), (rax) as u64);
            // 14f31 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 14f34 movzx r14d,BYTE PTR [rbx+0x192]
            r14 = (m.r8(rbx.wrapping_add(0x192)) as u64) as u32 as u64;
            // 14f3c mov rdi,rax
            rdi = rax;
            // 14f3f call QWORD PTR [rcx+0x38]
            { let r = { let t = m.r64(rcx.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14f42 mov BYTE PTR [rbx+0x192],al
            m.w8(rbx.wrapping_add(0x192), ((rax as u8 as u64)) as u8);
            // 14f48 movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            // 14f4b cmp r14d,ecx
            fl = sub::<32>((r14 as u32 as u64), (rcx as u32 as u64)).1;
            // 14f4e je 0x15023
            if fl.e() { pc = 0x15023; continue; }
            pc = 0x14f54;
        }
        0x14f54 => {
            // block 14f54
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f54 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 14f5c mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 14f64 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 14f6c mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 14f74 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 14f7c mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 14f84 mov cl,BYTE PTR [rbx+0x190]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(0x190)) as u64) as u8 as u64);
            // 14f8a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 14f8c je 0x14fd5
            if fl.e() { pc = 0x14fd5; continue; }
            pc = 0x14f8e;
        }
        0x14f8e => {
            // block 14f8e
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f8e test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 14f90 je 0x14fa0
            if fl.e() { pc = 0x14fa0; continue; }
            pc = 0x14f92;
        }
        0x14f92 => {
            // block 14f92
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f92 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 14f96 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 14f99 je 0x14fa0
            if fl.e() { pc = 0x14fa0; continue; }
            pc = 0x14f9b;
        }
        0x14f9b => {
            // block 14f9b
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f9b call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x14fa0;
        }
        0x14fa0 => {
            // block 14fa0
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14fa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14fa0 mov al,BYTE PTR [rbx+0x191]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x191)) as u64) as u8 as u64);
            // 14fa6 mov BYTE PTR [rbx+0x190],al
            m.w8(rbx.wrapping_add(0x190), ((rax as u8 as u64)) as u8);
            // 14fac mov rax,QWORD PTR [rbx+0x70]
            rax = m.r64(rbx.wrapping_add(0x70)) as u64;
            // 14fb0 mov QWORD PTR [rbx+0x60],rax
            m.w64(rbx.wrapping_add(0x60), (rax) as u64);
            // 14fb4 mov rax,QWORD PTR [rbx+0x68]
            rax = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 14fb8 mov QWORD PTR [rbx+0x40],rax
            m.w64(rbx.wrapping_add(0x40), (rax) as u64);
            // 14fbc mov BYTE PTR [rbx+0x191],0x0
            m.w8(rbx.wrapping_add(0x191), (0x0u64) as u8);
            // 14fc3 mov QWORD PTR [rbx+0x70],0x0
            m.w64(rbx.wrapping_add(0x70), (0x0u64) as u64);
            // 14fcb mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 14fd3 jmp 0x15023
            pc = 0x15023; continue;
        }
        0x14fd5 => {
            // block 14fd5
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14fd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14fd5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 14fd7 jne 0x1500b
            if fl.ne() { pc = 0x1500b; continue; }
            pc = 0x14fd9;
        }
        0x14fd9 => {
            // block 14fd9
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14fd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14fd9 mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 14fdd lea rcx,[rbx+0x58]
            rcx = rbx.wrapping_add(0x58);
            // 14fe1 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 14fe4 je 0x1500b
            if fl.e() { pc = 0x1500b; continue; }
            pc = 0x14fe6;
        }
        0x14fe6 => {
            // block 14fe6
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14fe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14fe6 mov rdi,QWORD PTR [rbx+0x60]
            rdi = m.r64(rbx.wrapping_add(0x60)) as u64;
            // 14fea mov QWORD PTR [rbx+0x70],rdi
            m.w64(rbx.wrapping_add(0x70), (rdi) as u64);
            // 14fee mov QWORD PTR [rbx+0x68],rax
            m.w64(rbx.wrapping_add(0x68), (rax) as u64);
            // 14ff2 mov BYTE PTR [rbx+0x191],0x0
            m.w8(rbx.wrapping_add(0x191), (0x0u64) as u8);
            // 14ff9 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14ffe mov QWORD PTR [rbx+0x40],rax
            m.w64(rbx.wrapping_add(0x40), (rax) as u64);
            // 15002 mov BYTE PTR [rbx+0x190],0x1
            m.w8(rbx.wrapping_add(0x190), (0x1u64) as u8);
            // 15009 jmp 0x15023
            pc = 0x15023; continue;
        }
        0x1500b => {
            // block 1500b
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 1500b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1500b mov rdi,QWORD PTR [rbx+0x60]
            rdi = m.r64(rbx.wrapping_add(0x60)) as u64;
            // 1500f mov QWORD PTR [rbx+0x70],rdi
            m.w64(rbx.wrapping_add(0x70), (rdi) as u64);
            // 15013 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15018 mov QWORD PTR [rbx+0x68],rax
            m.w64(rbx.wrapping_add(0x68), (rax) as u64);
            // 1501c mov BYTE PTR [rbx+0x191],0x1
            m.w8(rbx.wrapping_add(0x191), (0x1u64) as u8);
            pc = 0x15023;
        }
        0x15023 => {
            // block 15023
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 15023 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15023 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15024 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15026 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15027 ret
            return Ret { rax, rdx, xmm0: x0 };
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
            if fl.e() { pc = 0xb921c; continue; }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0xb9206; continue; }
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
            if fl.ne() { pc = 0xb91f8; continue; }
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
            if fl.ne() { pc = 0xb9206; continue; }
            pc = 0xb9213;
        }
        0xb9213 => {
            // block b9213
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9213 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9213 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // b9216 jne 0xb9140
            if fl.ne() { pc = 0xb9140; continue; }
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

/// parity: P2v @0xe196e size=280
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e196e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe196e;
    loop {
        match pc {
        0xe196e => {
            // block e196e
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e196e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e196e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e196f mov rbp,rsp
            rbp = rsp;
            // e1972 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e1974 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1976 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1977 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e197b mov r14,rcx
            r14 = rcx;
            // e197e mov r15,rsi
            r15 = rsi;
            // e1981 lea rax,[rip+0x492d3] # 0x12ac5b
            rax = m.env.data_va(259112);
            // e1988 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e198c mov rsi,rdx
            rsi = rdx;
            // e198f mov rdx,rax
            rdx = rax;
            // e1992 call 0xebf06
            { let r = { let t = m.env.stub_va(0x6d3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1997 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // e199b test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e199d jne 0xe19a8
            if fl.ne() { pc = 0xe19a8; continue; }
            pc = 0xe199f;
        }
        0xe199f => {
            // block e199f
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e199f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e199f lea r8,[r15+0x1]
            r8 = r15.wrapping_add(0x1);
            // e19a3 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // e19a6 jmp 0xe19b0
            pc = 0xe19b0; continue;
        }
        0xe19a8 => {
            // block e19a8
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19a8 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // e19ac mov r8,QWORD PTR [r15+0x10]
            r8 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0xe19b0;
        }
        0xe19b0 => {
            // block e19b0
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19b0 movzx edx,BYTE PTR [rbp-0x30]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e19b4 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // e19b7 jne 0xe19c2
            if fl.ne() { pc = 0xe19c2; continue; }
            pc = 0xe19b9;
        }
        0xe19b9 => {
            // block e19b9
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19b9 lea r11,[rbp-0x2f]
            r11 = rbp.wrapping_add(0xffff_ffff_ffff_ffd1);
            // e19bd shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e19c0 jmp 0xe19ca
            pc = 0xe19ca; continue;
        }
        0xe19c2 => {
            // block e19c2
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19c2 mov rdx,QWORD PTR [rbp-0x28]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // e19c6 mov r11,QWORD PTR [rbp-0x20]
            r11 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            pc = 0xe19ca;
        }
        0xe19ca => {
            // block e19ca
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19ca cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // e19cd jb 0xe1a22
            if fl.b() { pc = 0xe1a22; continue; }
            pc = 0xe19cf;
        }
        0xe19cf => {
            // block e19cf
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19cf test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e19d2 je 0xe1a32
            if fl.e() { pc = 0xe1a32; continue; }
            pc = 0xe19d4;
        }
        0xe19d4 => {
            // block e19d4
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19d4 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // e19d7 jl 0xe1a22
            if fl.l() { pc = 0xe1a22; continue; }
            pc = 0xe19d9;
        }
        0xe19d9 => {
            // block e19d9
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19d9 mov r10d,0x1
            r10 = (0x1u64) as u32 as u64;
            // e19df sub r10,rdx
            { let (r, f) = sub::<64>(r10, rdx);
              r10 = r;
              fl = f; }
            // e19e2 add r10,rax
            { let (r, f) = add::<64>(r10, rax);
              r10 = r;
              fl = f; }
            // e19e5 je 0xe1a22
            if fl.e() { pc = 0xe1a22; continue; }
            pc = 0xe19e7;
        }
        0xe19e7 => {
            // block e19e7
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19e7 add rax,r8
            { let (r, f) = add::<64>(rax, r8);
              rax = r;
              fl = f; }
            // e19ea add r10,r8
            { let (r, f) = add::<64>(r10, r8);
              r10 = r;
              fl = f; }
            // e19ed movzx r9d,BYTE PTR [r11]
            r9 = (m.r8(r11) as u64) as u32 as u64;
            // e19f1 mov rcx,r8
            rcx = r8;
            pc = 0xe19f4;
        }
        0xe19f4 => {
            // block e19f4
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19f4 movzx ebx,BYTE PTR [rcx]
            rbx = (m.r8(rcx) as u64) as u32 as u64;
            // e19f7 cmp ebx,r9d
            fl = sub::<32>((rbx as u32 as u64), (r9 as u32 as u64)).1;
            // e19fa mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // e19ff jne 0xe1a16
            if fl.ne() { pc = 0xe1a16; continue; }
            pc = 0xe1a01;
        }
        0xe1a01 => {
            // block e1a01
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a01 cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // e1a04 je 0xe1a26
            if fl.e() { pc = 0xe1a26; continue; }
            pc = 0xe1a06;
        }
        0xe1a06 => {
            // block e1a06
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a06 movzx edi,BYTE PTR [r11+rbx*1]
            rdi = (m.r8(r11.wrapping_add(rbx)) as u64) as u32 as u64;
            // e1a0b movzx esi,BYTE PTR [rcx+rbx*1]
            rsi = (m.r8(rcx.wrapping_add(rbx)) as u64) as u32 as u64;
            // e1a0f inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e1a12 cmp esi,edi
            fl = sub::<32>((rsi as u32 as u64), (rdi as u32 as u64)).1;
            // e1a14 je 0xe1a01
            if fl.e() { pc = 0xe1a01; continue; }
            pc = 0xe1a16;
        }
        0xe1a16 => {
            // block e1a16
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a16 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // e1a19 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1a1b cmp rcx,r10
            fl = sub::<64>(rcx, r10).1;
            // e1a1e jne 0xe19f4
            if fl.ne() { pc = 0xe19f4; continue; }
            pc = 0xe1a20;
        }
        0xe1a20 => {
            // block e1a20
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a20 jmp 0xe1a5b
            pc = 0xe1a5b; continue;
        }
        0xe1a22 => {
            // block e1a22
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a22 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1a24 jmp 0xe1a5b
            pc = 0xe1a5b; continue;
        }
        0xe1a26 => {
            // block e1a26
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a26 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1a28 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // e1a2b je 0xe1a5b
            if fl.e() { pc = 0xe1a5b; continue; }
            pc = 0xe1a2d;
        }
        0xe1a2d => {
            // block e1a2d
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a2d cmp rcx,r8
            fl = sub::<64>(rcx, r8).1;
            // e1a30 jne 0xe1a5b
            if fl.ne() { pc = 0xe1a5b; continue; }
            pc = 0xe1a32;
        }
        0xe1a32 => {
            // block e1a32
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a32 mov rdi,r14
            rdi = r14;
            // e1a35 mov rsi,r15
            rsi = r15;
            // e1a38 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1a3d movzx edx,BYTE PTR [rbp-0x30]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e1a41 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // e1a44 jne 0xe1a4b
            if fl.ne() { pc = 0xe1a4b; continue; }
            pc = 0xe1a46;
        }
        0xe1a46 => {
            // block e1a46
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a46 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e1a49 jmp 0xe1a4f
            pc = 0xe1a4f; continue;
        }
        0xe1a4b => {
            // block e1a4b
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a4b mov rdx,QWORD PTR [rbp-0x28]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            pc = 0xe1a4f;
        }
        0xe1a4f => {
            // block e1a4f
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a4f mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // e1a51 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e1a53 mov rdi,r14
            rdi = r14;
            // e1a56 call 0xebfde
            { let r = { let t = m.env.stub_va(0x7ab); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1a5b;
        }
        0xe1a5b => {
            // block e1a5b
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a5b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e1a5f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1a64 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e1a66 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e1a6a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1a6b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e1a6d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e1a6f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1a70 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe1a71 => {
            // block e1a71
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a71 mov rbx,rax
            rbx = rax;
            // e1a74 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e1a78 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1a7d mov rdi,rbx
            rdi = rbx;
            // e1a80 call 0xec182
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
            { let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28819 add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 2881d cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28820 je 0x288b8
            if fl.e() { pc = 0x288b8; continue; }
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
            { let r = { let t = m.env.code_va(0x61aed); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2883e movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 28842 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 28845 jne 0x28850
            if fl.ne() { pc = 0x28850; continue; }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28860 mov rdi,r15
            rdi = r15;
            // 28863 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28868 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2886c test rax,rax
            fl = logic::<64>(rax & rax);
            // 2886f je 0x2887e
            if fl.e() { pc = 0x2887e; continue; }
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
            if fl.ne() { pc = 0x28871; continue; }
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
            if fl.ne() { pc = 0x28881; continue; }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 288af cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 288b2 jne 0x28831
            if fl.ne() { pc = 0x28831; continue; }
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

/// parity: P2v @0x123fc size=273
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000123fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x123fc;
    loop {
        match pc {
        0x123fc => {
            // block 123fc
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 123fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 123fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 123fd mov rbp,rsp
            rbp = rsp;
            // 12400 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12402 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12404 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12406 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12408 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12409 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 1240d mov rbx,rdx
            rbx = rdx;
            // 12410 mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // 12414 mov r15,rdi
            r15 = rdi;
            // 12417 cmp r15,rsi
            fl = sub::<64>(r15, rsi).1;
            // 1241a je 0x124d1
            if fl.e() { pc = 0x124d1; continue; }
            pc = 0x12420;
        }
        0x12420 => {
            // block 12420
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12420 mov rdi,rbx
            rdi = rbx;
            // 12423 mov rsi,r15
            rsi = r15;
            // 12426 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1242b lea rcx,[r15+0x38]
            rcx = r15.wrapping_add(0x38);
            // 1242f mov r13d,0x1
            r13 = (0x1u64) as u32 as u64;
            // 12435 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 12438 cmp rcx,QWORD PTR [rbp-0x40]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64).1;
            // 1243c je 0x124d1
            if fl.e() { pc = 0x124d1; continue; }
            pc = 0x12442;
        }
        0x12442 => {
            // block 12442
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12442 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12442 mov r14,rbx
            r14 = rbx;
            pc = 0x12445;
        }
        0x12445 => {
            // block 12445
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12445 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12445 lea rdi,[r14+0x38]
            rdi = r14.wrapping_add(0x38);
            // 12449 mov eax,DWORD PTR [r15+0x50]
            rax = (m.r32(r15.wrapping_add(0x50)) as u64) as u32 as u64;
            // 1244d cmp eax,DWORD PTR [r14+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x18)) as u64).1;
            // 12451 jge 0x1249e
            if fl.ge() { pc = 0x1249e; continue; }
            pc = 0x12453;
        }
        0x12453 => {
            // block 12453
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12453 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 12457 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 1245b mov rsi,r14
            rsi = r14;
            // 1245e call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12463 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // 12466 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 12469 mov r14,r12
            r14 = r12;
            // 1246c mov rdi,rbx
            rdi = rbx;
            // 1246f je 0x12493
            if fl.e() { pc = 0x12493; continue; }
            pc = 0x12471;
        }
        0x12471 => {
            // block 12471
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12471 lea rdi,[rbx+r14*1]
            rdi = rbx.wrapping_add(r14);
            // 12475 mov eax,DWORD PTR [r15+0x50]
            rax = (m.r32(r15.wrapping_add(0x50)) as u64) as u32 as u64;
            // 12479 cmp eax,DWORD PTR [rbx+r14*1-0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbx.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 1247e jge 0x12493
            if fl.ge() { pc = 0x12493; continue; }
            pc = 0x12480;
        }
        0x12480 => {
            // block 12480
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12480 lea rsi,[rbx+r14*1-0x38]
            rsi = rbx.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_ffc8);
            // 12485 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1248a add r14,0xffffffffffffffc8
            { let (r, f) = add::<64>(r14, 0xffff_ffff_ffff_ffc8u64);
              r14 = r;
              fl = f; }
            // 1248e jne 0x12471
            if fl.ne() { pc = 0x12471; continue; }
            pc = 0x12490;
        }
        0x12490 => {
            // block 12490
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12490 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12490 mov rdi,rbx
            rdi = rbx;
            pc = 0x12493;
        }
        0x12493 => {
            // block 12493
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12493 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12493 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 12497 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1249c jmp 0x124b1
            pc = 0x124b1; continue;
        }
        0x1249e => {
            // block 1249e
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 1249e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1249e mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 124a2 mov rsi,rcx
            rsi = rcx;
            // 124a5 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // 124a9 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 124ae inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            pc = 0x124b1;
        }
        0x124b1 => {
            // block 124b1
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124b1 mov rax,r15
            rax = r15;
            // 124b4 add rax,0x70
            { let (r, f) = add::<64>(rax, 0x70u64);
              rax = r;
              fl = f; }
            // 124b8 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 124bc cmp rax,QWORD PTR [rbp-0x40]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64).1;
            // 124c0 mov r15,QWORD PTR [rbp-0x30]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 124c4 mov rcx,rax
            rcx = rax;
            // 124c7 mov r14,QWORD PTR [rbp-0x38]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 124cb jne 0x12445
            if fl.ne() { pc = 0x12445; continue; }
            pc = 0x124d1;
        }
        0x124d1 => {
            // block 124d1
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124d1 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 124d5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 124d6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 124d8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 124da pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 124dc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 124de pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 124df ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x124e0 => {
            // block 124e0
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124e0 jmp 0x124e2
            pc = 0x124e2; continue;
        }
        0x124e2 => {
            // block 124e2
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124e2 mov r14,rax
            r14 = rax;
            // 124e5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 124e8 je 0x12505
            if fl.e() { pc = 0x12505; continue; }
            pc = 0x124ea;
        }
        0x124ea => {
            // block 124ea
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124ea test r13,r13
            fl = logic::<64>(r13 & r13);
            // 124ed je 0x12505
            if fl.e() { pc = 0x12505; continue; }
            pc = 0x124ef;
        }
        0x124ef => {
            // block 124ef
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124ef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 124f2 mov rdi,rbx
            rdi = rbx;
            // 124f5 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 124f7 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 124fb dec r13
            { let (r, f) = dec::<64>(r13, fl); r13 = r; fl = f; }
            // 124fe jne 0x124ef
            if fl.ne() { pc = 0x124ef; continue; }
            pc = 0x12500;
        }
        0x12500 => {
            // block 12500
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12500 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12500 jmp 0x12505
            pc = 0x12505; continue;
        }
        0x12502 => {
            // block 12502
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12502 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12502 mov r14,rax
            r14 = rax;
            pc = 0x12505;
        }
        0x12505 => {
            // block 12505
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12505 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12505 mov rdi,r14
            rdi = r14;
            // 12508 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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

/// parity: P2v @0xdff97 size=266
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dff97<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdff97;
    loop {
        match pc {
        0xdff97 => {
            // block dff97
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc dff97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dff97 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dff98 mov rbp,rsp
            rbp = rsp;
            // dff9b push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dff9d push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dff9f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dffa0 mov ax,WORD PTR [rdi]
            rax = (rax & !0xffff) | ((m.r16(rdi) as u64) as u16 as u64);
            // dffa3 mov WORD PTR [rdi-0x2],ax
            m.w16(rdi.wrapping_add(0xffff_ffff_ffff_fffe), ((rax as u16 as u64)) as u16);
            // dffa7 lea eax,[r8-0x1]
            rax = (r8.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // dffab cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dffad mov r10w,WORD PTR [rdi+rax*2]
            r10 = (r10 & !0xffff) | ((m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // dffb2 lea eax,[r8+0x1]
            rax = (r8.wrapping_add(0x1)) as u32 as u64;
            // dffb6 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dffb8 mov WORD PTR [rdi+rax*2],r10w
            m.w16(rdi.wrapping_add(rax.wrapping_mul(2)), ((r10 as u16 as u64)) as u16);
            // dffbd movsxd rbx,r8d
            rbx = sx::<32>((r8 as u32 as u64)) as u64;
            // dffc0 mov WORD PTR [rdi+rbx*2],r10w
            m.w16(rdi.wrapping_add(rbx.wrapping_mul(2)), ((r10 as u16 as u64)) as u16);
            // dffc5 imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dffc9 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // dffcb cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dffcc idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dffce mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // dffd1 imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // dffd5 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // dffd7 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dffd8 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dffda mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // dffdd cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // dffe0 jge 0xe009a
            if fl.ge() { pc = 0xe009a; continue; }
            pc = 0xdffe6;
        }
        0xdffe6 => {
            // block dffe6
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc dffe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dffe6 shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // dffea mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // dffed cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dffee idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // dfff1 sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // dfff4 movsxd rcx,r11d
            rcx = sx::<32>((r11 as u32 as u64)) as u64;
            // dfff7 lea r14,[rsi+rcx*2]
            r14 = rsi.wrapping_add(rcx.wrapping_mul(2));
            // dfffb imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            // dffff lea r9,[rip+0x4801a] # 0x128020
            r9 = m.env.data_va(247789);
            // e0006 mov r8w,0x4
            r8 = (r8 & !0xffff) | ((0x4u64) as u16 as u64);
            pc = 0xe000b;
        }
        0xe000b => {
            // block e000b
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc e000b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e000b mov edx,r11d
            rdx = ((r11 as u32 as u64)) as u32 as u64;
            // e000e sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e0011 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // e0014 shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0017 and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e001a movsxd r15,edx
            r15 = sx::<32>((rdx as u32 as u64)) as u64;
            // e001d movsx esi,WORD PTR [rdi+r15*2-0x2]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e0023 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // e0028 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e002a imul esi,DWORD PTR [r9+rcx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rcx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // e002f movsx ecx,WORD PTR [rdi+r15*2]
            rcx = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e0034 mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // e0039 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e003b imul ecx,DWORD PTR [r9+rdx*4]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(r9.wrapping_add(rdx.wrapping_mul(4))) as u64); rcx = (r) as u32 as u64; fl = f; }
            // e0040 add ecx,esi
            { let (r, f) = add::<32>((rcx as u32 as u64), (rsi as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0042 movsx edx,WORD PTR [rdi+r15*2+0x2]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e0048 mov esi,0x60
            rsi = (0x60u64) as u32 as u64;
            // e004d sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e004f imul edx,DWORD PTR [r9+rsi*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r9.wrapping_add(rsi.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // e0054 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0056 movsx esi,WORD PTR [rdi+r15*2+0x4]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e005c mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // e0061 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0063 imul esi,DWORD PTR [r9+rcx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rcx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // e0068 add esi,edx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rdx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e006a mov dx,0x3fb
            rdx = (rdx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e006e cmp esi,0x3fbffff
            fl = sub::<32>((rsi as u32 as u64), 0x3fbffffu64).1;
            // e0074 ja 0xe0086
            if fl.a() { pc = 0xe0086; continue; }
            pc = 0xe0076;
        }
        0xe0076 => {
            // block e0076
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc e0076 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0076 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // e0078 shr edx,0x10
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e007b cmp esi,0x40000
            fl = sub::<32>((rsi as u32 as u64), 0x40000u64).1;
            // e0081 cmovb dx,r8w
            if fl.b() { rdx = (rdx & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            pc = 0xe0086;
        }
        0xe0086 => {
            // block e0086
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc e0086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0086 mov WORD PTR [r14],dx
            m.w16(r14, ((rdx as u16 as u64)) as u16);
            // e008a add r14,0x2
            { let (r, f) = add::<64>(r14, 0x2u64);
              r14 = r;
              fl = f; }
            // e008e add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // e0091 dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // e0094 jne 0xe000b
            if fl.ne() { pc = 0xe000b; continue; }
            pc = 0xe009a;
        }
        0xe009a => {
            // block e009a
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc e009a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e009a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e009b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e009d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e009f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e00a0 ret
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9072 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9076 mov rsi,rbx
            rsi = rbx;
            // 9079 call 0x68488
            { let r = { let t = m.env.code_va(0x68488); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 907e lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9082 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9087 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // 908b call 0xceac8
            { let r = { let t = m.env.code_va(0xceac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x90f7; continue; }
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
            if fl.a() { pc = 0x90cd; continue; }
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
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x90e3; continue; }
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
            if fl.ne() { pc = 0x90d6; continue; }
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
            if fl.ne() { pc = 0x90e6; continue; }
            pc = 0x90f2;
        }
        0x90f2 => {
            // block 90f2
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90f2 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 90f5 jne 0x90a6
            if fl.ne() { pc = 0x90a6; continue; }
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
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x441ca size=252
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000441ca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x441ca;
    loop {
        match pc {
        0x441ca => {
            // block 441ca
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 441ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 441ca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 441cb mov rbp,rsp
            rbp = rsp;
            // 441ce push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 441d0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 441d2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 441d4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 441d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 441d7 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 441db mov r15,rsi
            r15 = rsi;
            // 441de mov r14,rdi
            r14 = rdi;
            // 441e1 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 441e8 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 441ef lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // 441f3 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 441f7 mov rdi,r15
            rdi = r15;
            // 441fa call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 441ff mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 44207 mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // 4420a mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 44212 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // 44216 call 0xcb77f
            { let r = { let t = m.env.code_va(0xcb77f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4421b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4421d je 0x442b1
            if fl.e() { pc = 0x442b1; continue; }
            pc = 0x44223;
        }
        0x44223 => {
            // block 44223
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 44223 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44223 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 44226 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 4422a xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 4422d mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // 44232 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44237 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4423a mov rdi,r14
            rdi = r14;
            // 4423d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44240 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44242 je 0x442b4
            if fl.e() { pc = 0x442b4; continue; }
            pc = 0x44244;
        }
        0x44244 => {
            // block 44244
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 44244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44244 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 44247 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 4424b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 4424e mov esi,0x19
            rsi = (0x19u64) as u32 as u64;
            // 44253 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44258 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4425b mov rdi,r14
            rdi = r14;
            // 4425e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44261 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44263 je 0x442b4
            if fl.e() { pc = 0x442b4; continue; }
            pc = 0x44265;
        }
        0x44265 => {
            // block 44265
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 44265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44265 movzx eax,BYTE PTR [rbp-0x29]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7)) as u64) as u32 as u64;
            // 44269 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4426c movzx r12d,BYTE PTR [rbp-0x2d]
            r12 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3)) as u64) as u32 as u64;
            // 44271 and r12d,0xf0
            { let r = (r12 as u32 as u64) & 0xf0u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 44278 or r12d,eax
            { let r = (r12 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4427b mov DWORD PTR [rbp-0x34],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x0u64) as u32);
            // 44282 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 44287 lea r14,[rbp-0x34]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            pc = 0x4428b;
        }
        0x4428b => {
            // block 4428b
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 4428b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4428b lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 4428e bt r12d,eax
            fl.cf = bit::<32>((r12 as u32 as u64), (rax as u32 as u64));
            // 44292 jb 0x4429f
            if fl.b() { pc = 0x4429f; continue; }
            pc = 0x44294;
        }
        0x44294 => {
            // block 44294
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 44294 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44294 mov rdi,r15
            rdi = r15;
            // 44297 mov rsi,r14
            rsi = r14;
            // 4429a call 0xebb52
            { let r = { let t = m.env.stub_va(0x31f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4429f;
        }
        0x4429f => {
            // block 4429f
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 4429f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4429f mov DWORD PTR [rbp-0x34],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rbx as u32 as u64)) as u32);
            // 442a2 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 442a5 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 442a8 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 442ab mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 442ad jl 0x4428b
            if fl.l() { pc = 0x4428b; continue; }
            pc = 0x442af;
        }
        0x442af => {
            // block 442af
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 442af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 442af jmp 0x442b4
            pc = 0x442b4; continue;
        }
        0x442b1 => {
            // block 442b1
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 442b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 442b1 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x442b4;
        }
        0x442b4 => {
            // block 442b4
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 442b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 442b4 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 442b7 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 442bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 442bc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 442be pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 442c0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 442c2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 442c4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 442c5 ret
            return Ret { rax, rdx, xmm0: x0 };
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
            { let r = { let t = m.env.code_va(0xcbc54); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b946a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b946c je 0xb952a
            if fl.e() { pc = 0xb952a; continue; }
            pc = 0xb9472;
        }
        0xb9472 => {
            // block b9472
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9472 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9472 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9476 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b947b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b947d je 0xb952a
            if fl.e() { pc = 0xb952a; continue; }
            pc = 0xb9483;
        }
        0xb9483 => {
            // block b9483
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9483 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9487 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.be() { pc = 0xb952c; continue; }
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
            if fl.e() { pc = 0xb94d5; continue; }
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
            if fl.e() { pc = 0xb94dc; continue; }
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
            if fl.ne() { pc = 0xb94c8; continue; }
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
            if fl.ne() { pc = 0xb94df; continue; }
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
            if fl.ne() { pc = 0xb94ba; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b951d inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // b9520 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // b9522 cmp r12d,0x5
            fl = sub::<32>((r12 as u32 as u64), 0x5u64).1;
            // b9526 jne 0xb94b6
            if fl.ne() { pc = 0xb94b6; continue; }
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

/// parity: P2v @0x63e43 size=238
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00063e43<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x63e43;
    loop {
        match pc {
        0x63e43 => {
            // block 63e43
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63e43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e43 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63e44 mov rbp,rsp
            rbp = rsp;
            // 63e47 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 63e49 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 63e4b push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 63e4d push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 63e4f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 63e50 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 63e54 mov r12,rsi
            r12 = rsi;
            // 63e57 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 63e5b mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 63e5f lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // 63e64 cmp rcx,r13
            fl = sub::<64>(rcx, r13).1;
            // 63e67 je 0x63f0a
            if fl.e() { pc = 0x63f0a; continue; }
            pc = 0x63e6d;
        }
        0x63e6d => {
            // block 63e6d
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63e6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e6d lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 63e71 mov r14,rcx
            r14 = rcx;
            // 63e74 jmp 0x63e7d
            pc = 0x63e7d; continue;
        }
        0x63e76 => {
            // block 63e76
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63e76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e76 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 63e7a mov r14,rax
            r14 = rax;
            pc = 0x63e7d;
        }
        0x63e7d => {
            // block 63e7d
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63e7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e7d cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // 63e80 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 63e83 lea rsi,[rip+0xd4e3e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 63e8a lea rcx,[rip+0xc660c] # 0x12a49d
            rcx = m.env.data_va(257130);
            // 63e91 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // 63e95 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 63e98 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 63e9b mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 63e9f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63ea4 mov rbx,rax
            rbx = rax;
            // 63ea7 mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 63eab xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 63ead mov rdi,r15
            rdi = r15;
            // 63eb0 call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63eb5 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 63eb9 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 63ebc jne 0x63ec7
            if fl.ne() { pc = 0x63ec7; continue; }
            pc = 0x63ebe;
        }
        0x63ebe => {
            // block 63ebe
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ebe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ebe shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 63ec1 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 63ec5 jmp 0x63ecf
            pc = 0x63ecf; continue;
        }
        0x63ec7 => {
            // block 63ec7
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ec7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ec7 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 63ecb mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x63ecf;
        }
        0x63ecf => {
            // block 63ecf
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ecf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ecf mov rdi,rbx
            rdi = rbx;
            // 63ed2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63ed7 mov rdi,r15
            rdi = r15;
            // 63eda call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63edf mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 63ee3 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 63ee6 je 0x63ef5
            if fl.e() { pc = 0x63ef5; continue; }
            pc = 0x63ee8;
        }
        0x63ee8 => {
            // block 63ee8
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ee8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ee8 mov rax,rcx
            rax = rcx;
            // 63eeb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 63eee test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 63ef1 jne 0x63ee8
            if fl.ne() { pc = 0x63ee8; continue; }
            pc = 0x63ef3;
        }
        0x63ef3 => {
            // block 63ef3
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ef3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ef3 jmp 0x63f01
            pc = 0x63f01; continue;
        }
        0x63ef5 => {
            // block 63ef5
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ef5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ef5 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 63ef9 cmp QWORD PTR [rax],r14
            fl = sub::<64>(m.r64(rax) as u64, r14).1;
            // 63efc mov r14,rax
            r14 = rax;
            // 63eff jne 0x63ef5
            if fl.ne() { pc = 0x63ef5; continue; }
            pc = 0x63f01;
        }
        0x63f01 => {
            // block 63f01
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63f01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63f01 cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // 63f04 jne 0x63e76
            if fl.ne() { pc = 0x63e76; continue; }
            pc = 0x63f0a;
        }
        0x63f0a => {
            // block 63f0a
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63f0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63f0a mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 63f0e add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 63f12 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 63f13 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 63f15 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 63f17 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 63f19 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 63f1b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 63f1c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x63f1d => {
            // block 63f1d
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63f1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63f1d mov rbx,rax
            rbx = rax;
            // 63f20 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 63f24 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63f29 mov rdi,rbx
            rdi = rbx;
            // 63f2c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            if fl.ge() { pc = 0xc8b11; continue; }
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
            if fl.ne() { pc = 0xc8a95; continue; }
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

/// parity: P2v @0xd0b0 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d0b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd0b0;
    loop {
        match pc {
        0xd0b0 => {
            // block d0b0
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d0b1 mov rbp,rsp
            rbp = rsp;
            // d0b4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d0b6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d0b8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d0b9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d0ba mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d0bd mov r15,rdi
            r15 = rdi;
            // d0c0 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // d0c3 je 0xd134
            if fl.e() { pc = 0xd134; continue; }
            pc = 0xd0c5;
        }
        0xd0c5 => {
            // block d0c5
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0c5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d0c8 mov rdi,r15
            rdi = r15;
            // d0cb call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d0d1 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // d0d6 je 0xd124
            if fl.e() { pc = 0xd124; continue; }
            pc = 0xd0d8;
        }
        0xd0d8 => {
            // block d0d8
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0d8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d0db mov rdi,r15
            rdi = r15;
            // d0de call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d0e4 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // d0e9 je 0xd124
            if fl.e() { pc = 0xd124; continue; }
            pc = 0xd0eb;
        }
        0xd0eb => {
            // block d0eb
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0eb mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d0ee mov rdi,r15
            rdi = r15;
            // d0f1 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d0f7 cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // d0fc je 0xd124
            if fl.e() { pc = 0xd124; continue; }
            pc = 0xd0fe;
        }
        0xd0fe => {
            // block d0fe
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0fe mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d101 mov rdi,r15
            rdi = r15;
            // d104 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d10a cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // d10f je 0xd124
            if fl.e() { pc = 0xd124; continue; }
            pc = 0xd111;
        }
        0xd111 => {
            // block d111
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d111 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d111 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d114 mov rdi,r15
            rdi = r15;
            // d117 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d11d cmp eax,0x10710851
            fl = sub::<32>((rax as u32 as u64), 0x10710851u64).1;
            // d122 jne 0xd134
            if fl.ne() { pc = 0xd134; continue; }
            pc = 0xd124;
        }
        0xd124 => {
            // block d124
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d124 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d124 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d127 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d12b mov rdi,r15
            rdi = r15;
            // d12e call QWORD PTR [rax+0x328]
            { let r = { let t = m.r64(rax.wrapping_add(0x328)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd134;
        }
        0xd134 => {
            // block d134
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d134 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d134 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d137 mov rdi,r15
            rdi = r15;
            // d13a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d140 cmp eax,0x10402100
            fl = sub::<32>((rax as u32 as u64), 0x10402100u64).1;
            // d145 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d148 xor al,r14b
            { let r = (rax as u8 as u64) ^ (r14 as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d14b movzx ebx,al
            rbx = ((rax as u8 as u64)) as u32 as u64;
            // d14e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d151 mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d156 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // d15b mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // d161 mov rdi,r15
            rdi = r15;
            // d164 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d166 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d169 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d16c xor ebx,0x1
            { let r = (rbx as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d16f mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d174 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // d179 mov r8d,0x5
            r8 = (0x5u64) as u32 as u64;
            // d17f mov rdi,r15
            rdi = r15;
            // d182 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d184 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d187 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d18b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d18c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d18e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d190 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d191 ret
            return Ret { rax, rdx, xmm0: x0 };
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
            { let r = { let t = m.env.code_va(0x6911e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x68fea); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 692a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 692a3 je 0x692b1
            if fl.e() { pc = 0x692b1; continue; }
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
            { let r = { let t = m.env.stub_va(0x475); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x692c7; continue; }
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
            if fl.ne() { pc = 0x692ba; continue; }
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
            if fl.ne() { pc = 0x692ca; continue; }
            pc = 0x692d6;
        }
        0x692d6 => {
            // block 692d6
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 692d6 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 692d9 jne 0x6928f
            if fl.ne() { pc = 0x6928f; continue; }
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
            { let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x3914 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003914<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3914;
    loop {
        match pc {
        0x3914 => {
            // block 3914
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 3914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3914 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3915 mov rbp,rsp
            rbp = rsp;
            // 3918 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 391a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 391c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 391e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3920 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3921 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3922 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 3925 mov r12,rdi
            r12 = rdi;
            // 3928 shl r14d,0xe
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0xeu64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 392c and r14d,0x3fffc000
            { let r = (r14 as u32 as u64) & 0x3fffc000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 3933 je 0x39ab
            if fl.e() { pc = 0x39ab; continue; }
            pc = 0x3935;
        }
        0x3935 => {
            // block 3935
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 3935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3935 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 3938 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 393a xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x393d;
        }
        0x393d => {
            // block 393d
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 393d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 393d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3941 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3946 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 394b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 394e mov rdi,r12
            rdi = r12;
            // 3951 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 3953 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3956 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 395a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 395f mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 3964 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3969 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 396c mov rdi,r12
            rdi = r12;
            // 396f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3972 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3976 mov rdi,r12
            rdi = r12;
            // 3979 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 397f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3983 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // 3988 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 398d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3990 mov rdi,r12
            rdi = r12;
            // 3993 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 3997 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 399a cmp DWORD PTR [rbp-0x2c],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0xffffffffu64).1;
            // 399e jne 0x39ae
            if fl.ne() { pc = 0x39ae; continue; }
            pc = 0x39a0;
        }
        0x39a0 => {
            // block 39a0
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39a0 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 39a3 add ebx,0x4
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x4u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 39a6 cmp r15d,r14d
            fl = sub::<32>((r15 as u32 as u64), (r14 as u32 as u64)).1;
            // 39a9 jb 0x393d
            if fl.b() { pc = 0x393d; continue; }
            pc = 0x39ab;
        }
        0x39ab => {
            // block 39ab
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39ab mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x39ae;
        }
        0x39ae => {
            // block 39ae
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39ae mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 39b1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 39b5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 39b6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 39b8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 39ba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 39bc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 39be pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 39bf ret
            return Ret { rax, rdx, xmm0: x0 };
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
