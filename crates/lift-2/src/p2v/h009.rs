//! Mechanically lifted functions (scripts/lift_run.py), module h009. Each one is checked against the original
//! in the world harness (tests/world_h009.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x2d6d0 size=364
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0002d6d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2d6d0;
    loop {
        match pc {
        0x2d6d0 => {
            // block 2d6d0
            #[cfg(feature = "trace")] eprintln!("f_0002d6d0 pc 2d6d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d6d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d6d1 mov rbp,rsp
            rbp = rsp;
            // 2d6d4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d6d6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d6d8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2d6da push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2d6dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d6dd sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 2d6e1 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 2d6e5 mov r12,rdi
            r12 = rdi;
            // 2d6e8 movabs r15,0x1555555555555555
            r15 = 0x1555_5555_5555_5555u64;
            // 2d6f2 mov r14,QWORD PTR [r12]
            r14 = m.r64(r12) as u64;
            // 2d6f6 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2d6fb sub rbx,r14
            { let (r, f) = sub::<64>(rbx, r14);
              rbx = r;
              fl = f; }
            // 2d6fe sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 2d702 movabs rsi,0xaaaaaaaaaaaaaaab
            rsi = 0xaaaa_aaaa_aaaa_aaabu64;
            // 2d70c imul rbx,rsi
            { let (r, f) = imul::<64>(rbx, rsi); rbx = r; fl = f; }
            // 2d710 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2d713 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 2d716 jbe 0x2d72a
            if fl.be() { pc = 0x2d72a; continue; }
            pc = 0x2d718;
        }
        0x2d718 => {
            // block 2d718
            #[cfg(feature = "trace")] eprintln!("f_0002d6d0 pc 2d718 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d718 mov rdi,r12
            rdi = r12;
            // 2d71b mov r14,rsi
            r14 = rsi;
            // 2d71e call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x2d72a => {
            // block 2d72a
            #[cfg(feature = "trace")] eprintln!("f_0002d6d0 pc 2d72a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d72a lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 2d72f mov rcx,QWORD PTR [r12+0x10]
            rcx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2d734 mov QWORD PTR [rbp-0x60],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rcx) as u64);
            // 2d738 sub rcx,r14
            { let (r, f) = sub::<64>(rcx, r14);
              rcx = r;
              fl = f; }
            // 2d73b sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 2d73f imul rcx,rsi
            { let (r, f) = imul::<64>(rcx, rsi); rcx = r; fl = f; }
            // 2d743 movabs rdx,0xaaaaaaaaaaaaaa9
            rdx = 0xaaa_aaaa_aaaa_aaa9u64;
            // 2d74d cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 2d750 ja 0x2d75f
            if fl.a() { pc = 0x2d75f; continue; }
            pc = 0x2d752;
        }
        0x2d752 => {
            // block 2d752
            #[cfg(feature = "trace")] eprintln!("f_0002d6d0 pc 2d752 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d752 add rcx,rcx
            { let (r, f) = add::<64>(rcx, rcx);
              rcx = r;
              fl = f; }
            // 2d755 cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 2d758 cmovb rcx,rbx
            if fl.b() { rcx = rbx; }
            // 2d75c mov r15,rcx
            r15 = rcx;
            pc = 0x2d75f;
        }
        0x2d75f => {
            // block 2d75f
            #[cfg(feature = "trace")] eprintln!("f_0002d6d0 pc 2d75f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d75f mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2d764 mov QWORD PTR [rbp-0x68],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r13) as u64);
            // 2d768 sub r13,r14
            { let (r, f) = sub::<64>(r13, r14);
              r13 = r;
              fl = f; }
            // 2d76b mov rbx,r13
            rbx = r13;
            // 2d76e sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 2d772 imul rbx,rsi
            { let (r, f) = imul::<64>(rbx, rsi); rbx = r; fl = f; }
            // 2d776 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 2d77e mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 2d782 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 2d784 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2d787 je 0x2d79d
            if fl.e() { pc = 0x2d79d; continue; }
            pc = 0x2d789;
        }
        0x2d789 => {
            // block 2d789
            #[cfg(feature = "trace")] eprintln!("f_0002d6d0 pc 2d789 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d789 lea rax,[r15*4+0x0]
            rax = r15.wrapping_mul(4);
            // 2d791 lea rdi,[rax+rax*2]
            rdi = rax.wrapping_add(rax.wrapping_mul(2));
            // 2d795 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d79a mov rcx,rax
            rcx = rax;
            pc = 0x2d79d;
        }
        0x2d79d => {
            // block 2d79d
            #[cfg(feature = "trace")] eprintln!("f_0002d6d0 pc 2d79d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d79d lea rax,[rbx+rbx*2]
            rax = rbx.wrapping_add(rbx.wrapping_mul(2));
            // 2d7a1 lea rdx,[r15+r15*2]
            rdx = r15.wrapping_add(r15.wrapping_mul(2));
            // 2d7a5 lea rdx,[rcx+rdx*4]
            rdx = rcx.wrapping_add(rdx.wrapping_mul(4));
            // 2d7a9 mov QWORD PTR [rbp-0x70],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rdx) as u64);
            // 2d7ad mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 2d7b1 mov edx,DWORD PTR [rsi+0x8]
            rdx = (m.r32(rsi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 2d7b4 mov DWORD PTR [rcx+rax*4+0x8],edx
            m.w32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2d7b8 mov rdx,QWORD PTR [rsi]
            rdx = m.r64(rsi) as u64;
            // 2d7bb mov QWORD PTR [rcx+rax*4],rdx
            m.w64(rcx.wrapping_add(rax.wrapping_mul(4)), (rdx) as u64);
            // 2d7bf lea r15,[rcx+rax*4+0xc]
            r15 = rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xc);
            // 2d7c4 movabs rdx,0xd555555555555555
            rdx = 0xd555_5555_5555_5555u64;
            // 2d7ce mov rax,r13
            rax = r13;
            // 2d7d1 imul rdx
            { let (lo, hi, f) = imul1::<64>(rax, rdx); rax = lo; rdx = hi; fl = f; }
            // 2d7d4 mov rax,rdx
            rax = rdx;
            // 2d7d7 shr rax,0x3f
            { let (r, f) = shr::<64>(rax, 0x3fu64, fl); rax = r; fl = f; }
            // 2d7db sar rdx,1
            { let (r, f) = sar::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2d7de add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 2d7e1 add rdx,rbx
            { let (r, f) = add::<64>(rdx, rbx);
              rdx = r;
              fl = f; }
            // 2d7e4 lea rax,[rdx+rdx*2]
            rax = rdx.wrapping_add(rdx.wrapping_mul(2));
            // 2d7e8 lea rbx,[rcx+rax*4]
            rbx = rcx.wrapping_add(rax.wrapping_mul(4));
            // 2d7ec mov rdi,rbx
            rdi = rbx;
            // 2d7ef mov rsi,r14
            rsi = r14;
            // 2d7f2 mov rdx,r13
            rdx = r13;
            // 2d7f5 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d7fa mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 2d7fe mov QWORD PTR [rbp-0x48],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r14) as u64);
            // 2d802 mov QWORD PTR [r12+0x8],r15
            m.w64(r12.wrapping_add(0x8), (r15) as u64);
            // 2d807 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 2d80b mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 2d80f mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2d813 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 2d818 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 2d81c mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 2d820 mov QWORD PTR [rbp-0x50],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r14) as u64);
            // 2d824 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2d828 call 0xeba9e
            { let r = { let t = m.env.stub_va(0x26b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d82d add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 2d831 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d832 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2d834 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2d836 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d838 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d83a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d83b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2eb06 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002eb06<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2eb06;
    loop {
        match pc {
        0x2eb06 => {
            // block 2eb06
            #[cfg(feature = "trace")] eprintln!("f_0002eb06 pc 2eb06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2eb06 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2eb07 mov rbp,rsp
            rbp = rsp;
            // 2eb0a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2eb0b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2eb0c mov ebx,0xffffffff
            rbx = (0xffffffffu64) as u32 as u64;
            // 2eb11 lock xadd DWORD PTR [rdi+0x64],ebx
            { let a = m.r32(rdi.wrapping_add(0x64)) as u64; let b = (rbx as u32 as u64); let (r, f) = add::<32>(a, b); rbx = (a) as u32 as u64; m.w32(rdi.wrapping_add(0x64), (r) as u32); fl = f; }
            // 2eb16 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2eb19 je 0x2eb25
            if fl.e() { pc = 0x2eb25; continue; }
            pc = 0x2eb1b;
        }
        0x2eb1b => {
            // block 2eb1b
            #[cfg(feature = "trace")] eprintln!("f_0002eb06 pc 2eb1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2eb1b test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 2eb1d jne 0x2eb25
            if fl.ne() { pc = 0x2eb25; continue; }
            pc = 0x2eb1f;
        }
        0x2eb1f => {
            // block 2eb1f
            #[cfg(feature = "trace")] eprintln!("f_0002eb06 pc 2eb1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2eb1f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2eb22 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2eb25;
        }
        0x2eb25 => {
            // block 2eb25
            #[cfg(feature = "trace")] eprintln!("f_0002eb06 pc 2eb25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2eb25 dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 2eb27 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 2eb29 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2eb2d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2eb2e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2eb2f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x424ce size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000424ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x424ce;
    loop {
        match pc {
        0x424ce => {
            // block 424ce
            #[cfg(feature = "trace")] eprintln!("f_000424ce pc 424ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 424ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 424cf mov rbp,rsp
            rbp = rsp;
            // 424d2 mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 424d5 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 424d8 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 424dc mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 424de lea rdx,[rip+0xbdf8b] # 0x100470
            rdx = m.env.data_va(85053);
            // 424e5 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 424e8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 424ed xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 424f0 mov edx,r9d
            rdx = ((r9 as u32 as u64)) as u32 as u64;
            // 424f3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 424f4 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6530a size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006530a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6530a;
    loop {
        match pc {
        0x6530a => {
            // block 6530a
            #[cfg(feature = "trace")] eprintln!("f_0006530a pc 6530a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6530a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6530b mov rbp,rsp
            rbp = rsp;
            // 6530e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 65310 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65312 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65314 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65315 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 65319 mov r15,rsi
            r15 = rsi;
            // 6531c mov r14,rdi
            r14 = rdi;
            // 6531f lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 65323 mov rdx,r15
            rdx = r15;
            // 65326 call 0xebb1c
            { let r = { let t = m.env.stub_va(0x2e9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6532b mov r12,rax
            r12 = rax;
            // 6532e mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 65332 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 65335 je 0x6533b
            if fl.e() { pc = 0x6533b; continue; }
            pc = 0x65337;
        }
        0x65337 => {
            // block 65337
            #[cfg(feature = "trace")] eprintln!("f_0006530a pc 65337 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65337 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65339 jmp 0x65362
            pc = 0x65362; continue;
        }
        0x6533b => {
            // block 6533b
            #[cfg(feature = "trace")] eprintln!("f_0006530a pc 6533b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6533b mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 65340 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65345 mov rbx,rax
            rbx = rax;
            // 65348 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 6534b mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 6534e mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 65352 mov rdi,r14
            rdi = r14;
            // 65355 mov rdx,r12
            rdx = r12;
            // 65358 mov rcx,rbx
            rcx = rbx;
            // 6535b call 0xebcc6
            { let r = { let t = m.env.stub_va(0x493); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65360 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x65362;
        }
        0x65362 => {
            // block 65362
            #[cfg(feature = "trace")] eprintln!("f_0006530a pc 65362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65362 mov rax,rbx
            rax = rbx;
            // 65365 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 65369 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6536a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6536c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6536e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 65370 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 65371 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x13ab0 size=134
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00013ab0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x13ab0;
    loop {
        match pc {
        0x13ab0 => {
            // block 13ab0
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13ab0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13ab0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 13ab1 mov rbp,rsp
            rbp = rsp;
            // 13ab4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 13ab6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 13ab8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 13aba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 13abb mov r15,rsi
            r15 = rsi;
            // 13abe mov r14,rdi
            r14 = rdi;
            // 13ac1 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 13ac4 je 0x13b2d
            if fl.e() { pc = 0x13b2d; continue; }
            pc = 0x13ac6;
        }
        0x13ac6 => {
            // block 13ac6
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13ac6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13ac6 mov al,BYTE PTR [r15]
            rax = (rax & !0xff) | ((m.r8(r15) as u64) as u8 as u64);
            // 13ac9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 13acb je 0x13b2d
            if fl.e() { pc = 0x13b2d; continue; }
            pc = 0x13acd;
        }
        0x13acd => {
            // block 13acd
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13acd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13acd add r14,0x298
            { let (r, f) = add::<64>(r14, 0x298u64);
              r14 = r;
              fl = f; }
            // 13ad4 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 13ada jmp 0x13ae3
            pc = 0x13ae3; continue;
        }
        0x13adc => {
            // block 13adc
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13adc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13adc mov al,BYTE PTR [r15+rbx*1]
            rax = (rax & !0xff) | ((m.r8(r15.wrapping_add(rbx)) as u64) as u8 as u64);
            // 13ae0 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            pc = 0x13ae3;
        }
        0x13ae3 => {
            // block 13ae3
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13ae3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13ae3 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 13ae5 add cl,0xbf
            { let (r, f) = add::<8>((rcx as u8 as u64), 0xbfu64);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 13ae8 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 13aeb cmp ecx,0x1a
            fl = sub::<32>((rcx as u32 as u64), 0x1au64).1;
            // 13aee jb 0x13b12
            if fl.b() { pc = 0x13b12; continue; }
            pc = 0x13af0;
        }
        0x13af0 => {
            // block 13af0
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13af0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13af0 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 13af2 add cl,0x9f
            { let (r, f) = add::<8>((rcx as u8 as u64), 0x9fu64);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 13af5 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 13af8 cmp ecx,0x1a
            fl = sub::<32>((rcx as u32 as u64), 0x1au64).1;
            // 13afb jb 0x13b12
            if fl.b() { pc = 0x13b12; continue; }
            pc = 0x13afd;
        }
        0x13afd => {
            // block 13afd
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13afd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13afd movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            // 13b00 cmp ecx,0x5f
            fl = sub::<32>((rcx as u32 as u64), 0x5fu64).1;
            // 13b03 je 0x13b12
            if fl.e() { pc = 0x13b12; continue; }
            pc = 0x13b05;
        }
        0x13b05 => {
            // block 13b05
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13b05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13b05 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 13b07 add cl,0xd0
            { let (r, f) = add::<8>((rcx as u8 as u64), 0xd0u64);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 13b0a movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 13b0d cmp ecx,0x9
            fl = sub::<32>((rcx as u32 as u64), 0x9u64).1;
            // 13b10 ja 0x13b2d
            if fl.a() { pc = 0x13b2d; continue; }
            pc = 0x13b12;
        }
        0x13b12 => {
            // block 13b12
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13b12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13b12 movsx esi,al
            rsi = (sx::<8>((rax as u8 as u64)) as u64) as u32 as u64;
            // 13b15 mov rdi,r14
            rdi = r14;
            // 13b18 call 0xec020
            { let r = { let t = m.env.stub_va(0x7ed); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13b1d mov ebx,r12d
            rbx = ((r12 as u32 as u64)) as u32 as u64;
            // 13b20 mov rdi,r15
            rdi = r15;
            // 13b23 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13b28 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 13b2b jb 0x13adc
            if fl.b() { pc = 0x13adc; continue; }
            pc = 0x13b2d;
        }
        0x13b2d => {
            // block 13b2d
            #[cfg(feature = "trace")] eprintln!("f_00013ab0 pc 13b2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13b2d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 13b2e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 13b30 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 13b32 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 13b34 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 13b35 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a430 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a430<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a430;
    loop {
        match pc {
        0x1a430 => {
            // block 1a430
            #[cfg(feature = "trace")] eprintln!("f_0001a430 pc 1a430 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a430 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a431 mov rbp,rsp
            rbp = rsp;
            // 1a434 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a436 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a437 mov rbx,rsi
            rbx = rsi;
            // 1a43a mov r14,rdi
            r14 = rdi;
            // 1a43d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 1a440 je 0x1a465
            if fl.e() { pc = 0x1a465; continue; }
            pc = 0x1a442;
        }
        0x1a442 => {
            // block 1a442
            #[cfg(feature = "trace")] eprintln!("f_0001a430 pc 1a442 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a442 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 1a445 mov rdi,r14
            rdi = r14;
            // 1a448 call 0x1a430
            { let r = { let t = m.env.code_va(0x1a430); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a44d mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1a451 mov rdi,r14
            rdi = r14;
            // 1a454 call 0x1a430
            { let r = { let t = m.env.code_va(0x1a430); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a459 mov rdi,rbx
            rdi = rbx;
            // 1a45c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a45d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a45f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a460 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x1a465 => {
            // block 1a465
            #[cfg(feature = "trace")] eprintln!("f_0001a430 pc 1a465 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a465 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a466 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a468 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a469 ret
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
            if fl.be() { pc = 0x19e63; continue; }
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
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e29 mov rax,rbx
            rax = rbx;
            // 19e2c shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19e30 je 0x19e3a
            if fl.e() { pc = 0x19e3a; continue; }
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
            if fl.a() { pc = 0x19e7c; continue; }
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
            if fl.be() { pc = 0x19ea3; continue; }
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
            { let r = { let t = m.env.stub_va(0x211); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e8e mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19e92 mov rsi,r12
            rsi = r12;
            // 19e95 mov rdx,r14
            rdx = r14;
            // 19e98 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19ebb mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19ebf test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 19ec2 je 0x19ee0
            if fl.e() { pc = 0x19ee0; continue; }
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
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x19f01; continue; }
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
            rax = !rax;
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

/// parity: P2v @0x51e68 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051e68<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51e68;
    loop {
        match pc {
        0x51e68 => {
            // block 51e68
            #[cfg(feature = "trace")] eprintln!("f_00051e68 pc 51e68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51e68 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51e69 mov rbp,rsp
            rbp = rsp;
            // 51e6c mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51e6e mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51e71 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51e75 mov esi,0x283f
            rsi = (0x283fu64) as u32 as u64;
            // 51e7a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51e7f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51e82 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51e83 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6814e size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006814e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6814e;
    loop {
        match pc {
        0x6814e => {
            // block 6814e
            #[cfg(feature = "trace")] eprintln!("f_0006814e pc 6814e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6814e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6814f mov rbp,rsp
            rbp = rsp;
            // 68152 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 68154 lea rcx,[rip+0x99b35] # 0x101c90
            rcx = m.env.data_va(91229);
            // 6815b mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 6815e mov eax,DWORD PTR [rdi+rax*4+0x168]
            rax = (m.r32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x168)) as u64) as u32 as u64;
            // 68165 mov DWORD PTR [rsi],eax
            m.w32(rsi, ((rax as u32 as u64)) as u32);
            // 68167 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 68169 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6816a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe80ec size=12
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e80ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe80ec;
    loop {
        match pc {
        0xe80ec => {
            // block e80ec
            #[cfg(feature = "trace")] eprintln!("f_000e80ec pc e80ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e80ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e80ed mov rbp,rsp
            rbp = rsp;
            // e80f0 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e80f3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e80f4 jmp QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x13004 size=1688
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00013004<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x13004;
    loop {
        match pc {
        0x13004 => {
            // block 13004
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13004 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13004 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 13005 mov rbp,rsp
            rbp = rsp;
            // 13008 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1300a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1300c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1300e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 13010 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 13011 sub rsp,0x168
            { let (r, f) = sub::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // 13018 mov r13,rdx
            r13 = rdx;
            // 1301b mov r15,rsi
            r15 = rsi;
            // 1301e mov QWORD PTR [rbp-0x160],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rdi) as u64);
            // 13025 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 1302c lea r12,[rbp-0x150]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 13033 mov rax,QWORD PTR [rip+0x12d196] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 1303a lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 1303e mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // 13045 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 13049 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // 13050 mov rsi,r12
            rsi = r12;
            // 13053 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13058 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 13060 mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // 13067 mov rbx,QWORD PTR [rip+0x12d352] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // 1306e lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 13072 mov QWORD PTR [rbp-0x158],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r14) as u64);
            // 13079 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // 1307d mov QWORD PTR [rbp-0xe8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rbx) as u64);
            // 13084 mov rdi,r12
            rdi = r12;
            // 13087 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1308c mov QWORD PTR [rbp-0x170],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rbx) as u64);
            // 13093 mov QWORD PTR [rbp-0x168],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (r14) as u64);
            // 1309a mov rbx,QWORD PTR [rip+0x12d307] # 0x1403a8
            rbx = m.r64(m.env.data_va(346997)) as u64;
            // 130a1 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 130a5 mov QWORD PTR [rbp-0x150],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rbx) as u64);
            // 130ac lea r14,[rbp-0x110]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 130b3 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 130be mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 130c9 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 130d4 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 130df mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // 130e9 xorps xmm0,xmm0
            x0 = 0;
            // 130ec movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 130f0 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 130f8 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 130fc mov rdi,r12
            rdi = r12;
            // 130ff call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13104 mov QWORD PTR [rbp-0x178],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rbx) as u64);
            // 1310b lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1310f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13114 mov rbx,r15
            rbx = r15;
            // 13117 mov r14,QWORD PTR [rbx+0x248]
            r14 = m.r64(rbx.wrapping_add(0x248)) as u64;
            // 1311e mov DWORD PTR [r13+0x0],0x0
            m.w32(r13, (0x0u64) as u32);
            // 13126 lea rsi,[rip+0xecad9] # 0xffc06
            rsi = m.env.data_va(82899);
            // 1312d mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 13132 mov rdi,r14
            rdi = r14;
            // 13135 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1313a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 1313c je 0x1315b
            if fl.e() { pc = 0x1315b; continue; }
            pc = 0x1313e;
        }
        0x1313e => {
            // block 1313e
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1313e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1313e lea rsi,[rip+0x116ec6] # 0x12a00b
            rsi = m.env.data_va(255960);
            // 13145 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 1314c mov edx,0x33
            rdx = (0x33u64) as u32 as u64;
            // 13151 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13156 jmp 0x13573
            pc = 0x13573; continue;
        }
        0x1315b => {
            // block 1315b
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1315b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1315b movzx r15d,BYTE PTR [r14+0xd]
            r15 = (m.r8(r14.wrapping_add(0xd)) as u64) as u32 as u64;
            // 13160 cmp r15d,0x61
            fl = sub::<32>((r15 as u32 as u64), 0x61u64).1;
            // 13164 jne 0x1328e
            if fl.ne() { pc = 0x1328e; continue; }
            pc = 0x1316a;
        }
        0x1316a => {
            // block 1316a
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1316a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1316a mov QWORD PTR [rbp-0x180],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (r13) as u64);
            // 13171 mov r15w,WORD PTR [r14+0xe]
            r15 = (r15 & !0xffff) | ((m.r16(r14.wrapping_add(0xe)) as u64) as u16 as u64);
            // 13176 lea rsi,[r14+0x10]
            rsi = r14.wrapping_add(0x10);
            // 1317a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1317d mov rax,QWORD PTR [rax+0x78]
            rax = m.r64(rax.wrapping_add(0x78)) as u64;
            // 13181 mov rdi,rbx
            rdi = rbx;
            // 13184 mov r13,rbx
            r13 = rbx;
            // 13187 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13189 rol r15w,0x8
            { let (r, f) = rol::<16>((r15 as u16 as u64), 0x8u64, fl); r15 = (r15 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // 1318e movzx r15d,r15w
            r15 = ((r15 as u16 as u64)) as u32 as u64;
            // 13192 movzx ebx,BYTE PTR [r15+r14*1+0x10]
            rbx = (m.r8(r15.wrapping_add(r14).wrapping_add(0x10)) as u64) as u32 as u64;
            // 13198 cmp ebx,0x62
            fl = sub::<32>((rbx as u32 as u64), 0x62u64).1;
            // 1319b jne 0x132f5
            if fl.ne() { pc = 0x132f5; continue; }
            pc = 0x131a1;
        }
        0x131a1 => {
            // block 131a1
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 131a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 131a1 mov r12w,WORD PTR [r15+r14*1+0x11]
            r12 = (r12 & !0xffff) | ((m.r16(r15.wrapping_add(r14).wrapping_add(0x11)) as u64) as u16 as u64);
            // 131a7 lea rsi,[r14+r15*1+0x13]
            rsi = r14.wrapping_add(r15).wrapping_add(0x13);
            // 131ac lea rdi,[r13+0x2b0]
            rdi = r13.wrapping_add(0x2b0);
            // 131b3 mov QWORD PTR [rbp-0x188],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rdi) as u64);
            // 131ba call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 131bf rol r12w,0x8
            { let (r, f) = rol::<16>((r12 as u16 as u64), 0x8u64, fl); r12 = (r12 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // 131c4 lea rbx,[r15+0x13]
            rbx = r15.wrapping_add(0x13);
            // 131c8 movzx eax,r12w
            rax = ((r12 as u16 as u64)) as u32 as u64;
            // 131cc add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 131cf lea r12d,[r15+rax*1+0x11]
            r12 = (r15.wrapping_add(rax).wrapping_add(0x11)) as u32 as u64;
            // 131d4 movzx r15d,BYTE PTR [r14+rbx*1]
            r15 = (m.r8(r14.wrapping_add(rbx)) as u64) as u32 as u64;
            // 131d9 cmp r15d,0x63
            fl = sub::<32>((r15 as u32 as u64), 0x63u64).1;
            // 131dd jne 0x1335d
            if fl.ne() { pc = 0x1335d; continue; }
            pc = 0x131e3;
        }
        0x131e3 => {
            // block 131e3
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 131e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 131e3 mov r15w,WORD PTR [rbx+r14*1+0x1]
            r15 = (r15 & !0xffff) | ((m.r16(rbx.wrapping_add(r14).wrapping_add(0x1)) as u64) as u16 as u64);
            // 131e9 lea rsi,[r14+rbx*1+0x3]
            rsi = r14.wrapping_add(rbx).wrapping_add(0x3);
            // 131ee lea rdi,[r13+0x268]
            rdi = r13.wrapping_add(0x268);
            // 131f5 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 131fa rol r15w,0x8
            { let (r, f) = rol::<16>((r15 as u16 as u64), 0x8u64, fl); r15 = (r15 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // 131ff add rbx,0x3
            { let (r, f) = add::<64>(rbx, 0x3u64);
              rbx = r;
              fl = f; }
            // 13203 movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // 13207 add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 1320a lea r12d,[r12+rax*1+0x2]
            r12 = (r12.wrapping_add(rax).wrapping_add(0x2)) as u32 as u64;
            // 1320f movzx r15d,BYTE PTR [r14+rbx*1]
            r15 = (m.r8(r14.wrapping_add(rbx)) as u64) as u32 as u64;
            // 13214 cmp r15d,0x64
            fl = sub::<32>((r15 as u32 as u64), 0x64u64).1;
            // 13218 jne 0x133c2
            if fl.ne() { pc = 0x133c2; continue; }
            pc = 0x1321e;
        }
        0x1321e => {
            // block 1321e
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1321e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1321e mov r15w,WORD PTR [rbx+r14*1+0x1]
            r15 = (r15 & !0xffff) | ((m.r16(rbx.wrapping_add(r14).wrapping_add(0x1)) as u64) as u16 as u64);
            // 13224 lea rsi,[r14+rbx*1+0x3]
            rsi = r14.wrapping_add(rbx).wrapping_add(0x3);
            // 13229 lea rdi,[r13+0x280]
            rdi = r13.wrapping_add(0x280);
            // 13230 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13235 rol r15w,0x8
            { let (r, f) = rol::<16>((r15 as u16 as u64), 0x8u64, fl); r15 = (r15 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // 1323a add rbx,0x3
            { let (r, f) = add::<64>(rbx, 0x3u64);
              rbx = r;
              fl = f; }
            // 1323e movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // 13242 add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 13245 lea r12d,[r12+rax*1+0x2]
            r12 = (r12.wrapping_add(rax).wrapping_add(0x2)) as u32 as u64;
            // 1324a movzx r15d,BYTE PTR [r14+rbx*1]
            r15 = (m.r8(r14.wrapping_add(rbx)) as u64) as u32 as u64;
            // 1324f cmp r15d,0x65
            fl = sub::<32>((r15 as u32 as u64), 0x65u64).1;
            // 13253 jne 0x13427
            if fl.ne() { pc = 0x13427; continue; }
            pc = 0x13259;
        }
        0x13259 => {
            // block 13259
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13259 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13259 mov eax,DWORD PTR [rbx+r14*1+0x1]
            rax = (m.r32(rbx.wrapping_add(r14).wrapping_add(0x1)) as u64) as u32 as u64;
            // 1325e bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            // 13260 mov rsi,r13
            rsi = r13;
            // 13263 mov DWORD PTR [rsi+0x2e0],eax
            m.w32(rsi.wrapping_add(0x2e0), ((rax as u32 as u64)) as u32);
            // 13269 mov al,BYTE PTR [rsi+0x2b0]
            rax = (rax & !0xff) | ((m.r8(rsi.wrapping_add(0x2b0)) as u64) as u8 as u64);
            // 1326f and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 13271 mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 13278 jne 0x1348c
            if fl.ne() { pc = 0x1348c; continue; }
            pc = 0x1327e;
        }
        0x1327e => {
            // block 1327e
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1327e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1327e mov rdi,QWORD PTR [rbp-0x188]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 13285 lea rcx,[rdi+0x1]
            rcx = rdi.wrapping_add(0x1);
            // 13289 jmp 0x1349a
            pc = 0x1349a; continue;
        }
        0x1328e => {
            // block 1328e
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1328e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1328e lea rsi,[rip+0x116daa] # 0x12a03f
            rsi = m.env.data_va(256012);
            // 13295 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 1329c mov edx,0x2a
            rdx = (0x2au64) as u32 as u64;
            // 132a1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 132a6 mov esi,0xd
            rsi = (0xdu64) as u32 as u64;
            // 132ab mov rdi,rax
            rdi = rax;
            // 132ae call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 132b3 lea rsi,[rip+0x116db0] # 0x12a06a
            rsi = m.env.data_va(256055);
            // 132ba mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 132bf mov rdi,rax
            rdi = rax;
            // 132c2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 132c7 mov BYTE PTR [rbp-0x2d],r15b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), ((r15 as u8 as u64)) as u8);
            // 132cb lea rsi,[rbp-0x2d]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd3);
            // 132cf mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 132d4 mov rdi,rax
            rdi = rax;
            // 132d7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 132dc lea rsi,[rip+0x116da5] # 0x12a088
            rsi = m.env.data_va(256085);
            // 132e3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 132e8 mov rdi,rax
            rdi = rax;
            // 132eb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 132f0 jmp 0x13573
            pc = 0x13573; continue;
        }
        0x132f5 => {
            // block 132f5
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 132f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 132f5 lea rsi,[rip+0x116d43] # 0x12a03f
            rsi = m.env.data_va(256012);
            // 132fc lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 13303 mov edx,0x2a
            rdx = (0x2au64) as u32 as u64;
            // 13308 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1330d add r15d,0xf
            { let (r, f) = add::<32>((r15 as u32 as u64), 0xfu64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 13311 mov rdi,rax
            rdi = rax;
            // 13314 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 13317 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1331c lea rsi,[rip+0x116d67] # 0x12a08a
            rsi = m.env.data_va(256087);
            // 13323 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 13328 mov rdi,rax
            rdi = rax;
            // 1332b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13330 mov BYTE PTR [rbp-0x2c],bl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rbx as u8 as u64)) as u8);
            // 13333 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 13337 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1333c mov rdi,rax
            rdi = rax;
            // 1333f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13344 lea rsi,[rip+0x116d3d] # 0x12a088
            rsi = m.env.data_va(256085);
            // 1334b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 13350 mov rdi,rax
            rdi = rax;
            // 13353 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13358 jmp 0x13573
            pc = 0x13573; continue;
        }
        0x1335d => {
            // block 1335d
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1335d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1335d lea rsi,[rip+0x116cdb] # 0x12a03f
            rsi = m.env.data_va(256012);
            // 13364 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 1336b mov edx,0x2a
            rdx = (0x2au64) as u32 as u64;
            // 13370 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13375 mov rdi,rax
            rdi = rax;
            // 13378 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 1337b call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13380 lea rsi,[rip+0x116d21] # 0x12a0a8
            rsi = m.env.data_va(256117);
            // 13387 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 1338c mov rdi,rax
            rdi = rax;
            // 1338f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13394 mov BYTE PTR [rbp-0x2b],r15b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd5), ((r15 as u8 as u64)) as u8);
            // 13398 lea rsi,[rbp-0x2b]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd5);
            // 1339c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 133a1 mov rdi,rax
            rdi = rax;
            // 133a4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 133a9 lea rsi,[rip+0x116cd8] # 0x12a088
            rsi = m.env.data_va(256085);
            // 133b0 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 133b5 mov rdi,rax
            rdi = rax;
            // 133b8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 133bd jmp 0x13573
            pc = 0x13573; continue;
        }
        0x133c2 => {
            // block 133c2
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 133c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 133c2 lea rsi,[rip+0x116c76] # 0x12a03f
            rsi = m.env.data_va(256012);
            // 133c9 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 133d0 mov edx,0x2a
            rdx = (0x2au64) as u32 as u64;
            // 133d5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 133da mov rdi,rax
            rdi = rax;
            // 133dd mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 133e0 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 133e5 lea rsi,[rip+0x116cda] # 0x12a0c6
            rsi = m.env.data_va(256147);
            // 133ec mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 133f1 mov rdi,rax
            rdi = rax;
            // 133f4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 133f9 mov BYTE PTR [rbp-0x2a],r15b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd6), ((r15 as u8 as u64)) as u8);
            // 133fd lea rsi,[rbp-0x2a]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd6);
            // 13401 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 13406 mov rdi,rax
            rdi = rax;
            // 13409 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1340e lea rsi,[rip+0x116c73] # 0x12a088
            rsi = m.env.data_va(256085);
            // 13415 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1341a mov rdi,rax
            rdi = rax;
            // 1341d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13422 jmp 0x13573
            pc = 0x13573; continue;
        }
        0x13427 => {
            // block 13427
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13427 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13427 lea rsi,[rip+0x116c11] # 0x12a03f
            rsi = m.env.data_va(256012);
            // 1342e lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 13435 mov edx,0x2a
            rdx = (0x2au64) as u32 as u64;
            // 1343a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1343f mov rdi,rax
            rdi = rax;
            // 13442 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 13445 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1344a lea rsi,[rip+0x116c93] # 0x12a0e4
            rsi = m.env.data_va(256177);
            // 13451 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 13456 mov rdi,rax
            rdi = rax;
            // 13459 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1345e mov BYTE PTR [rbp-0x29],r15b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7), ((r15 as u8 as u64)) as u8);
            // 13462 lea rsi,[rbp-0x29]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // 13466 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1346b mov rdi,rax
            rdi = rax;
            // 1346e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13473 lea rsi,[rip+0x116c0e] # 0x12a088
            rsi = m.env.data_va(256085);
            // 1347a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1347f mov rdi,rax
            rdi = rax;
            // 13482 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13487 jmp 0x13573
            pc = 0x13573; continue;
        }
        0x1348c => {
            // block 1348c
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1348c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1348c mov rcx,QWORD PTR [rsi+0x2c0]
            rcx = m.r64(rsi.wrapping_add(0x2c0)) as u64;
            // 13493 mov rdi,QWORD PTR [rbp-0x188]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            pc = 0x1349a;
        }
        0x1349a => {
            // block 1349a
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1349a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1349a movzx ecx,BYTE PTR [rcx]
            rcx = (m.r8(rcx) as u64) as u32 as u64;
            // 1349d cmp ecx,0x35
            fl = sub::<32>((rcx as u32 as u64), 0x35u64).1;
            // 134a0 je 0x134d4
            if fl.e() { pc = 0x134d4; continue; }
            pc = 0x134a2;
        }
        0x134a2 => {
            // block 134a2
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134a2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 134a4 jne 0x134ac
            if fl.ne() { pc = 0x134ac; continue; }
            pc = 0x134a6;
        }
        0x134a6 => {
            // block 134a6
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134a6 lea rcx,[rdi+0x1]
            rcx = rdi.wrapping_add(0x1);
            // 134aa jmp 0x134b3
            pc = 0x134b3; continue;
        }
        0x134ac => {
            // block 134ac
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134ac mov rcx,QWORD PTR [rsi+0x2c0]
            rcx = m.r64(rsi.wrapping_add(0x2c0)) as u64;
            pc = 0x134b3;
        }
        0x134b3 => {
            // block 134b3
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134b3 movzx ecx,BYTE PTR [rcx]
            rcx = (m.r8(rcx) as u64) as u32 as u64;
            // 134b6 cmp ecx,0x36
            fl = sub::<32>((rcx as u32 as u64), 0x36u64).1;
            // 134b9 je 0x134d4
            if fl.e() { pc = 0x134d4; continue; }
            pc = 0x134bb;
        }
        0x134bb => {
            // block 134bb
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134bb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 134bd jne 0x134c5
            if fl.ne() { pc = 0x134c5; continue; }
            pc = 0x134bf;
        }
        0x134bf => {
            // block 134bf
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134bf lea rcx,[rdi+0x1]
            rcx = rdi.wrapping_add(0x1);
            // 134c3 jmp 0x134cc
            pc = 0x134cc; continue;
        }
        0x134c5 => {
            // block 134c5
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134c5 mov rcx,QWORD PTR [rsi+0x2c0]
            rcx = m.r64(rsi.wrapping_add(0x2c0)) as u64;
            pc = 0x134cc;
        }
        0x134cc => {
            // block 134cc
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134cc movzx ecx,BYTE PTR [rcx]
            rcx = (m.r8(rcx) as u64) as u32 as u64;
            // 134cf cmp ecx,0x37
            fl = sub::<32>((rcx as u32 as u64), 0x37u64).1;
            // 134d2 jne 0x13510
            if fl.ne() { pc = 0x13510; continue; }
            pc = 0x134d4;
        }
        0x134d4 => {
            // block 134d4
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134d4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 134d6 jne 0x134de
            if fl.ne() { pc = 0x134de; continue; }
            pc = 0x134d8;
        }
        0x134d8 => {
            // block 134d8
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134d8 lea rcx,[rdi+0x1]
            rcx = rdi.wrapping_add(0x1);
            // 134dc jmp 0x134e5
            pc = 0x134e5; continue;
        }
        0x134de => {
            // block 134de
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134de mov rcx,QWORD PTR [rsi+0x2c0]
            rcx = m.r64(rsi.wrapping_add(0x2c0)) as u64;
            pc = 0x134e5;
        }
        0x134e5 => {
            // block 134e5
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134e5 movzx ecx,BYTE PTR [rcx]
            rcx = (m.r8(rcx) as u64) as u32 as u64;
            // 134e8 cmp ecx,0x35
            fl = sub::<32>((rcx as u32 as u64), 0x35u64).1;
            // 134eb je 0x13558
            if fl.e() { pc = 0x13558; continue; }
            pc = 0x134ed;
        }
        0x134ed => {
            // block 134ed
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134ed test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 134ef jne 0x134f7
            if fl.ne() { pc = 0x134f7; continue; }
            pc = 0x134f1;
        }
        0x134f1 => {
            // block 134f1
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134f1 lea rcx,[rdi+0x1]
            rcx = rdi.wrapping_add(0x1);
            // 134f5 jmp 0x134fe
            pc = 0x134fe; continue;
        }
        0x134f7 => {
            // block 134f7
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134f7 mov rcx,QWORD PTR [rsi+0x2c0]
            rcx = m.r64(rsi.wrapping_add(0x2c0)) as u64;
            pc = 0x134fe;
        }
        0x134fe => {
            // block 134fe
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 134fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 134fe movzx ecx,BYTE PTR [rcx]
            rcx = (m.r8(rcx) as u64) as u32 as u64;
            // 13501 cmp ecx,0x36
            fl = sub::<32>((rcx as u32 as u64), 0x36u64).1;
            // 13504 jne 0x13526
            if fl.ne() { pc = 0x13526; continue; }
            pc = 0x13506;
        }
        0x13506 => {
            // block 13506
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13506 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13506 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 13508 jne 0x13516
            if fl.ne() { pc = 0x13516; continue; }
            pc = 0x1350a;
        }
        0x1350a => {
            // block 1350a
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1350a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1350a lea rcx,[rdi+0x1]
            rcx = rdi.wrapping_add(0x1);
            // 1350e jmp 0x1351d
            pc = 0x1351d; continue;
        }
        0x13510 => {
            // block 13510
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13510 add r12d,0x4
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x4u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 13514 jmp 0x13562
            pc = 0x13562; continue;
        }
        0x13516 => {
            // block 13516
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13516 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13516 mov rcx,QWORD PTR [rsi+0x2c0]
            rcx = m.r64(rsi.wrapping_add(0x2c0)) as u64;
            pc = 0x1351d;
        }
        0x1351d => {
            // block 1351d
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1351d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1351d movzx ecx,BYTE PTR [rcx+0x1]
            rcx = (m.r8(rcx.wrapping_add(0x1)) as u64) as u32 as u64;
            // 13521 cmp ecx,0x76
            fl = sub::<32>((rcx as u32 as u64), 0x76u64).1;
            // 13524 je 0x13558
            if fl.e() { pc = 0x13558; continue; }
            pc = 0x13526;
        }
        0x13526 => {
            // block 13526
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13526 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13526 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 13528 jne 0x13530
            if fl.ne() { pc = 0x13530; continue; }
            pc = 0x1352a;
        }
        0x1352a => {
            // block 1352a
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1352a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1352a lea rcx,[rdi+0x1]
            rcx = rdi.wrapping_add(0x1);
            // 1352e jmp 0x13537
            pc = 0x13537; continue;
        }
        0x13530 => {
            // block 13530
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13530 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13530 mov rcx,QWORD PTR [rsi+0x2c0]
            rcx = m.r64(rsi.wrapping_add(0x2c0)) as u64;
            pc = 0x13537;
        }
        0x13537 => {
            // block 13537
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13537 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13537 movzx ecx,BYTE PTR [rcx]
            rcx = (m.r8(rcx) as u64) as u32 as u64;
            // 1353a cmp ecx,0x37
            fl = sub::<32>((rcx as u32 as u64), 0x37u64).1;
            // 1353d jne 0x1355e
            if fl.ne() { pc = 0x1355e; continue; }
            pc = 0x1353f;
        }
        0x1353f => {
            // block 1353f
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1353f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1353f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 13541 jne 0x13548
            if fl.ne() { pc = 0x13548; continue; }
            pc = 0x13543;
        }
        0x13543 => {
            // block 13543
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13543 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13543 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 13546 jmp 0x1354f
            pc = 0x1354f; continue;
        }
        0x13548 => {
            // block 13548
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13548 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13548 mov rdi,QWORD PTR [rsi+0x2c0]
            rdi = m.r64(rsi.wrapping_add(0x2c0)) as u64;
            pc = 0x1354f;
        }
        0x1354f => {
            // block 1354f
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1354f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1354f movzx eax,BYTE PTR [rdi+0x1]
            rax = (m.r8(rdi.wrapping_add(0x1)) as u64) as u32 as u64;
            // 13553 cmp eax,0x6b
            fl = sub::<32>((rax as u32 as u64), 0x6bu64).1;
            // 13556 jne 0x1355e
            if fl.ne() { pc = 0x1355e; continue; }
            pc = 0x13558;
        }
        0x13558 => {
            // block 13558
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13558 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13558 add r12d,0x30
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x30u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 1355c jmp 0x13562
            pc = 0x13562; continue;
        }
        0x1355e => {
            // block 1355e
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1355e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1355e add r12d,0x10
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x10u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x13562;
        }
        0x13562 => {
            // block 13562
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13562 mov eax,DWORD PTR [rsi+0x250]
            rax = (m.r32(rsi.wrapping_add(0x250)) as u64) as u32 as u64;
            // 13568 sub eax,DWORD PTR [rsi+0x248]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rsi.wrapping_add(0x248)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 1356e sub eax,r12d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r12 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 13571 mov DWORD PTR [rdx],eax
            m.w32(rdx, ((rax as u32 as u64)) as u32);
            pc = 0x13573;
        }
        0x13573 => {
            // block 13573
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13573 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13573 mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 1357a lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 13581 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13586 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 1358d mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 13594 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 1359b mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // 135a2 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 135a9 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 135b0 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 135b7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 135bc lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 135c3 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 135c8 mov rsi,QWORD PTR [rip+0x12cc39] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 135cf add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 135d3 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 135da call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 135df lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 135e6 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 135eb mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 135f2 add rsp,0x168
            { let (r, f) = add::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // 135f9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 135fa pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 135fc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 135fe pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 13600 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 13602 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 13603 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x13604 => {
            // block 13604
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13604 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13604 mov rbx,rax
            rbx = rax;
            // 13607 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 1360e mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 13615 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 1361c mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // 13623 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 1362a mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 13631 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 13638 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1363d lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 13644 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13649 jmp 0x13671
            pc = 0x13671; continue;
        }
        0x1364b => {
            // block 1364b
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 1364b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1364b mov rbx,rax
            rbx = rax;
            // 1364e jmp 0x13688
            pc = 0x13688; continue;
        }
        0x13650 => {
            // block 13650
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13650 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13650 mov rbx,rax
            rbx = rax;
            // 13653 jmp 0x13671
            pc = 0x13671; continue;
        }
        0x13655 => {
            // block 13655
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13655 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13655 mov rbx,rax
            rbx = rax;
            // 13658 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1365c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13661 mov rdi,r14
            rdi = r14;
            // 13664 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13669 mov rdi,r12
            rdi = r12;
            // 1366c call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x13671;
        }
        0x13671 => {
            // block 13671
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13671 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13671 mov rsi,QWORD PTR [rip+0x12cb90] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 13678 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 1367c lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 13683 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x13688;
        }
        0x13688 => {
            // block 13688
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13688 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 1368f call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13694 mov rdi,rbx
            rdi = rbx;
            // 13697 call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d590 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d590<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2d590;
    loop {
        match pc {
        0x2d590 => {
            // block 2d590
            #[cfg(feature = "trace")] eprintln!("f_0002d590 pc 2d590 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d590 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d591 mov rbp,rsp
            rbp = rsp;
            // 2d594 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d596 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d597 mov rbx,rdi
            rbx = rdi;
            // 2d59a mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            // 2d59d test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2d5a0 je 0x2d5d0
            if fl.e() { pc = 0x2d5d0; continue; }
            pc = 0x2d5a2;
        }
        0x2d5a2 => {
            // block 2d5a2
            #[cfg(feature = "trace")] eprintln!("f_0002d590 pc 2d5a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d5a2 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2d5a6 cmp rdi,r14
            fl = sub::<64>(rdi, r14).1;
            // 2d5a9 je 0x2d5c4
            if fl.e() { pc = 0x2d5c4; continue; }
            pc = 0x2d5ab;
        }
        0x2d5ab => {
            // block 2d5ab
            #[cfg(feature = "trace")] eprintln!("f_0002d590 pc 2d5ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d5ab add rdi,0xffffffffffffffe8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffe8u64);
              rdi = r;
              fl = f; }
            // 2d5af mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 2d5b3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d5b8 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2d5bc cmp rdi,r14
            fl = sub::<64>(rdi, r14).1;
            // 2d5bf jne 0x2d5ab
            if fl.ne() { pc = 0x2d5ab; continue; }
            pc = 0x2d5c1;
        }
        0x2d5c1 => {
            // block 2d5c1
            #[cfg(feature = "trace")] eprintln!("f_0002d590 pc 2d5c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d5c1 mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            pc = 0x2d5c4;
        }
        0x2d5c4 => {
            // block 2d5c4
            #[cfg(feature = "trace")] eprintln!("f_0002d590 pc 2d5c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d5c4 mov rdi,r14
            rdi = r14;
            // 2d5c7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d5c8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d5ca pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d5cb jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2d5d0 => {
            // block 2d5d0
            #[cfg(feature = "trace")] eprintln!("f_0002d590 pc 2d5d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d5d0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d5d1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d5d3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d5d4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dcfe size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dcfe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2dcfe;
    loop {
        match pc {
        0x2dcfe => {
            // block 2dcfe
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dcfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcfe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dcff mov rbp,rsp
            rbp = rsp;
            // 2dd02 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2dd06 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 2dd0a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2dd0d je 0x2dd41
            if fl.e() { pc = 0x2dd41; continue; }
            pc = 0x2dd0f;
        }
        0x2dd0f => {
            // block 2dd0f
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd0f mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // 2dd11 mov rsi,rdi
            rsi = rdi;
            pc = 0x2dd14;
        }
        0x2dd14 => {
            // block 2dd14
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd14 mov rax,rdx
            rax = rdx;
            pc = 0x2dd17;
        }
        0x2dd17 => {
            // block 2dd17
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd17 cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // 2dd1a jae 0x2dd27
            if fl.ae() { pc = 0x2dd27; continue; }
            pc = 0x2dd1c;
        }
        0x2dd1c => {
            // block 2dd1c
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd1c mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2dd20 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dd23 jne 0x2dd17
            if fl.ne() { pc = 0x2dd17; continue; }
            pc = 0x2dd25;
        }
        0x2dd25 => {
            // block 2dd25
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd25 jmp 0x2dd34
            pc = 0x2dd34; continue;
        }
        0x2dd27 => {
            // block 2dd27
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd27 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 2dd2a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2dd2d mov rsi,rax
            rsi = rax;
            // 2dd30 jne 0x2dd14
            if fl.ne() { pc = 0x2dd14; continue; }
            pc = 0x2dd32;
        }
        0x2dd32 => {
            // block 2dd32
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd32 jmp 0x2dd37
            pc = 0x2dd37; continue;
        }
        0x2dd34 => {
            // block 2dd34
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd34 mov rax,rsi
            rax = rsi;
            pc = 0x2dd37;
        }
        0x2dd37 => {
            // block 2dd37
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd37 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 2dd3a je 0x2dd41
            if fl.e() { pc = 0x2dd41; continue; }
            pc = 0x2dd3c;
        }
        0x2dd3c => {
            // block 2dd3c
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd3c cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // 2dd3f jae 0x2dd44
            if fl.ae() { pc = 0x2dd44; continue; }
            pc = 0x2dd41;
        }
        0x2dd41 => {
            // block 2dd41
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd41 mov rax,rdi
            rax = rdi;
            pc = 0x2dd44;
        }
        0x2dd44 => {
            // block 2dd44
            #[cfg(feature = "trace")] eprintln!("f_0002dcfe pc 2dd44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd44 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dd45 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4c82 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004c82<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4c82;
    loop {
        match pc {
        0x4c82 => {
            // block 4c82
            #[cfg(feature = "trace")] eprintln!("f_00004c82 pc 4c82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c82 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4c83 mov rbp,rsp
            rbp = rsp;
            // 4c86 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4c88 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4c8a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4c8b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4c8c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 4c8f mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 4c92 mov rbx,rdi
            rbx = rdi;
            // 4c95 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4c98 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c9e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4ca0 je 0x4ca6
            if fl.e() { pc = 0x4ca6; continue; }
            pc = 0x4ca2;
        }
        0x4ca2 => {
            // block 4ca2
            #[cfg(feature = "trace")] eprintln!("f_00004c82 pc 4ca2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ca2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4ca4 jmp 0x4cd4
            pc = 0x4cd4; continue;
        }
        0x4ca6 => {
            // block 4ca6
            #[cfg(feature = "trace")] eprintln!("f_00004c82 pc 4ca6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ca6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4ca9 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 4cac lea rcx,[rip+0xfa5ad] # 0xff260
            rcx = m.env.data_va(80429);
            // 4cb3 mov esi,DWORD PTR [rcx+rdx*4]
            rsi = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4cb6 lea rcx,[rip+0xfa5d3] # 0xff290
            rcx = m.env.data_va(80477);
            // 4cbd mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4cc0 lea rdi,[rip+0xfa5f9] # 0xff2c0
            rdi = m.env.data_va(80525);
            // 4cc7 mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4ccb mov rdi,rbx
            rdi = rbx;
            // 4cce mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 4cd1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4cd4;
        }
        0x4cd4 => {
            // block 4cd4
            #[cfg(feature = "trace")] eprintln!("f_00004c82 pc 4cd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4cd4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4cd8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4cd9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4cdb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4cdd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4cde ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a350 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_0001a350<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a350;
    loop {
        match pc {
        0x1a350 => {
            // block 1a350
            #[cfg(feature = "trace")] eprintln!("f_0001a350 pc 1a350 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a350 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a351 mov rbp,rsp
            rbp = rsp;
            // 1a354 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a356 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a357 mov r14,rsi
            r14 = rsi;
            // 1a35a mov rbx,rdi
            rbx = rdi;
            // 1a35d mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1a365 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1a36d mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1a374 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a378 sub rsi,QWORD PTR [r14]
            { let (r, f) = sub::<64>(rsi, m.r64(r14) as u64);
              rsi = r;
              fl = f; }
            // 1a37b sar rsi,0x2
            { let (r, f) = sar::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 1a37f je 0x1a3a7
            if fl.e() { pc = 0x1a3a7; continue; }
            pc = 0x1a381;
        }
        0x1a381 => {
            // block 1a381
            #[cfg(feature = "trace")] eprintln!("f_0001a350 pc 1a381 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a381 mov rdi,rbx
            rdi = rbx;
            // 1a384 call 0xeba44
            { let r = { let t = m.env.stub_va(0x211); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a389 mov rsi,QWORD PTR [r14]
            rsi = m.r64(r14) as u64;
            // 1a38c mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a390 sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // 1a393 mov r14,rdx
            r14 = rdx;
            // 1a396 and r14,0xfffffffffffffffc
            { let r = r14 & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              r14 = r;
              fl = f; }
            // 1a39a mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1a39e call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a3a3 add QWORD PTR [rbx+0x8],r14
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, r14);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            pc = 0x1a3a7;
        }
        0x1a3a7 => {
            // block 1a3a7
            #[cfg(feature = "trace")] eprintln!("f_0001a350 pc 1a3a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a3a7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a3a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a3aa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a3ab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1a3ac => {
            // block 1a3ac
            #[cfg(feature = "trace")] eprintln!("f_0001a350 pc 1a3ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a3ac mov r14,rax
            r14 = rax;
            // 1a3af mov rdi,rbx
            rdi = rbx;
            // 1a3b2 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a3b7 mov rdi,r14
            rdi = r14;
            // 1a3ba call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63dd3 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00063dd3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x63dd3;
    loop {
        match pc {
        0x63dd3 => {
            // block 63dd3
            #[cfg(feature = "trace")] eprintln!("f_00063dd3 pc 63dd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63dd3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63dd4 mov rbp,rsp
            rbp = rsp;
            // 63dd7 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 63dd9 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 63ddb push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 63ddc push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 63ddd mov r14,rdi
            r14 = rdi;
            // 63de0 lea rax,[r14+0x8]
            rax = r14.wrapping_add(0x8);
            // 63de4 mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 63dec mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 63df4 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 63df7 lea r15,[rip+0x9cdb2] # 0x100bb0
            r15 = m.env.data_va(86909);
            // 63dfe xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x63e00;
        }
        0x63e00 => {
            // block 63e00
            #[cfg(feature = "trace")] eprintln!("f_00063dd3 pc 63e00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e00 cmp rbx,0x1d
            fl = sub::<64>(rbx, 0x1du64).1;
            // 63e04 je 0x63e1e
            if fl.e() { pc = 0x63e1e; continue; }
            pc = 0x63e06;
        }
        0x63e06 => {
            // block 63e06
            #[cfg(feature = "trace")] eprintln!("f_00063dd3 pc 63e06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e06 mov rdi,r14
            rdi = r14;
            // 63e09 mov rsi,r15
            rsi = r15;
            // 63e0c call 0xebbac
            { let r = { let t = m.env.stub_va(0x379); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63e11 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 63e14 add r15,0x4
            { let (r, f) = add::<64>(r15, 0x4u64);
              r15 = r;
              fl = f; }
            // 63e18 cmp rbx,0x1e
            fl = sub::<64>(rbx, 0x1eu64).1;
            // 63e1c jb 0x63e00
            if fl.b() { pc = 0x63e00; continue; }
            pc = 0x63e1e;
        }
        0x63e1e => {
            // block 63e1e
            #[cfg(feature = "trace")] eprintln!("f_00063dd3 pc 63e1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e1e mov rax,r14
            rax = r14;
            // 63e21 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 63e25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 63e26 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 63e28 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 63e2a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 63e2b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x63e2c => {
            // block 63e2c
            #[cfg(feature = "trace")] eprintln!("f_00063dd3 pc 63e2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e2c mov rbx,rax
            rbx = rax;
            // 63e2f mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 63e33 mov rdi,r14
            rdi = r14;
            // 63e36 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63e3b mov rdi,rbx
            rdi = rbx;
            // 63e3e call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc36 size=182
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000c36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc36;
    loop {
        match pc {
        0xc36 => {
            // block c36
            #[cfg(feature = "trace")] eprintln!("f_00000c36 pc c36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c37 mov rbp,rsp
            rbp = rsp;
            // c3a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3f sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c43 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // c46 mov rbx,rdi
            rbx = rdi;
            // c49 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c4c call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c52 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // c55 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c58 lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // c5c mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // c60 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // c64 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // c68 lea rcx,[rbp-0x24]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // c6c lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c70 lea r9,[rbp-0x2c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // c74 mov rdi,rbx
            rdi = rbx;
            // c77 call QWORD PTR [rax+0xd10]
            { let r = { let t = m.r64(rax.wrapping_add(0xd10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c7f je 0xcde
            if fl.e() { pc = 0xcde; continue; }
            pc = 0xc81;
        }
        0xc81 => {
            // block c81
            #[cfg(feature = "trace")] eprintln!("f_00000c36 pc c81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // c84 call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c89 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // c8c neg ecx
            { let (r, f) = neg::<32>((rcx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c90 cmovne ecx,r14d
            if fl.ne() { rcx = ((r14 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // c94 add ecx,DWORD PTR [rbp-0x28]
            { let (r, f) = add::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c97 mov DWORD PTR [rbp-0x28],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), ((rcx as u32 as u64)) as u32);
            // c9a mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // c9d cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // c9f jg 0xca8
            if fl.g() { pc = 0xca8; continue; }
            pc = 0xca1;
        }
        0xca1 => {
            // block ca1
            #[cfg(feature = "trace")] eprintln!("f_00000c36 pc ca1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ca1 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // ca4 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // ca6 jge 0xcab
            if fl.ge() { pc = 0xcab; continue; }
            pc = 0xca8;
        }
        0xca8 => {
            // block ca8
            #[cfg(feature = "trace")] eprintln!("f_00000c36 pc ca8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ca8 mov DWORD PTR [rbp-0x28],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), ((rax as u32 as u64)) as u32);
            pc = 0xcab;
        }
        0xcab => {
            // block cab
            #[cfg(feature = "trace")] eprintln!("f_00000c36 pc cab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // cae lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // cb2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // cb4 mov rdi,rbx
            rdi = rbx;
            // cb7 call QWORD PTR [rax+0xd60]
            { let r = { let t = m.r64(rax.wrapping_add(0xd60)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // cbd movzx eax,WORD PTR [rbp-0x34]
            rax = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // cc1 mov esi,DWORD PTR [rbp-0x28]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // cc4 shl esi,0x10
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // cc7 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // cc9 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // ccc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ccf xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // cd1 mov rdi,rbx
            rdi = rbx;
            // cd4 call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // cda mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // cdc jmp 0xce0
            pc = 0xce0; continue;
        }
        0xcde => {
            // block cde
            #[cfg(feature = "trace")] eprintln!("f_00000c36 pc cde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cde xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xce0;
        }
        0xce0 => {
            // block ce0
            #[cfg(feature = "trace")] eprintln!("f_00000c36 pc ce0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ce0 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // ce4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ce5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ce7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ce9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cea ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51ee0 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051ee0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51ee0;
    loop {
        match pc {
        0x51ee0 => {
            // block 51ee0
            #[cfg(feature = "trace")] eprintln!("f_00051ee0 pc 51ee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51ee0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51ee1 mov rbp,rsp
            rbp = rsp;
            // 51ee4 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51ee6 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51ee9 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51eed mov esi,0x27c5
            rsi = (0x27c5u64) as u32 as u64;
            // 51ef2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51ef7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51efa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51efb jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51f1c size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051f1c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51f1c;
    loop {
        match pc {
        0x51f1c => {
            // block 51f1c
            #[cfg(feature = "trace")] eprintln!("f_00051f1c pc 51f1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51f1c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51f1d mov rbp,rsp
            rbp = rsp;
            // 51f20 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51f22 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51f25 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51f29 mov esi,0x27c6
            rsi = (0x27c6u64) as u32 as u64;
            // 51f2e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51f33 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51f36 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51f37 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe605e size=292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e605e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe605e;
    loop {
        match pc {
        0xe605e => {
            // block e605e
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e605e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e605e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e605f mov rbp,rsp
            rbp = rsp;
            // e6062 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e6064 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e6066 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e6068 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e606a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e606b test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e606d mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e606f jne 0xe6074
            if fl.ne() { pc = 0xe6074; continue; }
            pc = 0xe6071;
        }
        0xe6071 => {
            // block e6071
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6071 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6071 mov eax,DWORD PTR [rdi+0x8]
            rax = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe6074;
        }
        0xe6074 => {
            // block e6074
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6074 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6074 mov r10d,DWORD PTR [rbp+0x10]
            r10 = (m.r32(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // e6078 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // e607b je 0xe6086
            if fl.e() { pc = 0xe6086; continue; }
            pc = 0xe607d;
        }
        0xe607d => {
            // block e607d
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e607d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e607d cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // e6080 jne 0xe6126
            if fl.ne() { pc = 0xe6126; continue; }
            pc = 0xe6086;
        }
        0xe6086 => {
            // block e6086
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6086 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e6088 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e608a jne 0xe608f
            if fl.ne() { pc = 0xe608f; continue; }
            pc = 0xe608c;
        }
        0xe608c => {
            // block e608c
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e608c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e608c mov eax,DWORD PTR [rdi+0x8]
            rax = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe608f;
        }
        0xe608f => {
            // block e608f
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e608f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e608f dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e6091 mov r11d,0x1e
            r11 = (0x1eu64) as u32 as u64;
            // e6097 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e609a ja 0xe60a9
            if fl.a() { pc = 0xe60a9; continue; }
            pc = 0xe609c;
        }
        0xe609c => {
            // block e609c
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e609c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e609c cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e609e lea rbx,[rip+0x43c3b] # 0x129ce0
            rbx = m.env.data_va(255149);
            // e60a5 mov r11d,DWORD PTR [rbx+rax*4]
            r11 = (m.r32(rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe60a9;
        }
        0xe60a9 => {
            // block e60a9
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e60a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e60a9 imul r15d,r11d,0x3c
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0x3cu64); r15 = (r) as u32 as u64; fl = f; }
            // e60ad cmp edx,0x8
            fl = sub::<32>((rdx as u32 as u64), 0x8u64).1;
            // e60b0 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // e60b3 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // e60b6 lea r14d,[rax+rax*1+0x2]
            r14 = (rax.wrapping_add(rax).wrapping_add(0x2)) as u32 as u64;
            // e60bb mov r12d,r11d
            r12 = ((r11 as u32 as u64)) as u32 as u64;
            // e60be sub r12d,r14d
            { let (r, f) = sub::<32>((r12 as u32 as u64), (r14 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // e60c1 imul edx,r11d,0x3b
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0x3bu64); rdx = (r) as u32 as u64; fl = f; }
            // e60c5 add edx,r12d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r12 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e60c8 lea ebx,[rdx+rdx*8]
            rbx = (rdx.wrapping_add(rdx.wrapping_mul(8))) as u32 as u64;
            // e60cb add ebx,r15d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r15 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e60ce lea r13d,[rcx+rcx*2]
            r13 = (rcx.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // e60d2 mov ecx,r8d
            rcx = ((r8 as u32 as u64)) as u32 as u64;
            // e60d5 mov eax,0xcccccccd
            rax = (0xcccccccdu64) as u32 as u64;
            // e60da imul rax,rcx
            { let (r, f) = imul::<64>(rax, rcx); rax = r; fl = f; }
            // e60de shr rax,0x23
            { let (r, f) = shr::<64>(rax, 0x23u64, fl); rax = r; fl = f; }
            // e60e2 lea ecx,[rax+r13*2]
            rcx = (rax.wrapping_add(r13.wrapping_mul(2))) as u32 as u64;
            // e60e6 imul ecx,ebx
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rbx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // e60e9 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e60eb lea eax,[rax+rax*4]
            rax = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e60ee sub r8d,eax
            { let (r, f) = sub::<32>((r8 as u32 as u64), (rax as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            // e60f1 je 0xe60ff
            if fl.e() { pc = 0xe60ff; continue; }
            pc = 0xe60f3;
        }
        0xe60f3 => {
            // block e60f3
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e60f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e60f3 lea eax,[r8-0x1]
            rax = (r8.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // e60f7 imul eax,edx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rdx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e60fa add eax,r15d
            { let (r, f) = add::<32>((rax as u32 as u64), (r15 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e60fd add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe60ff;
        }
        0xe60ff => {
            // block e60ff
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e60ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e60ff test r9d,r9d
            fl = logic::<32>((r9 as u32 as u64) & (r9 as u32 as u64));
            // e6102 jle 0xe6160
            if fl.le() { pc = 0xe6160; continue; }
            pc = 0xe6104;
        }
        0xe6104 => {
            // block e6104
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6104 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6104 mov eax,DWORD PTR [rdi+0x58]
            rax = (m.r32(rdi.wrapping_add(0x58)) as u64) as u32 as u64;
            // e6107 mov edx,0xcccccccd
            rdx = (0xcccccccdu64) as u32 as u64;
            // e610c imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e6110 shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // e6114 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e6116 lea edx,[rdx+rdx*4]
            rdx = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e6119 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // e611b je 0xe6157
            if fl.e() { pc = 0xe6157; continue; }
            pc = 0xe611d;
        }
        0xe611d => {
            // block e611d
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e611d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e611d add ecx,r12d
            { let (r, f) = add::<32>((rcx as u32 as u64), (r12 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6120 lea eax,[r9-0x1]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // e6124 jmp 0xe615a
            pc = 0xe615a; continue;
        }
        0xe6126 => {
            // block e6126
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6126 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6126 imul eax,ecx,0x3c
            { let (r, f) = imul::<32>((rcx as u32 as u64), 0x3cu64); rax = (r) as u32 as u64; fl = f; }
            // e6129 add eax,r8d
            { let (r, f) = add::<32>((rax as u32 as u64), (r8 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e612c imul eax,eax,0x3c
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x3cu64); rax = (r) as u32 as u64; fl = f; }
            // e612f test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e6131 jne 0xe6136
            if fl.ne() { pc = 0xe6136; continue; }
            pc = 0xe6133;
        }
        0xe6133 => {
            // block e6133
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6133 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6133 mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe6136;
        }
        0xe6136 => {
            // block e6136
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6136 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6136 add eax,r9d
            { let (r, f) = add::<32>((rax as u32 as u64), (r9 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e6139 dec edx
            { let (r, f) = dec::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e613b mov ecx,0x1e
            rcx = (0x1eu64) as u32 as u64;
            // e6140 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // e6143 ja 0xe6152
            if fl.a() { pc = 0xe6152; continue; }
            pc = 0xe6145;
        }
        0xe6145 => {
            // block e6145
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6145 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6145 movsxd rcx,edx
            rcx = sx::<32>((rdx as u32 as u64)) as u64;
            // e6148 lea rdx,[rip+0x43b91] # 0x129ce0
            rdx = m.env.data_va(255149);
            // e614f mov ecx,DWORD PTR [rdx+rcx*4]
            rcx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe6152;
        }
        0xe6152 => {
            // block e6152
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6152 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6152 imul ecx,eax
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rax as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // e6155 jmp 0xe6172
            pc = 0xe6172; continue;
        }
        0xe6157 => {
            // block e6157
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6157 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6157 mov eax,r9d
            rax = ((r9 as u32 as u64)) as u32 as u64;
            pc = 0xe615a;
        }
        0xe615a => {
            // block e615a
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e615a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e615a imul eax,r11d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r11 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e615e add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe6160;
        }
        0xe6160 => {
            // block e6160
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6160 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6160 test r9d,r9d
            fl = logic::<32>((r9 as u32 as u64) & (r9 as u32 as u64));
            // e6163 jne 0xe6172
            if fl.ne() { pc = 0xe6172; continue; }
            pc = 0xe6165;
        }
        0xe6165 => {
            // block e6165
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6165 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6165 test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // e6168 je 0xe6172
            if fl.e() { pc = 0xe6172; continue; }
            pc = 0xe616a;
        }
        0xe616a => {
            // block e616a
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e616a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e616a cmp r14d,r10d
            fl = sub::<32>((r14 as u32 as u64), (r10 as u32 as u64)).1;
            // e616d ja 0xe6175
            if fl.a() { pc = 0xe6175; continue; }
            pc = 0xe616f;
        }
        0xe616f => {
            // block e616f
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e616f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e616f sub r10d,r14d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r14 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe6172;
        }
        0xe6172 => {
            // block e6172
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6172 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6172 add ecx,r10d
            { let (r, f) = add::<32>((rcx as u32 as u64), (r10 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe6175;
        }
        0xe6175 => {
            // block e6175
            #[cfg(feature = "trace")] eprintln!("f_000e605e pc e6175 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6175 mov DWORD PTR [rsi],ecx
            m.w32(rsi, ((rcx as u32 as u64)) as u32);
            // e6177 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e6178 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e617a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e617c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e617e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e6180 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e6181 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
