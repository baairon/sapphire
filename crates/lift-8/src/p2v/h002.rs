//! Mechanically lifted functions (scripts/lift_run.py), module h002. Each one is checked against the original
//! in the world harness (tests/world_h002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

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
            if fl.b() { pc = 0x10752; continue; }
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
            if fl.ne() { pc = 0x109e6; continue; }
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
            if fl.be() { pc = 0x10859; continue; }
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
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            if fl.ae() { pc = 0x10794; continue; }
            pc = 0x1076c;
        }
        0x1076c => {
            // block 1076c
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 1076c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1076c mov edi,0x1000
            rdi = (0x1000u64) as u32 as u64;
            // 10771 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10776 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 10779 je 0x10908
            if fl.e() { pc = 0x10908; continue; }
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
            { let r = { let t = m.env.stub_va(0x13); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107dc mov rbx,rax
            rbx = rax;
            // 107df mov QWORD PTR [rbp-0x68],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rbx) as u64);
            // 107e3 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 107e7 lea rsi,[rbp-0x68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 107eb call 0xeb858
            { let r = { let t = m.env.stub_va(0x25); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            if fl.e() { pc = 0x10826; continue; }
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
            { let r = { let t = m.env.stub_va(0x1f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            if fl.e() { pc = 0x109a8; continue; }
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
            if fl.ne() { pc = 0x108c7; continue; }
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
            { let r = { let t = m.env.stub_va(0xd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            if fl.ne() { pc = 0x10adf; continue; }
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
            if fl.be() { pc = 0x109fd; continue; }
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
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            if fl.e() { pc = 0x10aa1; continue; }
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
            if fl.ne() { pc = 0x10a63; continue; }
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
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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

/// parity: P2v @0x17bf1 size=1119
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017bf1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x17bf1;
    loop {
        match pc {
        0x17bf1 => {
            // block 17bf1
            #[cfg(feature = "trace")] eprintln!("f_00017bf1 pc 17bf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17bf1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17bf2 mov rbp,rsp
            rbp = rsp;
            // 17bf5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 17bf7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 17bf9 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 17bfb push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 17bfc sub rsp,0x40
            { let (r, f) = sub::<64>(rsp, 0x40u64);
              rsp = r;
              fl = f; }
            // 17c00 mov r15,rsi
            r15 = rsi;
            // 17c03 mov r14,rdi
            r14 = rdi;
            // 17c06 lea rsi,[rip+0x1126b1] # 0x12a2be
            rsi = m.env.data_va(256651);
            // 17c0d mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 17c12 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17c17 lea rsi,[r15+0x18]
            rsi = r15.wrapping_add(0x18);
            // 17c1b mov rdi,rax
            rdi = rax;
            // 17c1e call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17c23 lea rsi,[rip+0x11245e] # 0x12a088
            rsi = m.env.data_va(256085);
            // 17c2a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 17c2f mov rdi,rax
            rdi = rax;
            // 17c32 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17c37 mov rbx,rax
            rbx = rax;
            // 17c3a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17c3d mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17c41 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 17c44 lea r12,[rbp-0x58]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 17c48 mov rdi,r12
            rdi = r12;
            // 17c4b call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17c50 mov rsi,QWORD PTR [rip+0x128539] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 17c57 mov rdi,r12
            rdi = r12;
            // 17c5a call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17c5f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17c62 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 17c66 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 17c6b mov rdi,rax
            rdi = rax;
            // 17c6e call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17c70 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17c73 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 17c77 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17c7c movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 17c80 mov rdi,rbx
            rdi = rbx;
            // 17c83 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17c88 mov rdi,rbx
            rdi = rbx;
            // 17c8b call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17c90 lea rsi,[rip+0x112639] # 0x12a2d0
            rsi = m.env.data_va(256669);
            // 17c97 mov edx,0x21
            rdx = (0x21u64) as u32 as u64;
            // 17c9c mov rdi,rbx
            rdi = rbx;
            // 17c9f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17ca4 mov esi,DWORD PTR [r15+0x4]
            rsi = (m.r32(r15.wrapping_add(0x4)) as u64) as u32 as u64;
            // 17ca8 mov rdi,rax
            rdi = rax;
            // 17cab call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17cb0 mov rbx,rax
            rbx = rax;
            // 17cb3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17cb6 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17cba add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 17cbd lea r12,[rbp-0x50]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 17cc1 mov rdi,r12
            rdi = r12;
            // 17cc4 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17cc9 mov rsi,QWORD PTR [rip+0x1284c0] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 17cd0 mov rdi,r12
            rdi = r12;
            // 17cd3 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17cd8 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17cdb mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 17cdf mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 17ce4 mov rdi,rax
            rdi = rax;
            // 17ce7 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17ce9 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17cec lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 17cf0 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17cf5 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 17cf9 mov rdi,rbx
            rdi = rbx;
            // 17cfc call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17d01 mov rdi,rbx
            rdi = rbx;
            // 17d04 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17d09 lea rsi,[rip+0x1125e2] # 0x12a2f2
            rsi = m.env.data_va(256703);
            // 17d10 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 17d15 mov rdi,rbx
            rdi = rbx;
            // 17d18 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17d1d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17d20 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17d24 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 17d28 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17d2b or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17d2e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 17d32 mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 17d35 mov rdi,rax
            rdi = rax;
            // 17d38 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17d3d mov rbx,rax
            rbx = rax;
            // 17d40 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17d43 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17d47 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 17d4b and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 17d4e or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 17d51 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 17d55 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17d58 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17d5c add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 17d5f lea r12,[rbp-0x48]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 17d63 mov rdi,r12
            rdi = r12;
            // 17d66 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17d6b mov rsi,QWORD PTR [rip+0x12841e] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 17d72 mov rdi,r12
            rdi = r12;
            // 17d75 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17d7a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17d7d mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 17d81 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 17d86 mov rdi,rax
            rdi = rax;
            // 17d89 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17d8b mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17d8e lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 17d92 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17d97 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 17d9b mov rdi,rbx
            rdi = rbx;
            // 17d9e call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17da3 mov rdi,rbx
            rdi = rbx;
            // 17da6 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17dab lea rsi,[rip+0x112564] # 0x12a316
            rsi = m.env.data_va(256739);
            // 17db2 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 17db7 mov rdi,rbx
            rdi = rbx;
            // 17dba call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17dbf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17dc2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17dc6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 17dca and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17dcd or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17dd0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 17dd4 mov rsi,QWORD PTR [r15+0x10]
            rsi = m.r64(r15.wrapping_add(0x10)) as u64;
            // 17dd8 mov rdi,rax
            rdi = rax;
            // 17ddb call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17de0 mov rbx,rax
            rbx = rax;
            // 17de3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17de6 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17dea mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 17dee and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 17df1 or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 17df4 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 17df8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17dfb mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17dff add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 17e02 lea r12,[rbp-0x40]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 17e06 mov rdi,r12
            rdi = r12;
            // 17e09 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e0e mov rsi,QWORD PTR [rip+0x12837b] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 17e15 mov rdi,r12
            rdi = r12;
            // 17e18 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e1d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17e20 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 17e24 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 17e29 mov rdi,rax
            rdi = rax;
            // 17e2c call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e2e mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17e31 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 17e35 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e3a movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 17e3e mov rdi,rbx
            rdi = rbx;
            // 17e41 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e46 mov rdi,rbx
            rdi = rbx;
            // 17e49 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e4e lea rsi,[rip+0x1124e5] # 0x12a33a
            rsi = m.env.data_va(256775);
            // 17e55 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 17e5a mov rdi,rbx
            rdi = rbx;
            // 17e5d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e62 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17e65 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17e69 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 17e6d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17e70 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17e73 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 17e77 mov esi,DWORD PTR [r15+0x8]
            rsi = (m.r32(r15.wrapping_add(0x8)) as u64) as u32 as u64;
            // 17e7b mov rdi,rax
            rdi = rax;
            // 17e7e call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e83 mov rbx,rax
            rbx = rax;
            // 17e86 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17e89 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17e8d mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 17e91 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 17e94 or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 17e97 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 17e9b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17e9e mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17ea2 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 17ea5 lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 17ea9 mov rdi,r12
            rdi = r12;
            // 17eac call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17eb1 mov rsi,QWORD PTR [rip+0x1282d8] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 17eb8 mov rdi,r12
            rdi = r12;
            // 17ebb call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17ec0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17ec3 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 17ec7 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 17ecc mov rdi,rax
            rdi = rax;
            // 17ecf call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17ed1 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17ed4 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 17ed8 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17edd movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 17ee1 mov rdi,rbx
            rdi = rbx;
            // 17ee4 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17ee9 mov rdi,rbx
            rdi = rbx;
            // 17eec call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17ef1 lea rsi,[rip+0x112466] # 0x12a35e
            rsi = m.env.data_va(256811);
            // 17ef8 mov edx,0x21
            rdx = (0x21u64) as u32 as u64;
            // 17efd mov rdi,rbx
            rdi = rbx;
            // 17f00 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f05 movzx esi,WORD PTR [r15+0x30]
            rsi = (m.r16(r15.wrapping_add(0x30)) as u64) as u32 as u64;
            // 17f0a mov rdi,rax
            rdi = rax;
            // 17f0d call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f12 mov rbx,rax
            rbx = rax;
            // 17f15 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17f18 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17f1c add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 17f1f lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 17f23 mov rdi,r12
            rdi = r12;
            // 17f26 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f2b mov rsi,QWORD PTR [rip+0x12825e] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 17f32 mov rdi,r12
            rdi = r12;
            // 17f35 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f3a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17f3d mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 17f41 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 17f46 mov rdi,rax
            rdi = rax;
            // 17f49 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f4b mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17f4e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 17f52 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f57 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 17f5b mov rdi,rbx
            rdi = rbx;
            // 17f5e call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f63 mov rdi,rbx
            rdi = rbx;
            // 17f66 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f6b lea rsi,[rip+0x11240e] # 0x12a380
            rsi = m.env.data_va(256845);
            // 17f72 mov edx,0x21
            rdx = (0x21u64) as u32 as u64;
            // 17f77 mov rdi,rbx
            rdi = rbx;
            // 17f7a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f7f movzx esi,WORD PTR [r15+0x32]
            rsi = (m.r16(r15.wrapping_add(0x32)) as u64) as u32 as u64;
            // 17f84 mov rdi,rax
            rdi = rax;
            // 17f87 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17f8c mov rbx,rax
            rbx = rax;
            // 17f8f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17f92 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 17f96 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 17f99 lea r15,[rbp-0x28]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17f9d mov rdi,r15
            rdi = r15;
            // 17fa0 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17fa5 mov rsi,QWORD PTR [rip+0x1281e4] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 17fac mov rdi,r15
            rdi = r15;
            // 17faf call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17fb4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 17fb7 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 17fbb mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 17fc0 mov rdi,rax
            rdi = rax;
            // 17fc3 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17fc5 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17fc8 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17fcc call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17fd1 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // 17fd5 mov rdi,rbx
            rdi = rbx;
            // 17fd8 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17fdd mov rdi,rbx
            rdi = rbx;
            // 17fe0 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17fe5 lea rsi,[rip+0x120cdc] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 17fec xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17fee mov rdi,rbx
            rdi = rbx;
            // 17ff1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17ff6 mov rax,r14
            rax = r14;
            // 17ff9 add rsp,0x40
            { let (r, f) = add::<64>(rsp, 0x40u64);
              rsp = r;
              fl = f; }
            // 17ffd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 17ffe pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18000 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18002 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18004 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18005 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x18006 => {
            // block 18006
            #[cfg(feature = "trace")] eprintln!("f_00017bf1 pc 18006 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18006 mov rbx,rax
            rbx = rax;
            // 18009 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 1800d jmp 0x18043
            pc = 0x18043; continue;
        }
        0x1800f => {
            // block 1800f
            #[cfg(feature = "trace")] eprintln!("f_00017bf1 pc 1800f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1800f mov rbx,rax
            rbx = rax;
            // 18012 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 18016 jmp 0x18043
            pc = 0x18043; continue;
        }
        0x18018 => {
            // block 18018
            #[cfg(feature = "trace")] eprintln!("f_00017bf1 pc 18018 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18018 mov rbx,rax
            rbx = rax;
            // 1801b lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 1801f jmp 0x18043
            pc = 0x18043; continue;
        }
        0x18021 => {
            // block 18021
            #[cfg(feature = "trace")] eprintln!("f_00017bf1 pc 18021 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18021 mov rbx,rax
            rbx = rax;
            // 18024 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18028 jmp 0x18043
            pc = 0x18043; continue;
        }
        0x1802a => {
            // block 1802a
            #[cfg(feature = "trace")] eprintln!("f_00017bf1 pc 1802a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1802a mov rbx,rax
            rbx = rax;
            // 1802d lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18031 jmp 0x18043
            pc = 0x18043; continue;
        }
        0x18033 => {
            // block 18033
            #[cfg(feature = "trace")] eprintln!("f_00017bf1 pc 18033 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18033 mov rbx,rax
            rbx = rax;
            // 18036 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 1803a jmp 0x18043
            pc = 0x18043; continue;
        }
        0x1803c => {
            // block 1803c
            #[cfg(feature = "trace")] eprintln!("f_00017bf1 pc 1803c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1803c mov rbx,rax
            rbx = rax;
            // 1803f lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            pc = 0x18043;
        }
        0x18043 => {
            // block 18043
            #[cfg(feature = "trace")] eprintln!("f_00017bf1 pc 18043 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18043 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18048 mov rdi,rbx
            rdi = rbx;
            // 1804b call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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

/// parity: P2v @0x26b9e size=1023
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00026b9e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x26b9e;
    loop {
        match pc {
        0x26b9e => {
            // block 26b9e
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26b9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26b9e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 26b9f mov rbp,rsp
            rbp = rsp;
            // 26ba2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 26ba4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 26ba6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 26ba8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 26baa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 26bab sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 26bb2 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 26bb5 mov r13,rsi
            r13 = rsi;
            // 26bb8 mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 26bbf lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 26bc6 lea r15,[rbp-0x140]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 26bcd mov rax,QWORD PTR [rip+0x1195fc] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 26bd4 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 26bd8 mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // 26bdf add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 26be3 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 26bea mov rsi,r15
            rsi = r15;
            // 26bed call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26bf2 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 26bfa mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // 26c01 mov rbx,QWORD PTR [rip+0x1197b8] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // 26c08 lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 26c0c mov QWORD PTR [rbp-0x148],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r14) as u64);
            // 26c13 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // 26c17 mov QWORD PTR [rbp-0xd8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rbx) as u64);
            // 26c1e mov rdi,r15
            rdi = r15;
            // 26c21 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26c26 mov QWORD PTR [rbp-0x160],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r14) as u64);
            // 26c2d mov QWORD PTR [rbp-0x158],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rbx) as u64);
            // 26c34 mov rax,QWORD PTR [rip+0x11976d] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 26c3b add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 26c3f mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 26c46 mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            // 26c4d mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // 26c58 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 26c63 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 26c6e mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 26c79 mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // 26c83 xorps xmm0,xmm0
            x0 = 0;
            // 26c86 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 26c8a mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 26c92 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 26c96 mov rdi,r15
            rdi = r15;
            // 26c99 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26c9e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 26ca2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26ca7 lea rsi,[rip+0x1037c7] # 0x12a475
            rsi = m.env.data_va(257090);
            // 26cae lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 26cb5 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 26cba call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26cbf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26cc2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26cc6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26cca and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26ccd or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26cd0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 26cd4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26cd7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26cdb or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // 26ce3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26ce6 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26cea mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 26cf3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26cf6 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26cfa mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 26d05 mov rsi,QWORD PTR [r13+0x0]
            rsi = m.r64(r13) as u64;
            // 26d09 mov rdi,rax
            rdi = rax;
            // 26d0c call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26d11 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26d14 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26d18 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26d1c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26d1f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26d22 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 26d26 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26d29 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26d2d mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // 26d38 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26d3b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26d3f and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // 26d47 lea rsi,[rip+0x1030da] # 0x129e28
            rsi = m.env.data_va(255477);
            // 26d4e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 26d53 mov rdi,rax
            rdi = rax;
            // 26d56 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26d5b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26d5e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26d62 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26d66 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26d69 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26d6c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 26d70 mov esi,DWORD PTR [r13+0x8]
            rsi = (m.r32(r13.wrapping_add(0x8)) as u64) as u32 as u64;
            // 26d74 mov rdi,rax
            rdi = rax;
            // 26d77 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26d7c lea rsi,[rip+0x1036f5] # 0x12a478
            rsi = m.env.data_va(257093);
            // 26d83 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 26d88 mov rdi,rax
            rdi = rax;
            // 26d8b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26d90 test r12w,r12w
            fl = logic::<16>((r12 as u16 as u64) & (r12 as u16 as u64));
            // 26d94 je 0x26e8e
            if fl.e() { pc = 0x26e8e; continue; }
            pc = 0x26d9a;
        }
        0x26d9a => {
            // block 26d9a
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26d9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26d9a lea rsi,[rip+0x103087] # 0x129e28
            rsi = m.env.data_va(255477);
            // 26da1 lea r14,[rbp-0x148]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 26da8 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 26dad mov rdi,r14
            rdi = r14;
            // 26db0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26db5 movzx eax,r12w
            rax = ((r12 as u16 as u64)) as u32 as u64;
            // 26db9 cmp eax,0x40
            fl = sub::<32>((rax as u32 as u64), 0x40u64).1;
            // 26dbc mov ax,0x40
            rax = (rax & !0xffff) | ((0x40u64) as u16 as u64);
            // 26dc0 cmovbe ax,r12w
            if fl.be() { rax = (rax & !0xffff) | (((r12 as u16 as u64)) as u16 as u64); }
            // 26dc5 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 26dc8 mov ecx,DWORD PTR [r13+0x8]
            rcx = (m.r32(r13.wrapping_add(0x8)) as u64) as u32 as u64;
            // 26dcc cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // 26dce cmova ax,cx
            if fl.a() { rax = (rax & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            // 26dd2 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 26dd5 je 0x26e8e
            if fl.e() { pc = 0x26e8e; continue; }
            pc = 0x26ddb;
        }
        0x26ddb => {
            // block 26ddb
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26ddb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26ddb mov r12,QWORD PTR [r13+0x0]
            r12 = m.r64(r13) as u64;
            // 26ddf movzx ebx,ax
            rbx = ((rax as u16 as u64)) as u32 as u64;
            // 26de2 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x26de5;
        }
        0x26de5 => {
            // block 26de5
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26de5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26de5 mov rax,QWORD PTR [rbp-0x148]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 26dec mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26df0 mov ecx,DWORD PTR [rax+r14*1+0x8]
            rcx = (m.r32(rax.wrapping_add(r14).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26df5 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 26df8 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 26dfb mov DWORD PTR [rax+r14*1+0x8],ecx
            m.w32(rax.wrapping_add(r14).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 26e00 mov rax,QWORD PTR [rbp-0x148]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 26e07 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26e0b or DWORD PTR [rax+r14*1+0x8],0x4000
            { let r = m.r32(rax.wrapping_add(r14).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rax.wrapping_add(r14).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // 26e14 mov rax,QWORD PTR [rbp-0x148]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 26e1b mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26e1f mov QWORD PTR [rax+r14*1+0x18],0x2
            m.w64(rax.wrapping_add(r14).wrapping_add(0x18), (0x2u64) as u64);
            // 26e28 mov rax,QWORD PTR [rbp-0x148]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 26e2f mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26e33 mov DWORD PTR [rax+r14*1+0x90],0x30
            m.w32(rax.wrapping_add(r14).wrapping_add(0x90), (0x30u64) as u32);
            // 26e3f movzx esi,BYTE PTR [r12+r13*1]
            rsi = (m.r8(r12.wrapping_add(r13)) as u64) as u32 as u64;
            // 26e44 mov rdi,r14
            rdi = r14;
            // 26e47 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26e4c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26e4f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26e53 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26e57 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26e5a or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26e5d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 26e61 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26e64 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26e68 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // 26e73 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26e76 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26e7a and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // 26e82 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // 26e85 cmp r13,rbx
            fl = sub::<64>(r13, rbx).1;
            // 26e88 jb 0x26de5
            if fl.b() { pc = 0x26de5; continue; }
            pc = 0x26e8e;
        }
        0x26e8e => {
            // block 26e8e
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26e8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26e8e mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 26e95 mov rsi,r15
            rsi = r15;
            // 26e98 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26e9d mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 26ea4 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 26eab mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 26eb2 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 26eb9 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 26ec0 mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            // 26ec7 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 26ece call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26ed3 mov rdi,r15
            rdi = r15;
            // 26ed6 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26edb mov rsi,QWORD PTR [rip+0x119326] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 26ee2 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 26ee6 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 26eed call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26ef2 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 26ef9 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26efe mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 26f05 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 26f0c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 26f0d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 26f0f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 26f11 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 26f13 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 26f15 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 26f16 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x26f17 => {
            // block 26f17
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26f17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26f17 jmp 0x26f19
            pc = 0x26f19; continue;
        }
        0x26f19 => {
            // block 26f19
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26f19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26f19 mov r14,rax
            r14 = rax;
            // 26f1c mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 26f23 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 26f2a mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 26f31 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 26f38 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 26f3f mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            pc = 0x26f46;
        }
        0x26f46 => {
            // block 26f46
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26f46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26f46 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 26f4d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26f52 mov rdi,r15
            rdi = r15;
            // 26f55 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26f5a;
        }
        0x26f5a => {
            // block 26f5a
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26f5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26f5a mov rsi,QWORD PTR [rip+0x1192a7] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 26f61 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 26f65 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 26f6c call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26f71;
        }
        0x26f71 => {
            // block 26f71
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26f71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26f71 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 26f78 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26f7d mov rdi,r14
            rdi = r14;
            // 26f80 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26f85 mov r14,rax
            r14 = rax;
            // 26f88 jmp 0x26f71
            pc = 0x26f71; continue;
        }
        0x26f8a => {
            // block 26f8a
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26f8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26f8a mov r14,rax
            r14 = rax;
            // 26f8d jmp 0x26f5a
            pc = 0x26f5a; continue;
        }
        0x26f8f => {
            // block 26f8f
            #[cfg(feature = "trace")] eprintln!("f_00026b9e pc 26f8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26f8f mov r14,rax
            r14 = rax;
            // 26f92 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 26f96 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26f9b jmp 0x26f46
            pc = 0x26f46; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdddb0 size=1000
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
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
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x5e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x32d57); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xddeaa; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xddf0a; continue; }
            pc = 0xddebf;
        }
        0xddebf => {
            // block ddebf
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddebf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddebf cmp BYTE PTR [r12+0x279],0x0
            fl = sub::<8>(m.r8(r12.wrapping_add(0x279)) as u64, 0x0u64).1;
            // ddec8 je 0xddee5
            if fl.e() { pc = 0xddee5; continue; }
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
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x5932c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddf22 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // ddf24 je 0xddf80
            if fl.e() { pc = 0xddf80; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddf53 mov edx,DWORD PTR [rbp-0xa088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // ddf59 shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ddf5c mov rdi,r15
            rdi = r15;
            // ddf5f mov rsi,r13
            rsi = r13;
            // ddf62 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddf67 mov eax,DWORD PTR [rbp-0xa088]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // ddf6d lea r15,[r15+rax*4]
            r15 = r15.wrapping_add(rax.wrapping_mul(4));
            // ddf71 dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // ddf73 jne 0xddf3a
            if fl.ne() { pc = 0xddf3a; continue; }
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
            if fl.ae() { pc = 0xde084; continue; }
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
            if fl.e() { pc = 0xde03f; continue; }
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
            if fl.ne() { pc = 0xddff2; continue; }
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
            if fl.ne() { pc = 0xde01a; continue; }
            pc = 0xddfea;
        }
        0xddfea => {
            // block ddfea
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddfea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddfea test dil,0x2
            fl = logic::<8>((rdi as u8 as u64) & 0x2u64);
            // ddfee jne 0xde02c
            if fl.ne() { pc = 0xde02c; continue; }
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
            if fl.e() { pc = 0xde004; continue; }
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
            if fl.ne() { pc = 0xde02c; continue; }
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
            if fl.b() { pc = 0xddfd5; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de056 mov edx,DWORD PTR [rbp-0xa088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de05c shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // de05f mov rdi,r15
            rdi = r15;
            // de062 mov rsi,rbx
            rsi = rbx;
            // de065 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de06a mov eax,DWORD PTR [rbp-0xa088]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de070 lea r15,[r15+rax*4]
            r15 = r15.wrapping_add(rax.wrapping_mul(4));
            // de074 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // de077 cmp r13d,DWORD PTR [rbp-0xa090]
            fl = sub::<32>((r13 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f70)) as u64).1;
            // de07e jne 0xddf9d
            if fl.ne() { pc = 0xddf9d; continue; }
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
            if fl.e() { pc = 0xde10d; continue; }
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
            if fl.e() { pc = 0xde10d; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de0b8 mov r14d,DWORD PTR [rbp-0xa08c]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f74)) as u64) as u32 as u64;
            // de0bf mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de0c3 mov rdi,r12
            rdi = r12;
            // de0c6 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de0cc cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // de0cf jbe 0xde0df
            if fl.be() { pc = 0xde0df; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de105 mov rdi,rbx
            rdi = rbx;
            // de108 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de15c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de160 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // de168 mov rdi,r12
            rdi = r12;
            // de16b call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de171 mov rax,QWORD PTR [rip+0x62260] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // de178 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // de17b cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // de17f jne 0xde193
            if fl.ne() { pc = 0xde193; continue; }
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
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb232 size=982
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eb232<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xeb232;
    loop {
        match pc {
        0xeb232 => {
            // block eb232
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb232 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb232 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb233 mov rbp,rsp
            rbp = rsp;
            // eb236 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb238 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb23a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // eb23c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eb23e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb23f sub rsp,0x308
            { let (r, f) = sub::<64>(rsp, 0x308u64);
              rsp = r;
              fl = f; }
            // eb246 mov r13,rsi
            r13 = rsi;
            // eb249 mov rbx,rdi
            rbx = rdi;
            // eb24c lea r14,[rbp-0x2b0]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb253 mov rdi,r14
            rdi = r14;
            // eb256 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb25b mov edi,DWORD PTR [rbx+0x5c]
            rdi = (m.r32(rbx.wrapping_add(0x5c)) as u64) as u32 as u64;
            // eb25e mov rsi,r14
            rsi = r14;
            // eb261 call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb266 mov ebx,0xffffffff
            rbx = (0xffffffffu64) as u32 as u64;
            // eb26b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb26d je 0xeb5ca
            if fl.e() { pc = 0xeb5ca; continue; }
            pc = 0xeb273;
        }
        0xeb273 => {
            // block eb273
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb273 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb273 mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // eb27a mov rax,QWORD PTR [rax+0x3e8]
            rax = m.r64(rax.wrapping_add(0x3e8)) as u64;
            // eb281 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb288 lea rsi,[rbp-0x2b4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd4c);
            // eb28f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eb291 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb293 mov esi,DWORD PTR [rbp-0x2b4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // eb299 lea rdi,[rbp-0x2f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd10);
            // eb2a0 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // eb2a5 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eb2a7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // eb2aa xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // eb2ad call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb2b2 mov eax,DWORD PTR [rbp-0x2f0]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd10)) as u64) as u32 as u64;
            // eb2b8 mov r15d,DWORD PTR [rbp-0x2ec]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd14)) as u64) as u32 as u64;
            // eb2bf mov r12d,DWORD PTR [rbp-0x2e8]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64) as u32 as u64;
            // eb2c6 shl r12d,0x2
            { let (r, f) = shl::<32>((r12 as u32 as u64), 0x2u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // eb2ca xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // eb2cc xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // eb2cf test r13,r13
            fl = logic::<64>(r13 & r13);
            // eb2d2 je 0xeb399
            if fl.e() { pc = 0xeb399; continue; }
            pc = 0xeb2d8;
        }
        0xeb2d8 => {
            // block eb2d8
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb2d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb2d8 mov QWORD PTR [rbp-0x320],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce0), (rax) as u64);
            // eb2df mov DWORD PTR [rbp-0x324],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc), ((r12 as u32 as u64)) as u32);
            // eb2e6 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb2ea mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // eb2ee mov rdi,r13
            rdi = r13;
            // eb2f1 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb2f3 mov QWORD PTR [rbp-0x310],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), (rax) as u64);
            // eb2fa mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb2fe mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // eb302 mov rdi,r13
            rdi = r13;
            // eb305 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb307 mov DWORD PTR [rbp-0x2f8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd08), ((rax as u32 as u64)) as u32);
            // eb30d mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb311 mov rax,QWORD PTR [rax+0x40]
            rax = m.r64(rax.wrapping_add(0x40)) as u64;
            // eb315 mov rdi,r13
            rdi = r13;
            // eb318 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb31a mov DWORD PTR [rbp-0x300],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd00), ((rax as u32 as u64)) as u32);
            // eb320 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb324 mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // eb328 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eb32a mov rdi,r13
            rdi = r13;
            // eb32d call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb32f mov rbx,rax
            rbx = rax;
            // eb332 mov rax,QWORD PTR [rbp-0x320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // eb339 cmp DWORD PTR [rbp-0x2f8],eax
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd08)) as u64, (rax as u32 as u64)).1;
            // eb33f setne r12b
            r12 = (r12 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // eb343 mov rax,QWORD PTR [rbp-0x310]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0)) as u64;
            // eb34a cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // eb34d setne r14b
            r14 = (r14 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // eb351 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb355 mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // eb359 mov rdi,r13
            rdi = r13;
            // eb35c call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb35e or r14b,r12b
            { let r = (r14 as u8 as u64) | (r12 as u8 as u64); let f = logic::<8>(r);
              r14 = (r14 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // eb361 mov cl,0x1
            rcx = (rcx & !0xff) | ((0x1u64) as u8 as u64);
            // eb363 mov QWORD PTR [rbp-0x318],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rcx) as u64);
            // eb36a cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // eb36d je 0xeb389
            if fl.e() { pc = 0xeb389; continue; }
            pc = 0xeb36f;
        }
        0xeb36f => {
            // block eb36f
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb36f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb36f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb373 mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // eb377 mov rdi,r13
            rdi = r13;
            // eb37a call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb37c cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // eb37f sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // eb382 mov QWORD PTR [rbp-0x318],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rax) as u64);
            pc = 0xeb389;
        }
        0xeb389 => {
            // block eb389
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb389 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb389 mov r12d,DWORD PTR [rbp-0x324]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc)) as u64) as u32 as u64;
            // eb390 mov rcx,QWORD PTR [rbp-0x310]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0)) as u64;
            // eb397 jmp 0xeb3bb
            pc = 0xeb3bb; continue;
        }
        0xeb399 => {
            // block eb399
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb399 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb399 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // eb39c mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // eb39e mov DWORD PTR [rbp-0x2f8],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd08), ((rdx as u32 as u64)) as u32);
            // eb3a4 mov QWORD PTR [rbp-0x320],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce0), (rax) as u64);
            // eb3ab mov DWORD PTR [rbp-0x300],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd00), ((r12 as u32 as u64)) as u32);
            // eb3b2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eb3b4 mov QWORD PTR [rbp-0x318],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rax) as u64);
            pc = 0xeb3bb;
        }
        0xeb3bb => {
            // block eb3bb
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3bb mov QWORD PTR [rbp-0x310],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), (rcx) as u64);
            // eb3c2 lea r15d,[rcx*4+0x0]
            r15 = (rcx.wrapping_mul(4)) as u32 as u64;
            // eb3ca test r13,r13
            fl = logic::<64>(r13 & r13);
            // eb3cd sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // eb3d0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eb3d2 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // eb3d5 jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3d7;
        }
        0xeb3d7 => {
            // block eb3d7
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3d7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // eb3d9 jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3db;
        }
        0xeb3db => {
            // block eb3db
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3db mov rcx,QWORD PTR [rbp-0x318]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce8)) as u64;
            // eb3e2 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // eb3e4 jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3e6;
        }
        0xeb3e6 => {
            // block eb3e6
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3e6 cmp DWORD PTR [rbp-0x300],r12d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64, (r12 as u32 as u64)).1;
            // eb3ed jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3ef;
        }
        0xeb3ef => {
            // block eb3ef
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3ef cmp DWORD PTR [rbp-0x300],r15d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64, (r15 as u32 as u64)).1;
            // eb3f6 jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3f8;
        }
        0xeb3f8 => {
            // block eb3f8
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3f8 cmp DWORD PTR [rbp-0x300],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64, 0x0u64).1;
            // eb3ff jns 0xeb413
            if fl.ns() { pc = 0xeb413; continue; }
            pc = 0xeb401;
        }
        0xeb401 => {
            // block eb401
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb401 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb401 mov rax,QWORD PTR [rbp-0x320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // eb408 movsxd rdi,eax
            rdi = sx::<32>((rax as u32 as u64)) as u64;
            // eb40b movsxd rsi,r12d
            rsi = sx::<32>((r12 as u32 as u64)) as u64;
            // eb40e call 0xec1ac
            { let r = { let t = m.env.stub_va(0x979); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb413;
        }
        0xeb413 => {
            // block eb413
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb413 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb413 mov DWORD PTR [rbp-0x324],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc), ((r12 as u32 as u64)) as u32);
            // eb41a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // eb41d mov r12,rax
            r12 = rax;
            // eb420 cmove rbx,r12
            if fl.e() { rbx = r12; }
            // eb424 test r13,r13
            fl = logic::<64>(r13 & r13);
            // eb427 je 0xeb436
            if fl.e() { pc = 0xeb436; continue; }
            pc = 0xeb429;
        }
        0xeb429 => {
            // block eb429
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb429 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb429 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb42d mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // eb431 mov rdi,r13
            rdi = r13;
            // eb434 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb436;
        }
        0xeb436 => {
            // block eb436
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb436 test r12,r12
            fl = logic::<64>(r12 & r12);
            // eb439 je 0xeb579
            if fl.e() { pc = 0xeb579; continue; }
            pc = 0xeb43f;
        }
        0xeb43f => {
            // block eb43f
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb43f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb43f cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // eb442 je 0xeb579
            if fl.e() { pc = 0xeb579; continue; }
            pc = 0xeb448;
        }
        0xeb448 => {
            // block eb448
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb448 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb448 mov rax,QWORD PTR [rbp-0x320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // eb44f lea eax,[rax-0x1]
            rax = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // eb452 mov ecx,DWORD PTR [rbp-0x2f8]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd08)) as u64) as u32 as u64;
            // eb458 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // eb45a cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // eb45c cmovg ecx,eax
            if fl.g() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // eb45f mov DWORD PTR [rbp-0x2f8],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd08), ((rcx as u32 as u64)) as u32);
            // eb465 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // eb467 mov r14,r12
            r14 = r12;
            // eb46a js 0xeb57c
            if fl.s() { pc = 0xeb57c; continue; }
            pc = 0xeb470;
        }
        0xeb470 => {
            // block eb470
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb470 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb470 movsxd rax,DWORD PTR [rbp-0x300]
            rax = sx::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64) as u64;
            // eb477 mov QWORD PTR [rbp-0x300],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd00), (rax) as u64);
            // eb47e movsxd rax,DWORD PTR [rbp-0x324]
            rax = sx::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc)) as u64) as u64;
            // eb485 mov QWORD PTR [rbp-0x308],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf8), (rax) as u64);
            // eb48c movsxd r14,r15d
            r14 = sx::<32>((r15 as u32 as u64)) as u64;
            // eb48f mov r15,r12
            r15 = r12;
            // eb492 mov QWORD PTR [rbp-0x330],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0), (r15) as u64);
            // eb499 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xeb49c;
        }
        0xeb49c => {
            // block eb49c
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb49c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb49c test r13,r13
            fl = logic::<64>(r13 & r13);
            // eb49f je 0xeb4f4
            if fl.e() { pc = 0xeb4f4; continue; }
            pc = 0xeb4a1;
        }
        0xeb4a1 => {
            // block eb4a1
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4a1 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb4a5 mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // eb4a9 mov rdi,r13
            rdi = r13;
            // eb4ac call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb4ae cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // eb4b1 jne 0xeb4f4
            if fl.ne() { pc = 0xeb4f4; continue; }
            pc = 0xeb4b3;
        }
        0xeb4b3 => {
            // block eb4b3
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4b3 mov rax,QWORD PTR [rbp-0x310]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0)) as u64;
            // eb4ba test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // eb4bc mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // eb4c1 jle 0xeb54a
            if fl.le() { pc = 0xeb54a; continue; }
            pc = 0xeb4c7;
        }
        0xeb4c7 => {
            // block eb4c7
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4c7 mov cl,BYTE PTR [rbx+rax*1+0x3]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x3)) as u64) as u8 as u64);
            // eb4cb mov BYTE PTR [r15+rax*1],cl
            m.w8(r15.wrapping_add(rax), ((rcx as u8 as u64)) as u8);
            // eb4cf mov cl,BYTE PTR [rbx+rax*1]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax)) as u64) as u8 as u64);
            // eb4d2 mov BYTE PTR [r15+rax*1+0x1],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x1), ((rcx as u8 as u64)) as u8);
            // eb4d7 mov cl,BYTE PTR [rbx+rax*1+0x1]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x1)) as u64) as u8 as u64);
            // eb4db mov BYTE PTR [r15+rax*1+0x2],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x2), ((rcx as u8 as u64)) as u8);
            // eb4e0 mov cl,BYTE PTR [rbx+rax*1+0x2]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x2)) as u64) as u8 as u64);
            // eb4e4 mov BYTE PTR [r15+rax*1+0x3],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x3), ((rcx as u8 as u64)) as u8);
            // eb4e9 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // eb4ed cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // eb4f0 jl 0xeb4c7
            if fl.l() { pc = 0xeb4c7; continue; }
            pc = 0xeb4f2;
        }
        0xeb4f2 => {
            // block eb4f2
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4f2 jmp 0xeb54a
            pc = 0xeb54a; continue;
        }
        0xeb4f4 => {
            // block eb4f4
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4f4 mov rax,QWORD PTR [rbp-0x318]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce8)) as u64;
            // eb4fb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb4fd je 0xeb53c
            if fl.e() { pc = 0xeb53c; continue; }
            pc = 0xeb4ff;
        }
        0xeb4ff => {
            // block eb4ff
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4ff mov rax,QWORD PTR [rbp-0x310]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0)) as u64;
            // eb506 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // eb508 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // eb50d jle 0xeb54a
            if fl.le() { pc = 0xeb54a; continue; }
            pc = 0xeb50f;
        }
        0xeb50f => {
            // block eb50f
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb50f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb50f mov cl,BYTE PTR [rbx+rax*1+0x3]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x3)) as u64) as u8 as u64);
            // eb513 mov BYTE PTR [r15+rax*1],cl
            m.w8(r15.wrapping_add(rax), ((rcx as u8 as u64)) as u8);
            // eb517 mov cl,BYTE PTR [rbx+rax*1+0x2]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x2)) as u64) as u8 as u64);
            // eb51b mov BYTE PTR [r15+rax*1+0x1],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x1), ((rcx as u8 as u64)) as u8);
            // eb520 mov cl,BYTE PTR [rbx+rax*1+0x1]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x1)) as u64) as u8 as u64);
            // eb524 mov BYTE PTR [r15+rax*1+0x2],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x2), ((rcx as u8 as u64)) as u8);
            // eb529 mov cl,BYTE PTR [rbx+rax*1]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax)) as u64) as u8 as u64);
            // eb52c mov BYTE PTR [r15+rax*1+0x3],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x3), ((rcx as u8 as u64)) as u8);
            // eb531 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // eb535 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // eb538 jl 0xeb50f
            if fl.l() { pc = 0xeb50f; continue; }
            pc = 0xeb53a;
        }
        0xeb53a => {
            // block eb53a
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb53a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb53a jmp 0xeb54a
            pc = 0xeb54a; continue;
        }
        0xeb53c => {
            // block eb53c
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb53c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb53c mov rdi,r15
            rdi = r15;
            // eb53f mov rsi,rbx
            rsi = rbx;
            // eb542 mov rdx,r14
            rdx = r14;
            // eb545 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb54a;
        }
        0xeb54a => {
            // block eb54a
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb54a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb54a add rbx,QWORD PTR [rbp-0x300]
            { let (r, f) = add::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64);
              rbx = r;
              fl = f; }
            // eb551 add r15,QWORD PTR [rbp-0x308]
            { let (r, f) = add::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf8)) as u64);
              r15 = r;
              fl = f; }
            // eb558 cmp r12d,DWORD PTR [rbp-0x2f8]
            fl = sub::<32>((r12 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd08)) as u64).1;
            // eb55f lea eax,[r12+0x1]
            rax = (r12.wrapping_add(0x1)) as u32 as u64;
            // eb564 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // eb567 jl 0xeb49c
            if fl.l() { pc = 0xeb49c; continue; }
            pc = 0xeb56d;
        }
        0xeb56d => {
            // block eb56d
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb56d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb56d mov r12,QWORD PTR [rbp-0x330]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // eb574 mov r14,r12
            r14 = r12;
            // eb577 jmp 0xeb57c
            pc = 0xeb57c; continue;
        }
        0xeb579 => {
            // block eb579
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb579 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb579 mov r14,rbx
            r14 = rbx;
            pc = 0xeb57c;
        }
        0xeb57c => {
            // block eb57c
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb57c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb57c mov DWORD PTR [rbp-0x2f4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd0c), (0x0u64) as u32);
            // eb586 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb58d lea rdx,[rbp-0x2f4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fd0c);
            // eb594 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eb596 call 0x2b74
            { let r = { let t = m.env.code_va(0x2b74); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb59b mov esi,DWORD PTR [rbp-0x2f4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd0c)) as u64) as u32 as u64;
            // eb5a1 mov ecx,DWORD PTR [rbp-0x324]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc)) as u64) as u32 as u64;
            // eb5a7 mov rax,QWORD PTR [rbp-0x320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // eb5ae imul ecx,eax
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rax as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // eb5b1 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb5b8 mov rdx,r14
            rdx = r14;
            // eb5bb call 0x44d68
            { let r = { let t = m.env.code_va(0x44d68); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5c0 mov rdi,r12
            rdi = r12;
            // eb5c3 call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5c8 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xeb5ca;
        }
        0xeb5ca => {
            // block eb5ca
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb5ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb5ca lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb5d1 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5d6 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // eb5d8 add rsp,0x308
            { let (r, f) = add::<64>(rsp, 0x308u64);
              rsp = r;
              fl = f; }
            // eb5df pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb5e0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eb5e2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eb5e4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb5e6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb5e8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb5e9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeb5ea => {
            // block eb5ea
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb5ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb5ea jmp 0xeb5ec
            pc = 0xeb5ec; continue;
        }
        0xeb5ec => {
            // block eb5ec
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb5ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb5ec mov rbx,rax
            rbx = rax;
            // eb5ef lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb5f6 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5fb mov rdi,rbx
            rdi = rbx;
            // eb5fe call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9874 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d9876 je 0xd98a6
            if fl.e() { pc = 0xd98a6; continue; }
            pc = 0xd9878;
        }
        0xd9878 => {
            // block d9878
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9878 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9878 cmp QWORD PTR [rbx+0x6b8],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x6b8)) as u64, 0x0u64).1;
            // d9880 je 0xd98ad
            if fl.e() { pc = 0xd98ad; continue; }
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
            if fl.ne() { pc = 0xd98ec; continue; }
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
            { let r = { let t = m.env.code_va(0xd7b18); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d98ba mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // d98bf mov rdi,rbx
            rdi = rbx;
            // d98c2 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xd9a4d; continue; }
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
            if fl.ne() { pc = 0xd9914; continue; }
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
            if fl.e() { pc = 0xd994d; continue; }
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
            if fl.ae() { pc = 0xd993b; continue; }
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
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ae() { pc = 0xd9968; continue; }
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
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d999e mov rdi,QWORD PTR [r15+0x6b8]
            rdi = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // d99a5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d99a8 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d99ac mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // d99b1 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x2e3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = r14; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d99fe mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d9a00 mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // d9a04 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d9a07 je 0xd9a18
            if fl.e() { pc = 0xd9a18; continue; }
            pc = 0xd9a09;
        }
        0xd9a09 => {
            // block d9a09
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a09 cmp QWORD PTR [rbp-0x58],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64, rdi).1;
            // d9a0d je 0xd9a13
            if fl.e() { pc = 0xd9a13; continue; }
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
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9a2c mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d9a30 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d9a33 je 0xd9bc6
            if fl.e() { pc = 0xd9bc6; continue; }
            pc = 0xd9a39;
        }
        0xd9a39 => {
            // block d9a39
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a39 cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // d9a3d je 0xd9a43
            if fl.e() { pc = 0xd9a43; continue; }
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
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9d9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9a7a mov r12,rax
            r12 = rax;
            // d9a7d mov rdi,r12
            rdi = r12;
            // d9a80 mov rsi,rbx
            rsi = rbx;
            // d9a83 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9a88 cmp BYTE PTR [rbp-0x71],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff8f)) as u64, 0x0u64).1;
            // d9a8c jne 0xd9a93
            if fl.ne() { pc = 0xd9a93; continue; }
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
            { let r = { let t = m.env.stub_va(0xa75); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9aa2 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // d9aa7 mov r15,QWORD PTR [rbp-0x68]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // d9aab mov rdi,r15
            rdi = r15;
            // d9aae call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9ab3 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // d9ab7 cmp eax,0xfffffff9
            fl = sub::<32>((rax as u32 as u64), 0xfffffff9u64).1;
            // d9aba jl 0xd9ae0
            if fl.l() { pc = 0xd9ae0; continue; }
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
            { let r = { let t = m.env.code_va(0xd8140); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9ad4 add r13d,0x4
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x4u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d9ad8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // d9adb cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // d9ade jl 0xd9ac5
            if fl.l() { pc = 0xd9ac5; continue; }
            pc = 0xd9ae0;
        }
        0xd9ae0 => {
            // block d9ae0
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9ae0 mov rdi,r12
            rdi = r12;
            // d9ae3 call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b03 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b06 mov rdi,r15
            rdi = r15;
            // d9b09 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b45 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b48 mov rdi,r15
            rdi = r15;
            // d9b4b call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b6c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b6f mov rdi,r15
            rdi = r15;
            // d9b72 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9bae mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9bb1 mov rdi,r15
            rdi = r15;
            // d9bb4 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9bba xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d9bbc mov rdi,r15
            rdi = r15;
            // d9bbf call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0xd9bf2; continue; }
            pc = 0xd9be3;
        }
        0xd9be3 => {
            // block d9be3
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9be3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9be3 cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // d9be7 je 0xd9bed
            if fl.e() { pc = 0xd9bed; continue; }
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
            if fl.e() { pc = 0xd9bda; continue; }
            pc = 0xd9c06;
        }
        0xd9c06 => {
            // block d9c06
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9c06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9c06 cmp QWORD PTR [rbp-0x58],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64, rdi).1;
            // d9c0a je 0xd9c10
            if fl.e() { pc = 0xd9c10; continue; }
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

/// parity: P2v @0xe3b48 size=952
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e3b48<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe3b48;
    loop {
        match pc {
        0xe3b48 => {
            // block e3b48
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3b48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3b48 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e3b49 mov rbp,rsp
            rbp = rsp;
            // e3b4c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e3b4e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e3b50 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e3b52 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e3b54 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e3b55 sub rsp,0xd8
            { let (r, f) = sub::<64>(rsp, 0xd8u64);
              rsp = r;
              fl = f; }
            // e3b5c mov r14,rdx
            r14 = rdx;
            // e3b5f mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // e3b62 mov r15,rdi
            r15 = rdi;
            // e3b65 call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3b6a mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // e3b6c test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e3b6e je 0xe3d36
            if fl.e() { pc = 0xe3d36; continue; }
            pc = 0xe3b74;
        }
        0xe3b74 => {
            // block e3b74
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3b74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3b74 lea rdi,[r15+0x34]
            rdi = r15.wrapping_add(0x34);
            // e3b78 lea rsi,[rip+0x558a9] # 0x139428
            rsi = m.env.data_va(318453);
            // e3b7f mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // e3b84 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e3b86 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // e3b89 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3b8e mov esi,0x3e8
            rsi = (0x3e8u64) as u32 as u64;
            // e3b93 mov rdi,r15
            rdi = r15;
            // e3b96 call 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3b9b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e3b9d je 0xe3d02
            if fl.e() { pc = 0xe3d02; continue; }
            pc = 0xe3ba3;
        }
        0xe3ba3 => {
            // block e3ba3
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3ba3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3ba3 xorps xmm0,xmm0
            x0 = 0;
            // e3ba6 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // e3baa mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // e3bb2 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e3bb6 mov rdi,r15
            rdi = r15;
            // e3bb9 call 0xe4a2e
            { let r = { let t = m.env.code_va(0xe4a2e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3bbe xorps xmm0,xmm0
            x0 = 0;
            // e3bc1 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // e3bc5 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // e3bcd lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e3bd1 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e3bd5 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3bda lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e3bde lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e3be2 call 0xe171e
            { let r = { let t = m.env.code_va(0xe171e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3be7 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e3beb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3bf0 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e3bf4 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e3bf8 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e3bfb sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e3bff movabs r12,0xaaaaaaaaaaaaaaab
            r12 = 0xaaaa_aaaa_aaaa_aaabu64;
            // e3c09 imul rax,r12
            { let (r, f) = imul::<64>(rax, r12); rax = r; fl = f; }
            // e3c0d cmp rax,0x3
            fl = sub::<64>(rax, 0x3u64).1;
            // e3c11 jb 0xe3d1a
            if fl.b() { pc = 0xe3d1a; continue; }
            pc = 0xe3c17;
        }
        0xe3c17 => {
            // block e3c17
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3c17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3c17 xorps xmm0,xmm0
            x0 = 0;
            // e3c1a movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // e3c21 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // e3c29 lea rsi,[rip+0x557e3] # 0x139413
            rsi = m.env.data_va(318432);
            // e3c30 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e3c37 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e3c3c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3c41 lea rdx,[rbp-0xa8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e3c48 lea rcx,[rbp-0x90]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e3c4f mov rsi,rbx
            rsi = rbx;
            // e3c52 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3c57 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e3c59 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e3c60 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3c65 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e3c67 je 0xe3d0e
            if fl.e() { pc = 0xe3d0e; continue; }
            pc = 0xe3c6d;
        }
        0xe3c6d => {
            // block e3c6d
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3c6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3c6d lea rsi,[rip+0x557a6] # 0x13941a
            rsi = m.env.data_va(318439);
            // e3c74 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e3c7b call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3c80 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e3c82 je 0xe3d4a
            if fl.e() { pc = 0xe3d4a; continue; }
            pc = 0xe3c88;
        }
        0xe3c88 => {
            // block e3c88
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3c88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3c88 lea rsi,[rip+0x5578e] # 0x13941d
            rsi = m.env.data_va(318442);
            // e3c8f lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e3c96 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3c9b test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e3c9d jne 0xe3d0e
            if fl.ne() { pc = 0xe3d0e; continue; }
            pc = 0xe3c9f;
        }
        0xe3c9f => {
            // block e3c9f
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3c9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3c9f mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e3ca3 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e3ca7 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e3caa sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e3cae imul rax,r12
            { let (r, f) = imul::<64>(rax, r12); rax = r; fl = f; }
            // e3cb2 cmp rax,0x3
            fl = sub::<64>(rax, 0x3u64).1;
            // e3cb6 jb 0xe3d0e
            if fl.b() { pc = 0xe3d0e; continue; }
            pc = 0xe3cb8;
        }
        0xe3cb8 => {
            // block e3cb8
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3cb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3cb8 lea rsi,[rip+0x55763] # 0x139422
            rsi = m.env.data_va(318447);
            // e3cbf lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // e3cc6 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e3ccb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3cd0 add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e3cd4 lea rcx,[r15+0x10]
            rcx = r15.wrapping_add(0x10);
            // e3cd8 lea rdx,[rbp-0x100]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // e3cdf mov rsi,rbx
            rsi = rbx;
            // e3ce2 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3ce7 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // e3cee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3cf3 mov DWORD PTR [r15+0x8],0x24
            m.w32(r15.wrapping_add(0x8), (0x24u64) as u32);
            // e3cfb xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e3cfd jmp 0xe3e68
            pc = 0xe3e68; continue;
        }
        0xe3d02 => {
            // block e3d02
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3d02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3d02 mov DWORD PTR [r15+0x8],0x15
            m.w32(r15.wrapping_add(0x8), (0x15u64) as u32);
            // e3d0a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e3d0c jmp 0xe3d36
            pc = 0xe3d36; continue;
        }
        0xe3d0e => {
            // block e3d0e
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3d0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3d0e lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e3d15 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe3d1a;
        }
        0xe3d1a => {
            // block e3d1a
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3d1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3d1a mov DWORD PTR [r15+0x8],0x17
            m.w32(r15.wrapping_add(0x8), (0x17u64) as u32);
            // e3d22 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe3d24;
        }
        0xe3d24 => {
            // block e3d24
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3d24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3d24 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e3d28 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3d2d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e3d31 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe3d36;
        }
        0xe3d36 => {
            // block e3d36
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3d36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3d36 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e3d38 add rsp,0xd8
            { let (r, f) = add::<64>(rsp, 0xd8u64);
              rsp = r;
              fl = f; }
            // e3d3f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e3d40 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e3d42 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e3d44 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e3d46 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e3d48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e3d49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe3d4a => {
            // block e3d4a
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3d4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3d4a xorps xmm0,xmm0
            x0 = 0;
            // e3d4d movaps XMMWORD PTR [rbp-0xc0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff40), x0);
            // e3d54 mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // e3d5f mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e3d63 lea rsi,[rip+0x55769] # 0x1394d3
            rsi = m.env.data_va(318624);
            // e3d6a lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e3d71 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // e3d76 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3d7b add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e3d7f lea rdx,[rbp-0xd8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e3d86 lea rcx,[rbp-0xc0]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e3d8d mov rsi,rbx
            rsi = rbx;
            // e3d90 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3d95 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e3d97 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e3d9e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3da3 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e3da5 je 0xe3e52
            if fl.e() { pc = 0xe3e52; continue; }
            pc = 0xe3dab;
        }
        0xe3dab => {
            // block e3dab
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3dab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3dab mov al,BYTE PTR [rbp-0xc0]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64) as u8 as u64);
            // e3db1 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e3db3 je 0xe3e52
            if fl.e() { pc = 0xe3e52; continue; }
            pc = 0xe3db9;
        }
        0xe3db9 => {
            // block e3db9
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3db9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3db9 cmp QWORD PTR [rbp-0xb8],0x81
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64, 0x81u64).1;
            // e3dc4 jne 0xe3e52
            if fl.ne() { pc = 0xe3e52; continue; }
            pc = 0xe3dca;
        }
        0xe3dca => {
            // block e3dca
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3dca xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e3dcd lea r15,[rbp-0xdc]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff24);
            // e3dd4 lea r12,[rbp-0xe8]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // e3ddb jmp 0xe3de6
            pc = 0xe3de6; continue;
        }
        0xe3ddd => {
            // block e3ddd
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3ddd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3ddd inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // e3de0 mov al,BYTE PTR [rbp-0xc0]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64) as u8 as u64);
            pc = 0xe3de6;
        }
        0xe3de6 => {
            // block e3de6
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3de6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3de6 mov BYTE PTR [rbp-0xda],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff26), (0x0u64) as u8);
            // e3ded mov WORD PTR [rbp-0xdc],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff24), (0x0u64) as u16);
            // e3df6 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e3df8 jne 0xe3e11
            if fl.ne() { pc = 0xe3e11; continue; }
            pc = 0xe3dfa;
        }
        0xe3dfa => {
            // block e3dfa
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3dfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3dfa mov al,BYTE PTR [rbp+r13*2-0xbf]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(r13.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_ff41)) as u64) as u8 as u64);
            // e3e02 mov BYTE PTR [rbp-0xdc],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff24), ((rax as u8 as u64)) as u8);
            // e3e08 lea rax,[rbp-0xbf]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff41);
            // e3e0f jmp 0xe3e29
            pc = 0xe3e29; continue;
        }
        0xe3e11 => {
            // block e3e11
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e11 mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // e3e18 mov al,BYTE PTR [rax+r13*2]
            rax = (rax & !0xff) | ((m.r8(rax.wrapping_add(r13.wrapping_mul(2))) as u64) as u8 as u64);
            // e3e1c mov BYTE PTR [rbp-0xdc],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff24), ((rax as u8 as u64)) as u8);
            // e3e22 mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            pc = 0xe3e29;
        }
        0xe3e29 => {
            // block e3e29
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e29 mov al,BYTE PTR [rax+r13*2+0x1]
            rax = (rax & !0xff) | ((m.r8(rax.wrapping_add(r13.wrapping_mul(2)).wrapping_add(0x1)) as u64) as u8 as u64);
            // e3e2e mov BYTE PTR [rbp-0xdb],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff25), ((rax as u8 as u64)) as u8);
            // e3e34 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // e3e39 mov rdi,r15
            rdi = r15;
            // e3e3c mov rsi,r12
            rsi = r12;
            // e3e3f call 0xec2d8
            { let r = { let t = m.env.stub_va(0xaa5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3e44 mov BYTE PTR [r14+r13*1],al
            m.w8(r14.wrapping_add(r13), ((rax as u8 as u64)) as u8);
            // e3e48 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // e3e4a cmp r13,0x3f
            fl = sub::<64>(r13, 0x3fu64).1;
            // e3e4e jne 0xe3ddd
            if fl.ne() { pc = 0xe3ddd; continue; }
            pc = 0xe3e50;
        }
        0xe3e50 => {
            // block e3e50
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e50 jmp 0xe3e5c
            pc = 0xe3e5c; continue;
        }
        0xe3e52 => {
            // block e3e52
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e52 mov DWORD PTR [r15+0x8],0x24
            m.w32(r15.wrapping_add(0x8), (0x24u64) as u32);
            // e3e5a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe3e5c;
        }
        0xe3e5c => {
            // block e3e5c
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e5c lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e3e63 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe3e68;
        }
        0xe3e68 => {
            // block e3e68
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e68 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e3e6f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3e74 jmp 0xe3d24
            pc = 0xe3d24; continue;
        }
        0xe3e79 => {
            // block e3e79
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e79 mov rbx,rax
            rbx = rax;
            // e3e7c jmp 0xe3edc
            pc = 0xe3edc; continue;
        }
        0xe3e7e => {
            // block e3e7e
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e7e mov rbx,rax
            rbx = rax;
            // e3e81 jmp 0xe3ed3
            pc = 0xe3ed3; continue;
        }
        0xe3e83 => {
            // block e3e83
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e83 mov rbx,rax
            rbx = rax;
            // e3e86 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e3e8a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3e8f jmp 0xe3ed3
            pc = 0xe3ed3; continue;
        }
        0xe3e91 => {
            // block e3e91
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e91 mov rbx,rax
            rbx = rax;
            // e3e94 jmp 0xe3ec7
            pc = 0xe3ec7; continue;
        }
        0xe3e96 => {
            // block e3e96
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3e96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3e96 mov rbx,rax
            rbx = rax;
            // e3e99 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e3ea0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3ea5 jmp 0xe3ec7
            pc = 0xe3ec7; continue;
        }
        0xe3ea7 => {
            // block e3ea7
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3ea7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3ea7 mov rbx,rax
            rbx = rax;
            // e3eaa jmp 0xe3ebb
            pc = 0xe3ebb; continue;
        }
        0xe3eac => {
            // block e3eac
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3eac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3eac mov rbx,rax
            rbx = rax;
            // e3eaf lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e3eb6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe3ebb;
        }
        0xe3ebb => {
            // block e3ebb
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3ebb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3ebb lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e3ec2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe3ec7;
        }
        0xe3ec7 => {
            // block e3ec7
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3ec7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3ec7 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e3ece call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe3ed3;
        }
        0xe3ed3 => {
            // block e3ed3
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3ed3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3ed3 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e3ed7 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe3edc;
        }
        0xe3edc => {
            // block e3edc
            #[cfg(feature = "trace")] eprintln!("f_000e3b48 pc e3edc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3edc lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e3ee0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3ee5 mov rdi,rbx
            rdi = rbx;
            // e3ee8 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3eed mov rbx,rax
            rbx = rax;
            // e3ef0 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // e3ef7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3efc jmp 0xe3ec7
            pc = 0xe3ec7; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa46c6 size=944
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a46c6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xa46c6;
    loop {
        match pc {
        0xa46c6 => {
            // block a46c6
            #[cfg(feature = "trace")] eprintln!("f_000a46c6 pc a46c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a46c6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a46c7 mov rbp,rsp
            rbp = rsp;
            // a46ca push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a46cc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a46ce push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // a46d0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // a46d2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a46d3 sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // a46da mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // a46dd mov QWORD PTR [rbp-0x160],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rdi) as u64);
            // a46e4 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a46eb lea r14,[rbp-0x150]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a46f2 mov rax,QWORD PTR [rip+0x9bad7] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // a46f9 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // a46fd mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // a4704 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // a4708 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a470f mov rsi,r14
            rsi = r14;
            // a4712 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4717 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // a471f mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // a4726 mov r15,QWORD PTR [rip+0x9bc93] # 0x1403c0
            r15 = m.r64(m.env.data_va(347021)) as u64;
            // a472d lea rbx,[r15+0x18]
            rbx = r15.wrapping_add(0x18);
            // a4731 mov QWORD PTR [rbp-0x158],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rbx) as u64);
            // a4738 add r15,0x40
            { let (r, f) = add::<64>(r15, 0x40u64);
              r15 = r;
              fl = f; }
            // a473c mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // a4743 mov rdi,r14
            rdi = r14;
            // a4746 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a474b mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // a4752 mov rax,QWORD PTR [rip+0x9bc4f] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // a4759 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // a475d mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // a4764 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a476b mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // a4776 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // a4781 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // a478c mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // a4797 mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // a47a1 xorps xmm0,xmm0
            x0 = 0;
            // a47a4 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // a47a8 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // a47b0 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a47b4 mov rdi,r14
            rdi = r14;
            // a47b7 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a47bc mov ebx,r13d
            rbx = ((r13 as u32 as u64)) as u32 as u64;
            // a47bf and ebx,0x20000000
            { let r = (rbx as u32 as u64) & 0x20000000u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // a47c5 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a47c9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a47ce test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // a47d0 lea rax,[rip+0x87599] # 0x12bd70
            rax = m.env.data_va(263485);
            // a47d7 lea rsi,[rip+0x87576] # 0x12bd54
            rsi = m.env.data_va(263457);
            // a47de cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // a47e2 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // a47e4 shr rdx,0x1d
            { let (r, f) = shr::<64>(rdx, 0x1du64, fl); rdx = r; fl = f; }
            // a47e8 xor rdx,0x3
            { let r = rdx ^ 0x3u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a47ec lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a47f3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a47f8 mov r12,rax
            r12 = rax;
            // a47fb mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // a47ff mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a4803 add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // a4806 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a480a call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a480f mov rbx,r14
            rbx = r14;
            // a4812 mov rsi,QWORD PTR [rip+0x9b977] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a4819 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a481d call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4822 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a4825 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a4829 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a482e mov rdi,rax
            rdi = rax;
            // a4831 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4833 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a4836 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a483a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a483f movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a4843 mov rdi,r12
            rdi = r12;
            // a4846 mov r14,rbx
            r14 = rbx;
            // a4849 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a484e mov rdi,r12
            rdi = r12;
            // a4851 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4856 lea rsi,[rip+0x8f4d3] # 0x133d30
            rsi = m.env.data_va(296189);
            // a485d mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // a4862 mov rdi,r12
            rdi = r12;
            // a4865 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a486a lea rsi,[rip+0x85c04] # 0x12a475
            rsi = m.env.data_va(257090);
            // a4871 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // a4876 mov rdi,rax
            rdi = rax;
            // a4879 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a487e shr r13d,0xf
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0xfu64, fl); r13 = (r) as u32 as u64; fl = f; }
            // a4882 and r13d,0x1f
            { let r = (r13 as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // a4886 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a4889 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a488d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // a4891 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a4894 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a4897 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // a489b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a489e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a48a2 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // a48aa mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a48ad mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a48b1 mov QWORD PTR [rcx+rax*1+0x18],0x4
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x4u64) as u64);
            // a48ba mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a48bd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a48c1 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // a48cc movzx ebx,r13w
            rbx = ((r13 as u16 as u64)) as u32 as u64;
            // a48d0 mov rdi,rax
            rdi = rax;
            // a48d3 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // a48d5 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a48da mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a48dd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a48e1 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // a48e5 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a48e8 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a48eb mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // a48ef mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a48f2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a48f6 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // a4901 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a4904 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a4908 and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // a4910 lea rsi,[rip+0x8f650] # 0x133f67
            rsi = m.env.data_va(296756);
            // a4917 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // a491c mov rdi,rax
            rdi = rax;
            // a491f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4924 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a4927 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a492b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // a492f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a4932 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a4935 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // a4939 mov rdi,rax
            rdi = rax;
            // a493c mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // a493e call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4943 lea rsi,[rip+0x85d38] # 0x12a682
            rsi = m.env.data_va(257615);
            // a494a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // a494f mov rdi,rax
            rdi = rax;
            // a4952 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4957 mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // a495e mov rsi,r14
            rsi = r14;
            // a4961 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4966 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a496d mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a4974 mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // a497b mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a4982 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a4989 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a4990 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4995 mov rdi,r14
            rdi = r14;
            // a4998 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a499d mov rsi,QWORD PTR [rip+0x9b864] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a49a4 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a49a8 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a49af call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a49b4 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a49bb call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a49c0 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // a49c7 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // a49ce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // a49cf pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // a49d1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // a49d3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // a49d5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // a49d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a49d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xa49d9 => {
            // block a49d9
            #[cfg(feature = "trace")] eprintln!("f_000a46c6 pc a49d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a49d9 mov rbx,r14
            rbx = r14;
            // a49dc mov r12,rax
            r12 = rax;
            pc = 0xa49df;
        }
        0xa49df => {
            // block a49df
            #[cfg(feature = "trace")] eprintln!("f_000a46c6 pc a49df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a49df mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a49e6 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a49ed mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // a49f4 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a49fb mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a4a02 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a4a09 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4a0e mov rdi,rbx
            rdi = rbx;
            pc = 0xa4a11;
        }
        0xa4a11 => {
            // block a4a11
            #[cfg(feature = "trace")] eprintln!("f_000a46c6 pc a4a11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a4a11 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa4a16;
        }
        0xa4a16 => {
            // block a4a16
            #[cfg(feature = "trace")] eprintln!("f_000a46c6 pc a4a16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a4a16 mov rsi,QWORD PTR [rip+0x9b7eb] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a4a1d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a4a21 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a4a28 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa4a2d;
        }
        0xa4a2d => {
            // block a4a2d
            #[cfg(feature = "trace")] eprintln!("f_000a46c6 pc a4a2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a4a2d lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a4a34 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4a39 mov rdi,r12
            rdi = r12;
            // a4a3c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4a41 mov r12,rax
            r12 = rax;
            // a4a44 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a4a48 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4a4d jmp 0xa49df
            pc = 0xa49df; continue;
        }
        0xa4a4f => {
            // block a4a4f
            #[cfg(feature = "trace")] eprintln!("f_000a46c6 pc a4a4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a4a4f mov r12,rax
            r12 = rax;
            // a4a52 jmp 0xa4a2d
            pc = 0xa4a2d; continue;
        }
        0xa4a54 => {
            // block a4a54
            #[cfg(feature = "trace")] eprintln!("f_000a46c6 pc a4a54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a4a54 mov r12,rax
            r12 = rax;
            // a4a57 jmp 0xa4a16
            pc = 0xa4a16; continue;
        }
        0xa4a59 => {
            // block a4a59
            #[cfg(feature = "trace")] eprintln!("f_000a46c6 pc a4a59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a4a59 mov r12,rax
            r12 = rax;
            // a4a5c lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a4a60 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4a65 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a4a6c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4a71 mov rdi,r14
            rdi = r14;
            // a4a74 jmp 0xa4a11
            pc = 0xa4a11; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdda0e size=930
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dda0e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdda0e;
    loop {
        match pc {
        0xdda0e => {
            // block dda0e
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc dda0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dda0e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dda0f mov rbp,rsp
            rbp = rsp;
            // dda12 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dda14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dda16 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dda18 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dda1a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dda1b sub rsp,0x100a8
            { let (r, f) = sub::<64>(rsp, 0x100a8u64);
              rsp = r;
              fl = f; }
            // dda22 mov r14,rdi
            r14 = rdi;
            // dda25 mov rax,QWORD PTR [rip+0x629ac] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dda2c mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dda2f mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dda33 mov QWORD PTR [rbp-0x10038],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ffc8), (0x0u64) as u64);
            // dda3e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dda41 lea rsi,[rbp-0x1003c]
            rsi = rbp.wrapping_add(0xffff_ffff_fffe_ffc4);
            // dda48 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // dda4b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dda4d call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dda53 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dda56 lea rsi,[rbp-0x10040]
            rsi = rbp.wrapping_add(0xffff_ffff_fffe_ffc0);
            // dda5d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dda5f mov rdi,r14
            rdi = r14;
            // dda62 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dda68 cmp DWORD PTR [rbp-0x10040],0xa
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ffc0)) as u64, 0xau64).1;
            // dda6f sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // dda72 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dda75 lea rsi,[rbp-0x10041]
            rsi = rbp.wrapping_add(0xffff_ffff_fffe_ffbf);
            // dda7c lea rdx,[rbp-0x10042]
            rdx = rbp.wrapping_add(0xffff_ffff_fffe_ffbe);
            // dda83 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dda85 mov rdi,r14
            rdi = r14;
            // dda88 call QWORD PTR [rax+0x5e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x5e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dda8e mov esi,DWORD PTR [rbp-0x1003c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ffc4)) as u64) as u32 as u64;
            // dda94 mov edx,DWORD PTR [r14+0x274]
            rdx = (m.r32(r14.wrapping_add(0x274)) as u64) as u32 as u64;
            // dda9b movzx r9d,BYTE PTR [rbp-0x10042]
            r9 = (m.r8(rbp.wrapping_add(0xffff_ffff_fffe_ffbe)) as u64) as u32 as u64;
            // ddaa3 movzx ecx,BYTE PTR [rbp-0x10041]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_fffe_ffbf)) as u64) as u32 as u64;
            // ddaaa movzx r8d,bl
            r8 = ((rbx as u8 as u64)) as u32 as u64;
            // ddaae lea rdi,[rbp-0x10080]
            rdi = rbp.wrapping_add(0xffff_ffff_fffe_ff80);
            // ddab5 call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddaba mov eax,DWORD PTR [rbp-0x10080]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff80)) as u64) as u32 as u64;
            // ddac0 mov DWORD PTR [rbp-0x1009c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_fffe_ff64), ((rax as u32 as u64)) as u32);
            // ddac6 mov r13d,DWORD PTR [rbp-0x1007c]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff84)) as u64) as u32 as u64;
            // ddacd mov eax,DWORD PTR [rbp-0x10078]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff88)) as u64) as u32 as u64;
            // ddad3 mov QWORD PTR [rbp-0x10098],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff68), (rax) as u64);
            // ddada mov ebx,DWORD PTR [rbp-0x10074]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff8c)) as u64) as u32 as u64;
            // ddae0 mov r12,QWORD PTR [r14+0x280]
            r12 = m.r64(r14.wrapping_add(0x280)) as u64;
            // ddae7 test r12,r12
            fl = logic::<64>(r12 & r12);
            // ddaea jne 0xddb3d
            if fl.ne() { pc = 0xddb3d; continue; }
            pc = 0xddaec;
        }
        0xddaec => {
            // block ddaec
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddaec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddaec cmp BYTE PTR [r14+0x279],0x0
            fl = sub::<8>(m.r8(r14.wrapping_add(0x279)) as u64, 0x0u64).1;
            // ddaf4 je 0xddb19
            if fl.e() { pc = 0xddb19; continue; }
            pc = 0xddaf6;
        }
        0xddaf6 => {
            // block ddaf6
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddaf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddaf6 mov eax,DWORD PTR [rbp-0x1009c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff64)) as u64) as u32 as u64;
            // ddafc mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // ddafe mov rax,QWORD PTR [rbp-0x10098]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddb05 imul edi,eax
            { let (r, f) = imul::<32>((rdi as u32 as u64), (rax as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // ddb08 shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // ddb0c call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddb11 mov r12,rax
            r12 = rax;
            // ddb14 mov r15,r12
            r15 = r12;
            // ddb17 jmp 0xddb3d
            pc = 0xddb3d; continue;
        }
        0xddb19 => {
            // block ddb19
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddb19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddb19 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ddb1c mov esi,DWORD PTR [r14+0x250]
            rsi = (m.r32(r14.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddb23 lea rdx,[rbp-0x10038]
            rdx = rbp.wrapping_add(0xffff_ffff_fffe_ffc8);
            // ddb2a mov rdi,r14
            rdi = r14;
            // ddb2d call QWORD PTR [rax+0xa10]
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddb33 mov r12,QWORD PTR [rbp-0x10038]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ffc8)) as u64;
            // ddb3a xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xddb3d;
        }
        0xddb3d => {
            // block ddb3d
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddb3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddb3d mov DWORD PTR [rbp-0x100bc],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_fffe_ff44), ((r13 as u32 as u64)) as u32);
            // ddb44 mov QWORD PTR [rbp-0x100b8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff48), (r15) as u64);
            // ddb4b mov QWORD PTR [rbp-0x100b0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff50), (r14) as u64);
            // ddb52 lea r14,[rbp-0x8030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // ddb59 mov esi,0x4000
            rsi = (0x4000u64) as u32 as u64;
            // ddb5e mov rdi,r14
            rdi = r14;
            // ddb61 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddb66 lea rsi,[rbp-0x10030]
            rsi = rbp.wrapping_add(0xffff_ffff_fffe_ffd0);
            // ddb6d mov rdi,r14
            rdi = r14;
            // ddb70 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // ddb73 call 0x67628
            { let r = { let t = m.env.code_va(0x67628); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddb78 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // ddb7a je 0xddbd2
            if fl.e() { pc = 0xddbd2; continue; }
            pc = 0xddb7c;
        }
        0xddb7c => {
            // block ddb7c
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddb7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddb7c mov rcx,QWORD PTR [rbp-0x10098]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddb83 lea r14d,[rcx*4+0x0]
            r14 = (rcx.wrapping_mul(4)) as u32 as u64;
            // ddb8b lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // ddb8e lea rax,[rax*4+0x4]
            rax = rax.wrapping_mul(4).wrapping_add(0x4);
            // ddb96 imul rax,rcx
            { let (r, f) = imul::<64>(rax, rcx); rax = r; fl = f; }
            // ddb9a add rax,r12
            { let (r, f) = add::<64>(rax, r12);
              rax = r;
              fl = f; }
            // ddb9d mov QWORD PTR [rbp-0x10090],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff70), (rax) as u64);
            // ddba4 lea r13,[rcx*4+0x0]
            r13 = rcx.wrapping_mul(4);
            // ddbac mov r15d,ebx
            r15 = ((rbx as u32 as u64)) as u32 as u64;
            pc = 0xddbaf;
        }
        0xddbaf => {
            // block ddbaf
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbaf mov rdi,r12
            rdi = r12;
            // ddbb2 lea rsi,[rbp-0x8030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // ddbb9 mov rdx,r14
            rdx = r14;
            // ddbbc call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddbc1 add r12,r13
            { let (r, f) = add::<64>(r12, r13);
              r12 = r;
              fl = f; }
            // ddbc4 dec r15d
            { let (r, f) = dec::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // ddbc7 jne 0xddbaf
            if fl.ne() { pc = 0xddbaf; continue; }
            pc = 0xddbc9;
        }
        0xddbc9 => {
            // block ddbc9
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbc9 mov r12,QWORD PTR [rbp-0x10090]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff70)) as u64;
            // ddbd0 jmp 0xddbd4
            pc = 0xddbd4; continue;
        }
        0xddbd2 => {
            // block ddbd2
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbd2 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xddbd4;
        }
        0xddbd4 => {
            // block ddbd4
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbd4 cmp ebx,DWORD PTR [rbp-0x1009c]
            fl = sub::<32>((rbx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff64)) as u64).1;
            // ddbda mov r13,rbx
            r13 = rbx;
            // ddbdd mov ebx,DWORD PTR [rbp-0x100bc]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff44)) as u64) as u32 as u64;
            // ddbe3 jae 0xddca6
            if fl.ae() { pc = 0xddca6; continue; }
            pc = 0xddbe9;
        }
        0xddbe9 => {
            // block ddbe9
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbe9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbe9 mov rax,QWORD PTR [rbp-0x10098]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddbf0 lea eax,[rax*4+0x0]
            rax = (rax.wrapping_mul(4)) as u32 as u64;
            // ddbf7 mov QWORD PTR [rbp-0x100a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff58), (rax) as u64);
            // ddbfe lea eax,[r13*8+0x0]
            rax = (r13.wrapping_mul(8)) as u32 as u64;
            // ddc06 lea r14,[rbp-0x10030]
            r14 = rbp.wrapping_add(0xffff_ffff_fffe_ffd0);
            // ddc0d xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xddc10;
        }
        0xddc10 => {
            // block ddc10
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddc10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddc10 mov DWORD PTR [rbp-0x10090],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_fffe_ff70), ((rax as u32 as u64)) as u32);
            // ddc16 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // ddc18 mov r8d,ebx
            r8 = ((rbx as u32 as u64)) as u32 as u64;
            // ddc1b lea rcx,[rbp-0x802a]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_7fd6);
            // ddc22 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // ddc24 je 0xddc5f
            if fl.e() { pc = 0xddc5f; continue; }
            pc = 0xddc26;
        }
        0xddc26 => {
            // block ddc26
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddc26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddc26 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // ddc28 and esi,0xfffe
            { let r = (rsi as u32 as u64) & 0xfffeu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ddc2e cmp esi,0x4095
            fl = sub::<32>((rsi as u32 as u64), 0x4095u64).1;
            // ddc34 movzx esi,ax
            rsi = ((rax as u16 as u64)) as u32 as u64;
            // ddc37 cmova esi,r15d
            if fl.a() { rsi = ((r15 as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // ddc3b mov edi,0x4095
            rdi = (0x4095u64) as u32 as u64;
            // ddc40 sub edi,esi
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rsi as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // ddc42 mov WORD PTR [rcx],di
            m.w16(rcx, ((rdi as u16 as u64)) as u16);
            // ddc45 shl esi,0x4
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x4u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // ddc48 mov WORD PTR [rcx-0x2],si
            m.w16(rcx.wrapping_add(0xffff_ffff_ffff_fffe), ((rsi as u16 as u64)) as u16);
            // ddc4c mov WORD PTR [rcx-0x6],si
            m.w16(rcx.wrapping_add(0xffff_ffff_ffff_fffa), ((rsi as u16 as u64)) as u16);
            // ddc50 mov WORD PTR [rcx-0x4],si
            m.w16(rcx.wrapping_add(0xffff_ffff_ffff_fffc), ((rsi as u16 as u64)) as u16);
            // ddc54 add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // ddc58 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // ddc5a dec r8d
            { let (r, f) = dec::<32>((r8 as u32 as u64), fl); r8 = (r) as u32 as u64; fl = f; }
            // ddc5d jne 0xddc26
            if fl.ne() { pc = 0xddc26; continue; }
            pc = 0xddc5f;
        }
        0xddc5f => {
            // block ddc5f
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddc5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddc5f lea rdi,[rbp-0x8030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // ddc66 mov rsi,r14
            rsi = r14;
            // ddc69 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // ddc6b call 0x67628
            { let r = { let t = m.env.code_va(0x67628); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddc70 mov rdi,r12
            rdi = r12;
            // ddc73 mov rsi,r14
            rsi = r14;
            // ddc76 mov rdx,QWORD PTR [rbp-0x100a8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff58)) as u64;
            // ddc7d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddc82 mov rax,QWORD PTR [rbp-0x10098]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddc89 lea r12,[r12+rax*4]
            r12 = r12.wrapping_add(rax.wrapping_mul(4));
            // ddc8d inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // ddc90 mov eax,DWORD PTR [rbp-0x10090]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff70)) as u64) as u32 as u64;
            // ddc96 add eax,0x8
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // ddc99 cmp r13d,DWORD PTR [rbp-0x1009c]
            fl = sub::<32>((r13 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff64)) as u64).1;
            // ddca0 jne 0xddc10
            if fl.ne() { pc = 0xddc10; continue; }
            pc = 0xddca6;
        }
        0xddca6 => {
            // block ddca6
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddca6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddca6 mov r15,QWORD PTR [rbp-0x100b8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff48)) as u64;
            // ddcad test r15,r15
            fl = logic::<64>(r15 & r15);
            // ddcb0 mov rbx,QWORD PTR [rbp-0x100b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff50)) as u64;
            // ddcb7 je 0xddd32
            if fl.e() { pc = 0xddd32; continue; }
            pc = 0xddcb9;
        }
        0xddcb9 => {
            // block ddcb9
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddcb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddcb9 mov al,BYTE PTR [rbx+0x279]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x279)) as u64) as u8 as u64);
            // ddcbf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // ddcc1 je 0xddd32
            if fl.e() { pc = 0xddd32; continue; }
            pc = 0xddcc3;
        }
        0xddcc3 => {
            // block ddcc3
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddcc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddcc3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddcc6 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddccc lea rdx,[rbp-0x10084]
            rdx = rbp.wrapping_add(0xffff_ffff_fffe_ff7c);
            // ddcd3 mov rdi,rbx
            rdi = rbx;
            // ddcd6 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddcdc mov r14d,DWORD PTR [rbp-0x10084]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff7c)) as u64) as u32 as u64;
            // ddce3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddce6 mov rdi,rbx
            rdi = rbx;
            // ddce9 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddcef cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // ddcf2 jbe 0xddd02
            if fl.be() { pc = 0xddd02; continue; }
            pc = 0xddcf4;
        }
        0xddcf4 => {
            // block ddcf4
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddcf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddcf4 mov DWORD PTR [rbp-0x10084],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_fffe_ff7c), (0x0u64) as u32);
            // ddcfe xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ddd00 jmp 0xddd08
            pc = 0xddd08; continue;
        }
        0xddd02 => {
            // block ddd02
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddd02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddd02 mov esi,DWORD PTR [rbp-0x10084]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff7c)) as u64) as u32 as u64;
            pc = 0xddd08;
        }
        0xddd08 => {
            // block ddd08
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddd08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddd08 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd0b mov ecx,DWORD PTR [rbp-0x1009c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff64)) as u64) as u32 as u64;
            // ddd11 mov rdx,QWORD PTR [rbp-0x10098]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddd18 imul ecx,edx
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rdx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // ddd1b shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // ddd1e mov rdi,rbx
            rdi = rbx;
            // ddd21 mov rdx,r15
            rdx = r15;
            // ddd24 call QWORD PTR [rax+0x2c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd2a mov rdi,r15
            rdi = r15;
            // ddd2d call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xddd32;
        }
        0xddd32 => {
            // block ddd32
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddd32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddd32 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd35 movzx esi,BYTE PTR [rbx+0x278]
            rsi = (m.r8(rbx.wrapping_add(0x278)) as u64) as u32 as u64;
            // ddd3c mov rdi,rbx
            rdi = rbx;
            // ddd3f call QWORD PTR [rax+0x570]
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd45 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd48 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddd4e mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // ddd54 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // ddd59 mov rdi,rbx
            rdi = rbx;
            // ddd5c call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd62 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd65 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddd6b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ddd6d mov rdi,rbx
            rdi = rbx;
            // ddd70 call QWORD PTR [rax+0x4f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd76 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd79 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddd7f mov rdi,rbx
            rdi = rbx;
            // ddd82 call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd88 mov rax,QWORD PTR [rip+0x62649] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // ddd8f mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // ddd92 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // ddd96 jne 0xdddaa
            if fl.ne() { pc = 0xdddaa; continue; }
            pc = 0xddd98;
        }
        0xddd98 => {
            // block ddd98
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddd98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddd98 add rsp,0x100a8
            { let (r, f) = add::<64>(rsp, 0x100a8u64);
              rsp = r;
              fl = f; }
            // ddd9f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ddda0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ddda2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ddda4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ddda6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ddda8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ddda9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdddaa => {
            // block dddaa
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc dddaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dddaa call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x57d4a size=914
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00057d4a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x57d4a;
    loop {
        match pc {
        0x57d4a => {
            // block 57d4a
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57d4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57d4a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 57d4b mov rbp,rsp
            rbp = rsp;
            // 57d4e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 57d50 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 57d52 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 57d54 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 57d56 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 57d57 sub rsp,0x178
            { let (r, f) = sub::<64>(rsp, 0x178u64);
              rsp = r;
              fl = f; }
            // 57d5e lea rdi,[rip+0xd3c08] # 0x12b96d
            rdi = m.env.data_va(262458);
            // 57d65 call 0xec1ee
            { let r = { let t = m.env.stub_va(0x9bb); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57d6a test rax,rax
            fl = logic::<64>(rax & rax);
            // 57d6d lea rbx,[rip+0xe0f54] # 0x138cc8
            rbx = m.env.data_va(316565);
            // 57d74 cmovne rbx,rax
            if fl.ne() { rbx = rax; }
            // 57d78 mov rdi,rbx
            rdi = rbx;
            // 57d7b call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57d80 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 57d84 mov rsi,rbx
            rsi = rbx;
            // 57d87 mov rdx,rax
            rdx = rax;
            // 57d8a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57d8f movzx ecx,BYTE PTR [rbp-0x58]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 57d93 mov al,cl
            rax = (rax & !0xff) | (((rcx as u8 as u64)) as u8 as u64);
            // 57d95 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 57d97 jne 0x57da1
            if fl.ne() { pc = 0x57da1; continue; }
            pc = 0x57d99;
        }
        0x57d99 => {
            // block 57d99
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57d99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57d99 mov rdx,rcx
            rdx = rcx;
            // 57d9c shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 57d9f jmp 0x57da5
            pc = 0x57da5; continue;
        }
        0x57da1 => {
            // block 57da1
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57da1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57da1 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0x57da5;
        }
        0x57da5 => {
            // block 57da5
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57da5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57da5 cmp rdx,0x12
            fl = sub::<64>(rdx, 0x12u64).1;
            // 57da9 jne 0x58019
            if fl.ne() { pc = 0x58019; continue; }
            pc = 0x57daf;
        }
        0x57daf => {
            // block 57daf
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57daf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57daf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 57db1 jne 0x57dbc
            if fl.ne() { pc = 0x57dbc; continue; }
            pc = 0x57db3;
        }
        0x57db3 => {
            // block 57db3
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57db3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57db3 lea rax,[rbp-0x57]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa9);
            // 57db7 shr rcx,1
            { let (r, f) = shr::<64>(rcx, 0x1u64, fl); rcx = r; fl = f; }
            // 57dba jmp 0x57dc4
            pc = 0x57dc4; continue;
        }
        0x57dbc => {
            // block 57dbc
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57dbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57dbc mov rcx,QWORD PTR [rbp-0x50]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 57dc0 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0x57dc4;
        }
        0x57dc4 => {
            // block 57dc4
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57dc4 cmp rcx,0x2
            fl = sub::<64>(rcx, 0x2u64).1;
            // 57dc8 jb 0x58019
            if fl.b() { pc = 0x58019; continue; }
            pc = 0x57dce;
        }
        0x57dce => {
            // block 57dce
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57dce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57dce cmp rcx,0x2
            fl = sub::<64>(rcx, 0x2u64).1;
            // 57dd2 jl 0x58019
            if fl.l() { pc = 0x58019; continue; }
            pc = 0x57dd8;
        }
        0x57dd8 => {
            // block 57dd8
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57dd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57dd8 mov rdx,rcx
            rdx = rcx;
            // 57ddb dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 57dde je 0x58019
            if fl.e() { pc = 0x58019; continue; }
            pc = 0x57de4;
        }
        0x57de4 => {
            // block 57de4
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57de4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57de4 lea r8,[rax+rcx*1]
            r8 = rax.wrapping_add(rcx);
            // 57de8 lea r9,[rax+rcx*1-0x1]
            r9 = rax.wrapping_add(rcx).wrapping_add(0xffff_ffff_ffff_ffff);
            // 57ded lea rdi,[rip+0xd2681] # 0x12a475
            rdi = m.env.data_va(257090);
            // 57df4 mov rcx,rax
            rcx = rax;
            pc = 0x57df7;
        }
        0x57df7 => {
            // block 57df7
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57df7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57df7 movzx ebx,BYTE PTR [rcx]
            rbx = (m.r8(rcx) as u64) as u32 as u64;
            // 57dfa cmp ebx,0x30
            fl = sub::<32>((rbx as u32 as u64), 0x30u64).1;
            // 57dfd mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 57e02 jne 0x57e19
            if fl.ne() { pc = 0x57e19; continue; }
            pc = 0x57e04;
        }
        0x57e04 => {
            // block 57e04
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57e04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57e04 cmp rbx,0x2
            fl = sub::<64>(rbx, 0x2u64).1;
            // 57e08 je 0x57e26
            if fl.e() { pc = 0x57e26; continue; }
            pc = 0x57e0a;
        }
        0x57e0a => {
            // block 57e0a
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57e0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57e0a movzx edx,BYTE PTR [rbx+rdi*1]
            rdx = (m.r8(rbx.wrapping_add(rdi)) as u64) as u32 as u64;
            // 57e0e movzx esi,BYTE PTR [rcx+rbx*1]
            rsi = (m.r8(rcx.wrapping_add(rbx)) as u64) as u32 as u64;
            // 57e12 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 57e15 cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // 57e17 je 0x57e04
            if fl.e() { pc = 0x57e04; continue; }
            pc = 0x57e19;
        }
        0x57e19 => {
            // block 57e19
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57e19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57e19 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 57e1c cmp rcx,r9
            fl = sub::<64>(rcx, r9).1;
            // 57e1f jne 0x57df7
            if fl.ne() { pc = 0x57df7; continue; }
            pc = 0x57e21;
        }
        0x57e21 => {
            // block 57e21
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57e21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57e21 jmp 0x58019
            pc = 0x58019; continue;
        }
        0x57e26 => {
            // block 57e26
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57e26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57e26 cmp rcx,r8
            fl = sub::<64>(rcx, r8).1;
            // 57e29 je 0x58019
            if fl.e() { pc = 0x58019; continue; }
            pc = 0x57e2f;
        }
        0x57e2f => {
            // block 57e2f
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57e2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57e2f cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 57e32 jne 0x58019
            if fl.ne() { pc = 0x58019; continue; }
            pc = 0x57e38;
        }
        0x57e38 => {
            // block 57e38
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57e38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57e38 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 57e40 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 57e47 mov r12,QWORD PTR [rip+0xe8562] # 0x1403b0
            r12 = m.r64(m.env.data_va(347005)) as u64;
            // 57e4e lea rbx,[r12+0x40]
            rbx = r12.wrapping_add(0x40);
            // 57e53 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // 57e5a lea r13,[rbp-0x160]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 57e61 mov rax,QWORD PTR [rip+0xe8358] # 0x1401c0
            rax = m.r64(m.env.data_va(346509)) as u64;
            // 57e68 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 57e6c mov QWORD PTR [rbp-0x178],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rcx) as u64);
            // 57e73 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 57e77 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 57e7e mov QWORD PTR [rbp-0x170],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (0x0u64) as u64);
            // 57e89 mov rsi,r13
            rsi = r13;
            // 57e8c call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57e91 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 57e99 mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // 57ea0 lea r14,[r12+0x18]
            r14 = r12.wrapping_add(0x18);
            // 57ea5 mov QWORD PTR [rbp-0x178],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (r14) as u64);
            // 57eac add r12,0x68
            { let (r, f) = add::<64>(r12, 0x68u64);
              r12 = r;
              fl = f; }
            // 57eb0 mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // 57eb7 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // 57ebe mov rdi,r13
            rdi = r13;
            // 57ec1 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57ec6 mov r15,QWORD PTR [rip+0xe84db] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // 57ecd add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // 57ed1 mov QWORD PTR [rbp-0x160],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r15) as u64);
            // 57ed8 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 57ee3 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 57eee mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 57ef9 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 57f04 mov DWORD PTR [rbp-0x100],0x18
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x18u64) as u32);
            // 57f0e xorps xmm0,xmm0
            x0 = 0;
            // 57f11 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 57f15 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 57f1d lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 57f21 mov rdi,r13
            rdi = r13;
            // 57f24 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57f29 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 57f2d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57f32 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 57f39 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 57f3d lea rcx,[rbp-0x178]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 57f44 mov edx,DWORD PTR [rax+rcx*1+0x18]
            rdx = (m.r32(rax.wrapping_add(rcx).wrapping_add(0x18)) as u64) as u32 as u64;
            // 57f48 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 57f4b or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 57f4e mov DWORD PTR [rax+rcx*1+0x18],edx
            m.w32(rax.wrapping_add(rcx).wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // 57f52 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 57f59 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 57f5d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 57f62 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // 57f67 mov r8,rsi
            r8 = rsi;
            // 57f6a call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57f6f lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 57f76 movzx edx,BYTE PTR [rbp-0x190]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64) as u32 as u64;
            // 57f7d test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 57f80 jne 0x57f8e
            if fl.ne() { pc = 0x57f8e; continue; }
            pc = 0x57f82;
        }
        0x57f82 => {
            // block 57f82
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57f82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57f82 lea rsi,[rbp-0x18f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe71);
            // 57f89 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 57f8c jmp 0x57f9c
            pc = 0x57f9c; continue;
        }
        0x57f8e => {
            // block 57f8e
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57f8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57f8e mov rdx,QWORD PTR [rbp-0x188]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 57f95 mov rsi,QWORD PTR [rbp-0x180]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            pc = 0x57f9c;
        }
        0x57f9c => {
            // block 57f9c
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 57f9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 57f9c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57fa1 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 57fa8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57fad lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 57fb4 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 57fb8 call 0xec062
            { let r = { let t = m.env.stub_va(0x82f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57fbd mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 57fc1 call 0x523cc
            { let r = { let t = m.env.code_va(0x523cc); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57fc6 mov QWORD PTR [rbp-0x178],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (r14) as u64);
            // 57fcd mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // 57fd4 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // 57fdb mov QWORD PTR [rbp-0x160],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r15) as u64);
            // 57fe2 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 57fe9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57fee mov rdi,r13
            rdi = r13;
            // 57ff1 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57ff6 mov rsi,QWORD PTR [rip+0xe81fb] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // 57ffd add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 58001 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 58008 call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5800d lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 58014 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x58019;
        }
        0x58019 => {
            // block 58019
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 58019 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58019 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 5801d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58022 add rsp,0x178
            { let (r, f) = add::<64>(rsp, 0x178u64);
              rsp = r;
              fl = f; }
            // 58029 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5802a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5802c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5802e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58030 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58032 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58033 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x58034 => {
            // block 58034
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 58034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58034 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // 5803b jmp 0x58074
            pc = 0x58074; continue;
        }
        0x5803d => {
            // block 5803d
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 5803d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5803d mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // 58044 jmp 0x580bb
            pc = 0x580bb; continue;
        }
        0x58046 => {
            // block 58046
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 58046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58046 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // 5804d jmp 0x580a4
            pc = 0x580a4; continue;
        }
        0x5804f => {
            // block 5804f
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 5804f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5804f mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // 58056 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5805a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5805f jmp 0x58090
            pc = 0x58090; continue;
        }
        0x58061 => {
            // block 58061
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 58061 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58061 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // 58068 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 5806f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x58074;
        }
        0x58074 => {
            // block 58074
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 58074 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58074 mov QWORD PTR [rbp-0x178],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (r14) as u64);
            // 5807b mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // 58082 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // 58089 mov QWORD PTR [rbp-0x160],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r15) as u64);
            pc = 0x58090;
        }
        0x58090 => {
            // block 58090
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 58090 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58090 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 58097 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5809c mov rdi,r13
            rdi = r13;
            // 5809f call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x580a4;
        }
        0x580a4 => {
            // block 580a4
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 580a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 580a4 mov rsi,QWORD PTR [rip+0xe814d] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // 580ab add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 580af lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 580b6 call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x580bb;
        }
        0x580bb => {
            // block 580bb
            #[cfg(feature = "trace")] eprintln!("f_00057d4a pc 580bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 580bb lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 580c2 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 580c7 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 580cb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 580d0 mov rdi,QWORD PTR [rbp-0x198]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            // 580d7 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd582c size=900
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d582c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd582c;
    loop {
        match pc {
        0xd582c => {
            // block d582c
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d582c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d582c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d582d mov rbp,rsp
            rbp = rsp;
            // d5830 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d5832 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d5834 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d5836 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d5838 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5839 sub rsp,0x348
            { let (r, f) = sub::<64>(rsp, 0x348u64);
              rsp = r;
              fl = f; }
            // d5840 mov r14,rsi
            r14 = rsi;
            // d5843 mov rbx,rdi
            rbx = rdi;
            // d5846 mov r13,QWORD PTR [rip+0x6ab8b] # 0x1403d8
            r13 = m.r64(m.env.data_va(347045)) as u64;
            // d584d mov r13,QWORD PTR [r13+0x0]
            r13 = m.r64(r13) as u64;
            // d5851 mov QWORD PTR [rbp-0x30],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r13) as u64);
            // d5855 mov edi,0x100
            rdi = (0x100u64) as u32 as u64;
            // d585a call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d585f mov QWORD PTR [rbp-0x360],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca0), (rax) as u64);
            // d5866 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d5869 call 0xcdbbf
            { let r = { let t = m.env.code_va(0xcdbbf); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d586e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5870 je 0xd589f
            if fl.e() { pc = 0xd589f; continue; }
            pc = 0xd5872;
        }
        0xd5872 => {
            // block d5872
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5872 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5872 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5875 lea rdx,[rbp-0x32c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fcd4);
            // d587c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d587f mov esi,0x40002
            rsi = (0x40002u64) as u32 as u64;
            // d5884 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5889 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d588c mov rdi,rbx
            rdi = rbx;
            // d588f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5892 cmp DWORD PTR [rbp-0x32c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcd4)) as u64, 0x1u64).1;
            // d5899 jne 0xd5b68
            if fl.ne() { pc = 0xd5b68; continue; }
            pc = 0xd589f;
        }
        0xd589f => {
            // block d589f
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d589f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d589f mov QWORD PTR [rbp-0x368],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc98), (r14) as u64);
            // d58a6 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d58a9 call 0xcdb57
            { let r = { let t = m.env.code_va(0xcdb57); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58ae test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d58b0 jne 0xd58ca
            if fl.ne() { pc = 0xd58ca; continue; }
            pc = 0xd58b2;
        }
        0xd58b2 => {
            // block d58b2
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d58b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d58b2 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d58b5 call 0xcdbbf
            { let r = { let t = m.env.code_va(0xcdbbf); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58ba test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d58bc jne 0xd58ca
            if fl.ne() { pc = 0xd58ca; continue; }
            pc = 0xd58be;
        }
        0xd58be => {
            // block d58be
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d58be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d58be mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d58c1 call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58c6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d58c8 je 0xd5949
            if fl.e() { pc = 0xd5949; continue; }
            pc = 0xd58ca;
        }
        0xd58ca => {
            // block d58ca
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d58ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d58ca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d58cd mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d58d2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d58d4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d58d9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d58dc mov rdi,rbx
            rdi = rbx;
            // d58df call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58e2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d58e5 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d58ea mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // d58ef mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d58f4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d58f7 mov rdi,rbx
            rdi = rbx;
            // d58fa call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58fd mov r14d,0x3e8
            r14 = (0x3e8u64) as u32 as u64;
            // d5903 lea r12,[rbp-0x330]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fcd0);
            // d590a mov r15d,0x100
            r15 = (0x100u64) as u32 as u64;
            pc = 0xd5910;
        }
        0xd5910 => {
            // block d5910
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5910 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5910 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5913 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d5918 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d591d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5920 mov rdi,rbx
            rdi = rbx;
            // d5923 mov rdx,r12
            rdx = r12;
            // d5926 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5929 mov eax,DWORD PTR [rbp-0x330]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64) as u32 as u64;
            // d592f and eax,r15d
            { let r = (rax as u32 as u64) & (r15 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d5932 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d5934 shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d5937 sub r14d,ecx
            { let (r, f) = sub::<32>((r14 as u32 as u64), (rcx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // d593a je 0xd5940
            if fl.e() { pc = 0xd5940; continue; }
            pc = 0xd593c;
        }
        0xd593c => {
            // block d593c
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d593c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d593c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d593e jne 0xd5910
            if fl.ne() { pc = 0xd5910; continue; }
            pc = 0xd5940;
        }
        0xd5940 => {
            // block d5940
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5940 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5940 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // d5943 je 0xd5a60
            if fl.e() { pc = 0xd5a60; continue; }
            pc = 0xd5949;
        }
        0xd5949 => {
            // block d5949
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5949 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d594c lea r12,[rbp-0x334]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fccc);
            // d5953 mov r15d,0x100
            r15 = (0x100u64) as u32 as u64;
            // d5959 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd595b;
        }
        0xd595b => {
            // block d595b
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d595b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d595b mov QWORD PTR [rbp-0x358],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca8), (rdx) as u64);
            // d5962 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5965 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d596a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d596f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5972 mov rdi,rbx
            rdi = rbx;
            // d5975 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5978 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d597b mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d5980 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d5985 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d598a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d598d mov rdi,rbx
            rdi = rbx;
            // d5990 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5993 mov r13d,0x3e8
            r13 = (0x3e8u64) as u32 as u64;
            pc = 0xd5999;
        }
        0xd5999 => {
            // block d5999
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5999 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5999 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d599c mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d59a1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d59a6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d59a9 mov rdi,rbx
            rdi = rbx;
            // d59ac mov rdx,r12
            rdx = r12;
            // d59af call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d59b2 mov eax,DWORD PTR [rbp-0x334]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fccc)) as u64) as u32 as u64;
            // d59b8 and eax,r15d
            { let r = (rax as u32 as u64) & (r15 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d59bb mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d59bd shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d59c0 sub r13d,ecx
            { let (r, f) = sub::<32>((r13 as u32 as u64), (rcx as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // d59c3 je 0xd59c9
            if fl.e() { pc = 0xd59c9; continue; }
            pc = 0xd59c5;
        }
        0xd59c5 => {
            // block d59c5
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d59c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d59c5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d59c7 jne 0xd5999
            if fl.ne() { pc = 0xd5999; continue; }
            pc = 0xd59c9;
        }
        0xd59c9 => {
            // block d59c9
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d59c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d59c9 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // d59cc je 0xd5a4d
            if fl.e() { pc = 0xd5a4d; continue; }
            pc = 0xd59ce;
        }
        0xd59ce => {
            // block d59ce
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d59ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d59ce mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d59d1 mov rcx,QWORD PTR [rbp-0x360]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca0)) as u64;
            // d59d8 lea rdx,[rcx+r14*4]
            rdx = rcx.wrapping_add(r14.wrapping_mul(4));
            // d59dc mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d59e1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d59e6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d59e9 mov rdi,rbx
            rdi = rbx;
            // d59ec call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d59ef inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // d59f2 mov rdx,QWORD PTR [rbp-0x358]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca8)) as u64;
            // d59f9 add edx,0x4
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x4u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d59fc cmp r14,0x40
            fl = sub::<64>(r14, 0x40u64).1;
            // d5a00 jb 0xd595b
            if fl.b() { pc = 0xd595b; continue; }
            pc = 0xd5a06;
        }
        0xd5a06 => {
            // block d5a06
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a06 lea rbx,[rbp-0x328]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // d5a0d mov rdi,rbx
            rdi = rbx;
            // d5a10 call 0x12b8c
            { let r = { let t = m.env.code_va(0x12b8c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5a15 lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // d5a1c mov rsi,rbx
            rsi = rbx;
            // d5a1f mov rdx,QWORD PTR [rbp-0x360]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca0)) as u64;
            // d5a26 call 0x12e88
            { let r = { let t = m.env.code_va(0x12e88); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5a2b mov rbx,QWORD PTR [rbp-0x368]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc98)) as u64;
            // d5a32 movzx eax,BYTE PTR [rbp-0x350]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64) as u32 as u64;
            // d5a39 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5a3b mov r13,QWORD PTR [rip+0x6a996] # 0x1403d8
            r13 = m.r64(m.env.data_va(347045)) as u64;
            // d5a42 mov r13,QWORD PTR [r13+0x0]
            r13 = m.r64(r13) as u64;
            // d5a46 jne 0xd5a68
            if fl.ne() { pc = 0xd5a68; continue; }
            pc = 0xd5a48;
        }
        0xd5a48 => {
            // block d5a48
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a48 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // d5a4b jmp 0xd5a6f
            pc = 0xd5a6f; continue;
        }
        0xd5a4d => {
            // block d5a4d
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a4d xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d5a50 mov r13,QWORD PTR [rip+0x6a981] # 0x1403d8
            r13 = m.r64(m.env.data_va(347045)) as u64;
            // d5a57 mov r13,QWORD PTR [r13+0x0]
            r13 = m.r64(r13) as u64;
            // d5a5b jmp 0xd5b68
            pc = 0xd5b68; continue;
        }
        0xd5a60 => {
            // block d5a60
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a60 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d5a63 jmp 0xd5b68
            pc = 0xd5b68; continue;
        }
        0xd5a68 => {
            // block d5a68
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a68 mov rax,QWORD PTR [rbp-0x348]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb8)) as u64;
            pc = 0xd5a6f;
        }
        0xd5a6f => {
            // block d5a6f
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a6f test rax,rax
            fl = logic::<64>(rax & rax);
            // d5a72 jne 0xd5b26
            if fl.ne() { pc = 0xd5b26; continue; }
            pc = 0xd5a78;
        }
        0xd5a78 => {
            // block d5a78
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a78 lea rdi,[rbx+0x10]
            rdi = rbx.wrapping_add(0x10);
            // d5a7c test BYTE PTR [rbp-0xc0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64 & 0x1u64);
            // d5a83 jne 0xd5a91
            if fl.ne() { pc = 0xd5a91; continue; }
            pc = 0xd5a85;
        }
        0xd5a85 => {
            // block d5a85
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a85 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // d5a8c inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // d5a8f jmp 0xd5a98
            pc = 0xd5a98; continue;
        }
        0xd5a91 => {
            // block d5a91
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a91 mov rsi,QWORD PTR [rbp-0xb0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            pc = 0xd5a98;
        }
        0xd5a98 => {
            // block d5a98
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a98 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // d5a9d call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5aa2 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // d5aa6 test BYTE PTR [rbp-0xa8],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64 & 0x1u64);
            // d5aad jne 0xd5abb
            if fl.ne() { pc = 0xd5abb; continue; }
            pc = 0xd5aaf;
        }
        0xd5aaf => {
            // block d5aaf
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5aaf lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d5ab6 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // d5ab9 jmp 0xd5ac2
            pc = 0xd5ac2; continue;
        }
        0xd5abb => {
            // block d5abb
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5abb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5abb mov rsi,QWORD PTR [rbp-0x98]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            pc = 0xd5ac2;
        }
        0xd5ac2 => {
            // block d5ac2
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5ac2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ac2 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // d5ac7 call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5acc lea rdi,[rbx+0x30]
            rdi = rbx.wrapping_add(0x30);
            // d5ad0 test BYTE PTR [rbp-0x90],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64 & 0x1u64);
            // d5ad7 jne 0xd5ae5
            if fl.ne() { pc = 0xd5ae5; continue; }
            pc = 0xd5ad9;
        }
        0xd5ad9 => {
            // block d5ad9
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5ad9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ad9 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d5ae0 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // d5ae3 jmp 0xd5ae9
            pc = 0xd5ae9; continue;
        }
        0xd5ae5 => {
            // block d5ae5
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5ae5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ae5 mov rsi,QWORD PTR [rbp-0x80]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            pc = 0xd5ae9;
        }
        0xd5ae9 => {
            // block d5ae9
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5ae9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ae9 mov edx,0x64
            rdx = (0x64u64) as u32 as u64;
            // d5aee call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5af3 lea rdi,[rbx+0x98]
            rdi = rbx.wrapping_add(0x98);
            // d5afa test BYTE PTR [rbp-0x78],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64 & 0x1u64);
            // d5afe jne 0xd5b09
            if fl.ne() { pc = 0xd5b09; continue; }
            pc = 0xd5b00;
        }
        0xd5b00 => {
            // block d5b00
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b00 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d5b04 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // d5b07 jmp 0xd5b0d
            pc = 0xd5b0d; continue;
        }
        0xd5b09 => {
            // block d5b09
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b09 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0xd5b0d;
        }
        0xd5b0d => {
            // block d5b0d
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b0d mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // d5b12 call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5b17 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // d5b1e call 0x1369c
            { let r = { let t = m.env.code_va(0x1369c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5b23 mov DWORD PTR [rbx+0xc],eax
            m.w32(rbx.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            pc = 0xd5b26;
        }
        0xd5b26 => {
            // block d5b26
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b26 mov rdi,QWORD PTR [rbp-0x360]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca0)) as u64;
            // d5b2d call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5b32 movzx eax,BYTE PTR [rbp-0x350]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64) as u32 as u64;
            // d5b39 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5b3b jne 0xd5b42
            if fl.ne() { pc = 0xd5b42; continue; }
            pc = 0xd5b3d;
        }
        0xd5b3d => {
            // block d5b3d
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b3d shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // d5b40 jmp 0xd5b49
            pc = 0xd5b49; continue;
        }
        0xd5b42 => {
            // block d5b42
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b42 mov rax,QWORD PTR [rbp-0x348]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb8)) as u64;
            pc = 0xd5b49;
        }
        0xd5b49 => {
            // block d5b49
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b49 test rax,rax
            fl = logic::<64>(rax & rax);
            // d5b4c sete r12b
            r12 = (r12 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d5b50 lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // d5b57 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5b5c lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // d5b63 call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd5b68;
        }
        0xd5b68 => {
            // block d5b68
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b68 cmp r13,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // d5b6c jne 0xd5b83
            if fl.ne() { pc = 0xd5b83; continue; }
            pc = 0xd5b6e;
        }
        0xd5b6e => {
            // block d5b6e
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b6e mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // d5b71 add rsp,0x348
            { let (r, f) = add::<64>(rsp, 0x348u64);
              rsp = r;
              fl = f; }
            // d5b78 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5b79 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d5b7b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d5b7d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d5b7f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d5b81 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5b82 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd5b83 => {
            // block d5b83
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b83 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5b88 mov rbx,rax
            rbx = rax;
            // d5b8b jmp 0xd5b9c
            pc = 0xd5b9c; continue;
        }
        0xd5b8d => {
            // block d5b8d
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b8d mov rbx,rax
            rbx = rax;
            // d5b90 lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // d5b97 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd5b9c;
        }
        0xd5b9c => {
            // block d5b9c
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b9c lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // d5ba3 call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5ba8 mov rdi,rbx
            rdi = rbx;
            // d5bab call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa8e86 size=888
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a8e86<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xa8e86;
    loop {
        match pc {
        0xa8e86 => {
            // block a8e86
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a8e86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a8e86 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a8e87 mov rbp,rsp
            rbp = rsp;
            // a8e8a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a8e8c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a8e8e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // a8e90 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // a8e92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a8e93 sub rsp,0x158
            { let (r, f) = sub::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // a8e9a mov DWORD PTR [rbp-0x15c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea4), ((rcx as u32 as u64)) as u32);
            // a8ea0 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // a8ea3 mov r12,rdi
            r12 = rdi;
            // a8ea6 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a8ead lea r15,[rbp-0x150]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a8eb4 mov rax,QWORD PTR [rip+0x97315] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // a8ebb lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // a8ebf mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // a8ec6 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // a8eca mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a8ed1 mov rsi,r15
            rsi = r15;
            // a8ed4 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8ed9 mov QWORD PTR [rbp-0x180],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (r12) as u64);
            // a8ee0 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // a8ee8 mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // a8eef mov rbx,QWORD PTR [rip+0x974ca] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // a8ef6 lea r12,[rbx+0x18]
            r12 = rbx.wrapping_add(0x18);
            // a8efa mov QWORD PTR [rbp-0x158],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r12) as u64);
            // a8f01 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // a8f05 mov QWORD PTR [rbp-0xe8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rbx) as u64);
            // a8f0c mov rdi,r15
            rdi = r15;
            // a8f0f call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8f14 mov QWORD PTR [rbp-0x170],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (r12) as u64);
            // a8f1b mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // a8f22 mov rax,QWORD PTR [rip+0x9747f] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // a8f29 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // a8f2d mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // a8f34 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a8f3b lea rbx,[rbp-0x110]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a8f42 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // a8f4d mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // a8f58 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // a8f63 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // a8f6e mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // a8f78 xorps xmm0,xmm0
            x0 = 0;
            // a8f7b movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // a8f7f mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // a8f87 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a8f8b mov rdi,r15
            rdi = r15;
            // a8f8e call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8f93 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a8f97 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8f9c cmp r14d,0x17
            fl = sub::<32>((r14 as u32 as u64), 0x17u64).1;
            // a8fa0 je 0xa8faf
            if fl.e() { pc = 0xa8faf; continue; }
            pc = 0xa8fa2;
        }
        0xa8fa2 => {
            // block a8fa2
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a8fa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a8fa2 cmp r14d,0x11a
            fl = sub::<32>((r14 as u32 as u64), 0x11au64).1;
            // a8fa9 jne 0xa90c1
            if fl.ne() { pc = 0xa90c1; continue; }
            pc = 0xa8faf;
        }
        0xa8faf => {
            // block a8faf
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a8faf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a8faf xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // a8fb2 lea r14,[rbp-0x158]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a8fb9 lea r13,[rbp-0x30]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0xa8fbd;
        }
        0xa8fbd => {
            // block a8fbd
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a8fbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a8fbd mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // a8fc2 mov rdi,r14
            rdi = r14;
            // a8fc5 lea rsi,[rip+0x8b47a] # 0x134446
            rsi = m.env.data_va(298003);
            // a8fcc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8fd1 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // a8fd4 and esi,0xfffe
            { let r = (rsi as u32 as u64) & 0xfffeu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // a8fda shr esi,1
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // a8fdc mov rdi,rax
            rdi = rax;
            // a8fdf call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8fe4 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // a8fe9 mov rdi,rax
            rdi = rax;
            // a8fec lea rsi,[rip+0x8b45a] # 0x13444d
            rsi = m.env.data_va(298010);
            // a8ff3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8ff8 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // a8ffb and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a8ffe lea rsi,[rip+0x8b453] # 0x134458
            rsi = m.env.data_va(298021);
            // a9005 lea rcx,[rip+0x8b446] # 0x134452
            rcx = m.env.data_va(298015);
            // a900c cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a9010 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // a9015 mov rdi,rax
            rdi = rax;
            // a9018 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a901d mov ecx,DWORD PTR [rbp-0x15c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea4)) as u64) as u32 as u64;
            // a9023 bt ecx,r12d
            fl.cf = bit::<32>((rcx as u32 as u64), (r12 as u32 as u64));
            // a9027 lea rsi,[rip+0x8b438] # 0x134466
            rsi = m.env.data_va(298035);
            // a902e lea rcx,[rip+0x8b429] # 0x13445e
            rcx = m.env.data_va(298027);
            // a9035 cmovb rsi,rcx
            if fl.b() { rsi = rcx; }
            // a9039 sbb rdx,rdx
            { let (r, f) = sbb::<64>(rdx, rdx, fl.cf);
              rdx = r;
              fl = f; }
            // a903c and rdx,0x1
            { let r = rdx & 0x1u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a9040 or rdx,0x6
            { let r = rdx | 0x6u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a9044 mov rdi,rax
            rdi = rax;
            // a9047 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a904c inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // a904f cmp r12d,0x7
            fl = sub::<32>((r12 as u32 as u64), 0x7u64).1;
            // a9053 ja 0xa90c1
            if fl.a() { pc = 0xa90c1; continue; }
            pc = 0xa9055;
        }
        0xa9055 => {
            // block a9055
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a9055 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a9055 mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // a905c mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a9060 add rsi,r14
            { let (r, f) = add::<64>(rsi, r14);
              rsi = r;
              fl = f; }
            // a9063 mov rdi,r13
            rdi = r13;
            // a9066 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a906b mov rdi,r13
            rdi = r13;
            // a906e mov rsi,QWORD PTR [rip+0x9711b] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a9075 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a907a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a907d mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a9081 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a9086 mov rdi,rax
            rdi = rax;
            // a9089 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a908b mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a908e mov rdi,r13
            rdi = r13;
            // a9091 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a9096 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // a909a mov rdi,r14
            rdi = r14;
            // a909d call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a90a2 mov rdi,r14
            rdi = r14;
            // a90a5 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a90aa jmp 0xa8fbd
            pc = 0xa8fbd; continue;
        }
        0xa90af => {
            // block a90af
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a90af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a90af mov r15,rbx
            r15 = rbx;
            // a90b2 mov r14,rax
            r14 = rax;
            // a90b5 lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a90bc jmp 0xa9165
            pc = 0xa9165; continue;
        }
        0xa90c1 => {
            // block a90c1
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a90c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a90c1 mov r15,rbx
            r15 = rbx;
            // a90c4 mov r14,QWORD PTR [rbp-0x180]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // a90cb mov rdi,r14
            rdi = r14;
            // a90ce lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a90d5 mov rsi,rbx
            rsi = rbx;
            // a90d8 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a90dd mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a90e4 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a90eb mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a90f2 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a90f9 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // a9100 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a9107 mov rdi,r15
            rdi = r15;
            // a910a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a910f mov rdi,rbx
            rdi = rbx;
            // a9112 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a9117 mov rsi,QWORD PTR [rip+0x970ea] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a911e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a9122 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a9129 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a912e lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a9135 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a913a mov rax,r14
            rax = r14;
            // a913d add rsp,0x158
            { let (r, f) = add::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // a9144 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // a9145 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // a9147 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // a9149 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // a914b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // a914d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a914e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xa914f => {
            // block a914f
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a914f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a914f mov r15,rbx
            r15 = rbx;
            // a9152 mov r14,rax
            r14 = rax;
            // a9155 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a9159 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a915e lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            pc = 0xa9165;
        }
        0xa9165 => {
            // block a9165
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a9165 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a9165 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a916c mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a9173 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a917a mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a9181 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // a9188 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a918f mov rdi,r15
            rdi = r15;
            // a9192 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a9197 mov rdi,rbx
            rdi = rbx;
            // a919a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa919f;
        }
        0xa919f => {
            // block a919f
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a919f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a919f mov rsi,QWORD PTR [rip+0x97062] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a91a6 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a91aa lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a91b1 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa91b6;
        }
        0xa91b6 => {
            // block a91b6
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a91b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a91b6 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a91bd call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91c2 mov rdi,r14
            rdi = r14;
            // a91c5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91ca mov r14,rax
            r14 = rax;
            // a91cd jmp 0xa91b6
            pc = 0xa91b6; continue;
        }
        0xa91cf => {
            // block a91cf
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a91cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a91cf mov r14,rax
            r14 = rax;
            // a91d2 jmp 0xa919f
            pc = 0xa919f; continue;
        }
        0xa91d4 => {
            // block a91d4
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a91d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a91d4 mov r14,rax
            r14 = rax;
            // a91d7 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a91db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91e0 mov rdi,rbx
            rdi = rbx;
            // a91e3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91e8 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a91ef call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91f4 jmp 0xa919f
            pc = 0xa919f; continue;
        }
        0xa91f6 => {
            // block a91f6
            #[cfg(feature = "trace")] eprintln!("f_000a8e86 pc a91f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a91f6 mov r14,rax
            r14 = rax;
            // a91f9 jmp 0xa9165
            pc = 0xa9165; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe00a1 size=867
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e00a1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe00a1;
    loop {
        match pc {
        0xe00a1 => {
            // block e00a1
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e00a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e00a1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e00a2 mov rbp,rsp
            rbp = rsp;
            // e00a5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e00a7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e00a9 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e00ab push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e00ad push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e00ae sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e00b2 mov DWORD PTR [rbp-0x38],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rcx as u32 as u64)) as u32);
            // e00b5 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // e00b7 mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // e00bb mov r14,rdi
            r14 = rdi;
            // e00be lea eax,[rbx+0x4]
            rax = (rbx.wrapping_add(0x4)) as u32 as u64;
            // e00c1 movsxd rdi,eax
            rdi = sx::<32>((rax as u32 as u64)) as u64;
            // e00c4 add rdi,rdi
            { let (r, f) = add::<64>(rdi, rdi);
              rdi = r;
              fl = f; }
            // e00c7 mov r15,0xffffffffffffffff
            r15 = 0xffff_ffff_ffff_ffffu64;
            // e00ce cmovb rdi,r15
            if fl.b() { rdi = r15; }
            // e00d2 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e00d7 mov r13,rax
            r13 = rax;
            // e00da mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e00dc shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e00df add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e00e1 sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e00e3 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e00e7 lea eax,[rax+0x4]
            rax = (rax.wrapping_add(0x4)) as u32 as u64;
            // e00ea movsxd r12,eax
            r12 = sx::<32>((rax as u32 as u64)) as u64;
            // e00ed add r12,r12
            { let (r, f) = add::<64>(r12, r12);
              r12 = r;
              fl = f; }
            // e00f0 cmovb r12,r15
            if fl.b() { r12 = r15; }
            // e00f4 mov rdi,r12
            rdi = r12;
            // e00f7 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e00fc mov r15,rax
            r15 = rax;
            // e00ff mov rdi,r12
            rdi = r12;
            // e0102 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0107 mov r12,rax
            r12 = rax;
            // e010a test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // e010c jle 0xe0159
            if fl.le() { pc = 0xe0159; continue; }
            pc = 0xe010e;
        }
        0xe010e => {
            // block e010e
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e010e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e010e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0110 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe0112;
        }
        0xe0112 => {
            // block e0112
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0112 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0112 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // e0114 shr esi,0x1f
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1fu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e0117 add esi,ecx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rcx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0119 sar esi,1
            { let (r, f) = sar::<32>((rsi as u32 as u64), 0x1u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e011b inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e011d test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // e0120 movsxd rdx,eax
            rdx = sx::<32>((rax as u32 as u64)) as u64;
            // e0123 movzx edx,WORD PTR [r14+rdx*2]
            rdx = (m.r16(r14.wrapping_add(rdx.wrapping_mul(2))) as u64) as u32 as u64;
            // e0128 movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // e012b jne 0xe0134
            if fl.ne() { pc = 0xe0134; continue; }
            pc = 0xe012d;
        }
        0xe012d => {
            // block e012d
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e012d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e012d mov WORD PTR [r15+rsi*2],dx
            m.w16(r15.wrapping_add(rsi.wrapping_mul(2)), ((rdx as u16 as u64)) as u16);
            // e0132 jmp 0xe0139
            pc = 0xe0139; continue;
        }
        0xe0134 => {
            // block e0134
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0134 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0134 mov WORD PTR [r12+rsi*2],dx
            m.w16(r12.wrapping_add(rsi.wrapping_mul(2)), ((rdx as u16 as u64)) as u16);
            pc = 0xe0139;
        }
        0xe0139 => {
            // block e0139
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0139 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0139 lea edx,[rax+0x1]
            rdx = (rax.wrapping_add(0x1)) as u32 as u64;
            // e013c movsxd rdx,edx
            rdx = sx::<32>((rdx as u32 as u64)) as u64;
            // e013f mov dx,WORD PTR [r14+rdx*2]
            rdx = (rdx & !0xffff) | ((m.r16(r14.wrapping_add(rdx.wrapping_mul(2))) as u64) as u16 as u64);
            // e0144 mov WORD PTR [r13+rax*1+0x2],dx
            m.w16(r13.wrapping_add(rax).wrapping_add(0x2), ((rdx as u16 as u64)) as u16);
            // e014a inc ecx
            { let (r, f) = inc::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // e014c add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // e0150 cmp ebx,ecx
            fl = sub::<32>((rbx as u32 as u64), (rcx as u32 as u64)).1;
            // e0152 jne 0xe0112
            if fl.ne() { pc = 0xe0112; continue; }
            pc = 0xe0154;
        }
        0xe0154 => {
            // block e0154
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0154 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0154 movzx eax,WORD PTR [r13+0x2]
            rax = (m.r16(r13.wrapping_add(0x2)) as u64) as u32 as u64;
            pc = 0xe0159;
        }
        0xe0159 => {
            // block e0159
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0159 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0159 mov WORD PTR [r13+0x0],ax
            m.w16(r13, ((rax as u16 as u64)) as u16);
            // e015e movsxd rax,ebx
            rax = sx::<32>((rbx as u32 as u64)) as u64;
            // e0161 mov ax,WORD PTR [r13+rax*2+0x0]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // e0167 lea ecx,[rbx+0x2]
            rcx = (rbx.wrapping_add(0x2)) as u32 as u64;
            // e016a movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e016d mov WORD PTR [r13+rcx*2+0x0],ax
            m.w16(r13.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e0173 lea ecx,[rbx+0x1]
            rcx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // e0176 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e0179 mov WORD PTR [r13+rcx*2+0x0],ax
            m.w16(r13.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e017f shl ebx,0x10
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0182 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e0184 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // e0185 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // e0188 idiv ecx
            { let (q, r) = idiv::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // e018a mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // e018d test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // e018f mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // e0193 jle 0xe0244
            if fl.le() { pc = 0xe0244; continue; }
            pc = 0xe0199;
        }
        0xe0199 => {
            // block e0199
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0199 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0199 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // e019e xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e01a1 lea r9,[rip+0x47e78] # 0x128020
            r9 = m.env.data_va(247789);
            // e01a8 mov r8w,0x4
            r8 = (r8 & !0xffff) | ((0x4u64) as u16 as u64);
            // e01ad mov r10d,DWORD PTR [rbp-0x38]
            r10 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            pc = 0xe01b1;
        }
        0xe01b1 => {
            // block e01b1
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e01b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e01b1 mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // e01b4 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e01b7 mov edx,r11d
            rdx = ((r11 as u32 as u64)) as u32 as u64;
            // e01ba shr edx,0xb
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e01bd and edx,0x1f
            { let r = (rdx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e01c0 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e01c2 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e01c4 movsx ebx,WORD PTR [r13+rax*2-0x2]
            rbx = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e01ca mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // e01cf sub edi,edx
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rdx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e01d1 imul ebx,DWORD PTR [r9+rdi*4]
            { let (r, f) = imul::<32>((rbx as u32 as u64), m.r32(r9.wrapping_add(rdi.wrapping_mul(4))) as u64); rbx = (r) as u32 as u64; fl = f; }
            // e01d6 movsx edi,WORD PTR [r13+rax*2+0x0]
            rdi = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e01dc mov esi,0x40
            rsi = (0x40u64) as u32 as u64;
            // e01e1 sub esi,edx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e01e3 imul edi,DWORD PTR [r9+rsi*4]
            { let (r, f) = imul::<32>((rdi as u32 as u64), m.r32(r9.wrapping_add(rsi.wrapping_mul(4))) as u64); rdi = (r) as u32 as u64; fl = f; }
            // e01e8 add edi,ebx
            { let (r, f) = add::<32>((rdi as u32 as u64), (rbx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e01ea movsx esi,WORD PTR [r13+rax*2+0x2]
            rsi = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e01f0 mov ebx,0x60
            rbx = (0x60u64) as u32 as u64;
            // e01f5 sub ebx,edx
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rdx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e01f7 imul esi,DWORD PTR [r9+rbx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rbx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // e01fc add esi,edi
            { let (r, f) = add::<32>((rsi as u32 as u64), (rdi as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e01fe movsx eax,WORD PTR [r13+rax*2+0x4]
            rax = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e0204 mov edi,0x80
            rdi = (0x80u64) as u32 as u64;
            // e0209 sub edi,edx
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rdx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e020b imul eax,DWORD PTR [r9+rdi*4]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(r9.wrapping_add(rdi.wrapping_mul(4))) as u64); rax = (r) as u32 as u64; fl = f; }
            // e0210 add eax,esi
            { let (r, f) = add::<32>((rax as u32 as u64), (rsi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e0212 mov bx,0x3fb
            rbx = (rbx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e0216 cmp eax,0x3fbffff
            fl = sub::<32>((rax as u32 as u64), 0x3fbffffu64).1;
            // e021b ja 0xe022c
            if fl.a() { pc = 0xe022c; continue; }
            pc = 0xe021d;
        }
        0xe021d => {
            // block e021d
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e021d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e021d mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e021f shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0222 cmp eax,0x40000
            fl = sub::<32>((rax as u32 as u64), 0x40000u64).1;
            // e0227 cmovb bx,r8w
            if fl.b() { rbx = (rbx & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            pc = 0xe022c;
        }
        0xe022c => {
            // block e022c
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e022c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e022c movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e022f mov WORD PTR [r14+rcx*2],bx
            m.w16(r14.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16 as u64)) as u16);
            // e0234 add ecx,0x2
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x2u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0237 add r11d,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e023b dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // e023e jne 0xe01b1
            if fl.ne() { pc = 0xe01b1; continue; }
            pc = 0xe0244;
        }
        0xe0244 => {
            // block e0244
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0244 mov QWORD PTR [rbp-0x60],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (r13) as u64);
            // e0248 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // e024c mov ax,WORD PTR [r15+0x2]
            rax = (rax & !0xffff) | ((m.r16(r15.wrapping_add(0x2)) as u64) as u16 as u64);
            // e0251 mov WORD PTR [r15],ax
            m.w16(r15, ((rax as u16 as u64)) as u16);
            // e0255 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e0259 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // e025c mov cx,WORD PTR [r15+rax*2]
            rcx = (rcx & !0xffff) | ((m.r16(r15.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // e0261 lea edx,[rsi+0x2]
            rdx = (rsi.wrapping_add(0x2)) as u32 as u64;
            // e0264 movsxd rdx,edx
            rdx = sx::<32>((rdx as u32 as u64)) as u64;
            // e0267 mov WORD PTR [r15+rdx*2],cx
            m.w16(r15.wrapping_add(rdx.wrapping_mul(2)), ((rcx as u16 as u64)) as u16);
            // e026c inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e026e movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // e0271 mov WORD PTR [r15+rsi*2],cx
            m.w16(r15.wrapping_add(rsi.wrapping_mul(2)), ((rcx as u16 as u64)) as u16);
            // e0276 mov QWORD PTR [rbp-0x50],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r12) as u64);
            // e027a mov cx,WORD PTR [r12+0x2]
            rcx = (rcx & !0xffff) | ((m.r16(r12.wrapping_add(0x2)) as u64) as u16 as u64);
            // e0280 mov WORD PTR [r12],cx
            m.w16(r12, ((rcx as u16 as u64)) as u16);
            // e0285 mov ax,WORD PTR [r12+rax*2]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // e028a mov WORD PTR [r12+rdx*2],ax
            m.w16(r12.wrapping_add(rdx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e028f mov WORD PTR [r12+rsi*2],ax
            m.w16(r12.wrapping_add(rsi.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e0294 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // e0297 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // e029a jl 0xe03db
            if fl.l() { pc = 0xe03db; continue; }
            pc = 0xe02a0;
        }
        0xe02a0 => {
            // block e02a0
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e02a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e02a0 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e02a2 shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e02a5 add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e02a7 sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e02a9 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e02ab mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e02af mov r12d,0x2
            r12 = (0x2u64) as u32 as u64;
            // e02b5 xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e02b8 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe02bb;
        }
        0xe02bb => {
            // block e02bb
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e02bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e02bb mov r13,r14
            r13 = r14;
            // e02be mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // e02c1 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e02c4 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // e02c7 shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e02ca and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e02cd inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e02cf movsxd r14,eax
            r14 = sx::<32>((rax as u32 as u64)) as u64;
            // e02d2 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e02d6 movsx eax,WORD PTR [rsi+r14*2-0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e02dc mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // e02e1 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e02e3 lea rdx,[rip+0x47d36] # 0x128020
            rdx = m.env.data_va(247789);
            // e02ea mov rdi,rdx
            rdi = rdx;
            // e02ed mov ecx,DWORD PTR [rdi+rcx*4]
            rcx = (m.r32(rdi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e02f0 mov DWORD PTR [rbp-0x38],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rcx as u32 as u64)) as u32);
            // e02f3 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e02f6 movsx ecx,WORD PTR [rsi+r14*2]
            rcx = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e02fb mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // e0300 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0302 mov edx,DWORD PTR [rdi+rdx*4]
            rdx = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // e0305 mov DWORD PTR [rbp-0x40],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rdx as u32 as u64)) as u32);
            // e0308 imul ecx,edx
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rdx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // e030b add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e030d movsx eax,WORD PTR [rsi+r14*2+0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e0313 mov edx,0x60
            rdx = (0x60u64) as u32 as u64;
            // e0318 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e031a mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // e031e imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e0322 add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e0324 movsx r10d,WORD PTR [rsi+r14*2+0x4]
            r10 = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e032a mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // e032f sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0331 mov r9d,DWORD PTR [rdi+rcx*4]
            r9 = (m.r32(rdi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e0335 imul r10d,r9d
            { let (r, f) = imul::<32>((r10 as u32 as u64), (r9 as u32 as u64)); r10 = (r) as u32 as u64; fl = f; }
            // e0339 add r10d,eax
            { let (r, f) = add::<32>((r10 as u32 as u64), (rax as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // e033c mov bx,0x3fb
            rbx = (rbx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e0340 cmp r10d,0x3fbffff
            fl = sub::<32>((r10 as u32 as u64), 0x3fbffffu64).1;
            // e0347 mov ax,0x3fb
            rax = (rax & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e034b ja 0xe0362
            if fl.a() { pc = 0xe0362; continue; }
            pc = 0xe034d;
        }
        0xe034d => {
            // block e034d
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e034d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e034d mov eax,r10d
            rax = ((r10 as u32 as u64)) as u32 as u64;
            // e0350 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0353 cmp r10d,0x40000
            fl = sub::<32>((r10 as u32 as u64), 0x40000u64).1;
            // e035a mov cx,0x4
            rcx = (rcx & !0xffff) | ((0x4u64) as u16 as u64);
            // e035e cmovb ax,cx
            if fl.b() { rax = (rax & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            pc = 0xe0362;
        }
        0xe0362 => {
            // block e0362
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0362 lea ecx,[r12-0x2]
            rcx = (r12.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // e0367 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e036a mov WORD PTR [r13+rcx*2+0x0],ax
            m.w16(r13.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e0370 mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e0374 movsx eax,WORD PTR [rsi+r14*2-0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e037a imul eax,DWORD PTR [rbp-0x38]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64); rax = (r) as u32 as u64; fl = f; }
            // e037e movsx ecx,WORD PTR [rsi+r14*2]
            rcx = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e0383 imul ecx,DWORD PTR [rbp-0x40]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // e0387 add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0389 movsx edx,WORD PTR [rsi+r14*2+0x2]
            rdx = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e038f imul edx,r8d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r8 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // e0393 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0395 movsx eax,WORD PTR [rsi+r14*2+0x4]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e039b imul eax,r9d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r9 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e039f add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e03a1 cmp eax,0x3fbffff
            fl = sub::<32>((rax as u32 as u64), 0x3fbffffu64).1;
            // e03a6 ja 0xe03ba
            if fl.a() { pc = 0xe03ba; continue; }
            pc = 0xe03a8;
        }
        0xe03a8 => {
            // block e03a8
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e03a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e03a8 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e03aa shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e03ad cmp eax,0x40000
            fl = sub::<32>((rax as u32 as u64), 0x40000u64).1;
            // e03b2 mov ax,0x4
            rax = (rax & !0xffff) | ((0x4u64) as u16 as u64);
            // e03b6 cmovb bx,ax
            if fl.b() { rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xe03ba;
        }
        0xe03ba => {
            // block e03ba
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e03ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e03ba movsxd rax,r12d
            rax = sx::<32>((r12 as u32 as u64)) as u64;
            // e03bd mov WORD PTR [r13+rax*2+0x0],bx
            m.w16(r13.wrapping_add(rax.wrapping_mul(2)), ((rbx as u16 as u64)) as u16);
            // e03c3 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e03c6 add r12d,0x4
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x4u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e03ca add r11d,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e03ce cmp r15,QWORD PTR [rbp-0x58]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64).1;
            // e03d2 mov r14,r13
            r14 = r13;
            // e03d5 jl 0xe02bb
            if fl.l() { pc = 0xe02bb; continue; }
            pc = 0xe03db;
        }
        0xe03db => {
            // block e03db
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e03db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e03db mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e03df call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e03e4 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e03e8 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e03ed mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e03f1 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e03f5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e03f6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e03f8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e03fa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e03fc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e03fe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e03ff jmp 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
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
            if fl.e() { pc = 0xb62af; continue; }
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
            if fl.ne() { pc = 0xb622f; continue; }
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
            if fl.ne() { pc = 0xb6247; continue; }
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
            if fl.ne() { pc = 0xb6255; continue; }
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
            if fl.ne() { pc = 0xb6274; continue; }
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
            if fl.e() { pc = 0xb62a6; continue; }
            pc = 0xb628f;
        }
        0xb628f => {
            // block b628f
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b628f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b628f mov r13,r8
            r13 = r8;
            // b6292 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6297 mov r8,r13
            r8 = r13;
            // b629a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b629c je 0xb62a6
            if fl.e() { pc = 0xb62a6; continue; }
            pc = 0xb629e;
        }
        0xb629e => {
            // block b629e
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b629e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b629e mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b62a2 jns 0xb62e0
            if fl.ns() { pc = 0xb62e0; continue; }
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
            if fl.ae() { pc = 0xb62e0; continue; }
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
            if fl.e() { pc = 0xb6428; continue; }
            pc = 0xb62c2;
        }
        0xb62c2 => {
            // block b62c2
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62c2 test rax,rax
            fl = logic::<64>(rax & rax);
            // b62c5 je 0xb6394
            if fl.e() { pc = 0xb6394; continue; }
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
            if fl.ne() { pc = 0xb62cf; continue; }
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
            if fl.ne() { pc = 0xb62ee; continue; }
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
            if fl.ne() { pc = 0xb6306; continue; }
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
            if fl.ne() { pc = 0xb6314; continue; }
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
            if fl.ne() { pc = 0xb6328; continue; }
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
            if fl.e() { pc = 0xb635d; continue; }
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
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b634f mov r9,r13
            r9 = r13;
            // b6352 mov r13,r12
            r13 = r12;
            // b6355 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b6357 je 0xb635d
            if fl.e() { pc = 0xb635d; continue; }
            pc = 0xb6359;
        }
        0xb6359 => {
            // block b6359
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6359 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6359 jns 0xb6388
            if fl.ns() { pc = 0xb6388; continue; }
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
            if fl.ae() { pc = 0xb6388; continue; }
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
            if fl.e() { pc = 0xb645c; continue; }
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
            if fl.ne() { pc = 0xb6378; continue; }
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
            if fl.e() { pc = 0xb639b; continue; }
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
            if fl.ne() { pc = 0xb63b8; continue; }
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
            if fl.ne() { pc = 0xb63cd; continue; }
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
            if fl.ne() { pc = 0xb63de; continue; }
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
            if fl.ne() { pc = 0xb63ef; continue; }
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
            if fl.e() { pc = 0xb641c; continue; }
            pc = 0xb6402;
        }
        0xb6402 => {
            // block b6402
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6402 mov r15,rcx
            r15 = rcx;
            // b6405 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b640a mov rcx,r15
            rcx = r15;
            // b640d test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b640f je 0xb641c
            if fl.e() { pc = 0xb641c; continue; }
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
            if fl.ns() { pc = 0xb643d; continue; }
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
            if fl.ae() { pc = 0xb643d; continue; }
            pc = 0xb6428;
        }
        0xb6428 => {
            // block b6428
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6428 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6428 test rax,rax
            fl = logic::<64>(rax & rax);
            // b642b je 0xb644d
            if fl.e() { pc = 0xb644d; continue; }
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
            if fl.ne() { pc = 0xb6465; continue; }
            pc = 0xb6471;
        }
        0xb6471 => {
            // block b6471
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6471 cmp rbx,QWORD PTR [rbp-0x48]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64).1;
            // b6475 je 0xb64ea
            if fl.e() { pc = 0xb64ea; continue; }
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
            if fl.ne() { pc = 0xb6484; continue; }
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
            if fl.ne() { pc = 0xb6498; continue; }
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
            if fl.ne() { pc = 0xb64a6; continue; }
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
            if fl.ne() { pc = 0xb64b9; continue; }
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
            if fl.e() { pc = 0xb64df; continue; }
            pc = 0xb64cc;
        }
        0xb64cc => {
            // block b64cc
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64cc call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b64d1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b64d3 je 0xb64df
            if fl.e() { pc = 0xb64df; continue; }
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
            if fl.ns() { pc = 0xb650a; continue; }
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
            if fl.ae() { pc = 0xb650a; continue; }
            pc = 0xb64ea;
        }
        0xb64ea => {
            // block b64ea
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64ea cmp QWORD PTR [rcx+0x8],0x0
            fl = sub::<64>(m.r64(rcx.wrapping_add(0x8)) as u64, 0x0u64).1;
            // b64ef je 0xb6525
            if fl.e() { pc = 0xb6525; continue; }
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

/// parity: P2v @0xdf804 size=818
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000df804<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdf804;
    loop {
        match pc {
        0xdf804 => {
            // block df804
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df804 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df804 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df805 mov rbp,rsp
            rbp = rsp;
            // df808 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // df80a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // df80c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // df80e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // df810 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df811 sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // df818 mov DWORD PTR [rbp-0xa8],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), ((r8 as u32 as u64)) as u32);
            // df81f mov DWORD PTR [rbp-0x84],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), ((rcx as u32 as u64)) as u32);
            // df825 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // df828 mov rbx,rsi
            rbx = rsi;
            // df82b mov r14,rdi
            r14 = rdi;
            // df82e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // df836 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df839 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // df83e call QWORD PTR [rax+0x1908]
            { let r = { let t = m.r64(rax.wrapping_add(0x1908)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df844 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df847 lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // df84b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // df84d mov rdi,r14
            rdi = r14;
            // df850 call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df856 mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // df859 mov edx,DWORD PTR [r14+0x274]
            rdx = (m.r32(r14.wrapping_add(0x274)) as u64) as u32 as u64;
            // df860 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // df864 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // df866 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df869 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df86c call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df871 mov eax,DWORD PTR [rbp-0x6c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // df874 mov DWORD PTR [rbp-0x74],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), ((rax as u32 as u64)) as u32);
            // df877 mov eax,DWORD PTR [rbp-0x68]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // df87a mov DWORD PTR [rbp-0x78],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), ((rax as u32 as u64)) as u32);
            // df87d mov r15d,DWORD PTR [rbp-0x70]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // df881 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df884 lea rsi,[rbp-0x7c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff84);
            // df888 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // df88a mov rdi,r14
            rdi = r14;
            // df88d call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df893 cmp DWORD PTR [rbp-0x7c],0xa
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64, 0xau64).1;
            // df897 jne 0xdf8ad
            if fl.ne() { pc = 0xdf8ad; continue; }
            pc = 0xdf899;
        }
        0xdf899 => {
            // block df899
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df899 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df899 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df89c lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df8a0 lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df8a4 mov rdi,r14
            rdi = r14;
            // df8a7 call QWORD PTR [rax+0x1a00]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdf8ad;
        }
        0xdf8ad => {
            // block df8ad
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df8ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df8ad mov r13,QWORD PTR [r14+0x280]
            r13 = m.r64(r14.wrapping_add(0x280)) as u64;
            // df8b4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // df8b6 mov QWORD PTR [rbp-0xb0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rax) as u64);
            // df8bd test r13,r13
            fl = logic::<64>(r13 & r13);
            // df8c0 jne 0xdf90c
            if fl.ne() { pc = 0xdf90c; continue; }
            pc = 0xdf8c2;
        }
        0xdf8c2 => {
            // block df8c2
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df8c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df8c2 cmp BYTE PTR [r14+0x279],0x0
            fl = sub::<8>(m.r8(r14.wrapping_add(0x279)) as u64, 0x0u64).1;
            // df8ca je 0xdf8e8
            if fl.e() { pc = 0xdf8e8; continue; }
            pc = 0xdf8cc;
        }
        0xdf8cc => {
            // block df8cc
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df8cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df8cc mov edi,DWORD PTR [rbp-0x78]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // df8cf imul edi,r15d
            { let (r, f) = imul::<32>((rdi as u32 as u64), (r15 as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // df8d3 shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // df8d7 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df8dc mov r13,rax
            r13 = rax;
            // df8df mov QWORD PTR [rbp-0xb0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (r13) as u64);
            // df8e6 jmp 0xdf90c
            pc = 0xdf90c; continue;
        }
        0xdf8e8 => {
            // block df8e8
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df8e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df8e8 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df8eb mov esi,DWORD PTR [r14+0x250]
            rsi = (m.r32(r14.wrapping_add(0x250)) as u64) as u32 as u64;
            // df8f2 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // df8f6 mov rdi,r14
            rdi = r14;
            // df8f9 call QWORD PTR [rax+0xa10]
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df8ff mov r13,QWORD PTR [rbp-0x30]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // df903 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // df905 mov QWORD PTR [rbp-0xb0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rax) as u64);
            pc = 0xdf90c;
        }
        0xdf90c => {
            // block df90c
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df90c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df90c mov QWORD PTR [rbp-0xb8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (r14) as u64);
            // df913 mov eax,DWORD PTR [rbp-0x74]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // df916 cmp eax,r12d
            fl = sub::<32>((rax as u32 as u64), (r12 as u32 as u64)).1;
            // df919 cmovb r12d,eax
            if fl.b() { r12 = ((rax as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // df91d mov QWORD PTR [rbp-0x90],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (r12) as u64);
            // df924 mov edx,DWORD PTR [rbp-0x84]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c)) as u64) as u32 as u64;
            // df92a cmp r15d,edx
            fl = sub::<32>((r15 as u32 as u64), (rdx as u32 as u64)).1;
            // df92d cmovb edx,r15d
            if fl.b() { rdx = ((r15 as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // df931 mov rcx,r12
            rcx = r12;
            // df934 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // df937 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // df939 mov r14d,0x0
            r14 = (0x0u64) as u32 as u64;
            // df93f mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // df944 jbe 0xdf959
            if fl.be() { pc = 0xdf959; continue; }
            pc = 0xdf946;
        }
        0xdf946 => {
            // block df946
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df946 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df946 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // df949 shr r14d,1
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x1u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // df94c sub eax,r14d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r14 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // df94f shl r14d,0x2
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // df953 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df956 mov rcx,rax
            rcx = rax;
            pc = 0xdf959;
        }
        0xdf959 => {
            // block df959
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df959 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df959 mov QWORD PTR [rbp-0x98],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rcx) as u64);
            // df960 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // df963 mov DWORD PTR [rbp-0xbc],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff44), ((r15 as u32 as u64)) as u32);
            // df96a sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // df96c mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // df96f mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // df974 jbe 0xdf981
            if fl.be() { pc = 0xdf981; continue; }
            pc = 0xdf976;
        }
        0xdf976 => {
            // block df976
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df976 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // df979 shr r12d,1
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x1u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // df97c sub eax,r12d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r12 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // df97f mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xdf981;
        }
        0xdf981 => {
            // block df981
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df981 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df981 mov QWORD PTR [rbp-0xc8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rcx) as u64);
            // df988 mov esi,DWORD PTR [rbp-0x78]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // df98b imul esi,r12d
            { let (r, f) = imul::<32>((rsi as u32 as u64), (r12 as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // df98f shl esi,0x2
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x2u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // df992 mov rdi,r13
            rdi = r13;
            // df995 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df99a mov esi,DWORD PTR [rbp-0x78]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // df99d imul r12d,esi
            { let (r, f) = imul::<32>((r12 as u32 as u64), (rsi as u32 as u64)); r12 = (r) as u32 as u64; fl = f; }
            // df9a1 lea r12,[r13+r12*4+0x0]
            r12 = r13.wrapping_add(r12.wrapping_mul(4));
            // df9a6 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // df9a9 je 0xdfa3c
            if fl.e() { pc = 0xdfa3c; continue; }
            pc = 0xdf9af;
        }
        0xdf9af => {
            // block df9af
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df9af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df9af mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // df9b6 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df9b9 mov QWORD PTR [rbp-0x90],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (rax) as u64);
            // df9c0 lea rax,[r14+rax*1]
            rax = r14.wrapping_add(rax);
            // df9c4 mov QWORD PTR [rbp-0xa0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (rax) as u64);
            // df9cb mov eax,DWORD PTR [rbp-0xa8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64) as u32 as u64;
            // df9d1 mov QWORD PTR [rbp-0xa8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rax) as u64);
            // df9d8 mov DWORD PTR [rbp-0x84],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), ((r15 as u32 as u64)) as u32);
            // df9df mov r13w,0x1
            r13 = (r13 & !0xffff) | ((0x1u64) as u16 as u64);
            // df9e4 mov r15,r12
            r15 = r12;
            pc = 0xdf9e7;
        }
        0xdf9e7 => {
            // block df9e7
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df9e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df9e7 mov rdi,r12
            rdi = r12;
            // df9ea mov rsi,r14
            rsi = r14;
            // df9ed call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df9f2 lea rdi,[r12+r14*1]
            rdi = r12.wrapping_add(r14);
            // df9f6 mov rsi,rbx
            rsi = rbx;
            // df9f9 mov rdx,QWORD PTR [rbp-0x90]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // dfa00 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa05 add r12,QWORD PTR [rbp-0xa0]
            { let (r, f) = add::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64);
              r12 = r;
              fl = f; }
            // dfa0c mov rdi,r12
            rdi = r12;
            // dfa0f mov rsi,QWORD PTR [rbp-0x98]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // dfa16 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa1b mov esi,DWORD PTR [rbp-0x78]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // dfa1e lea r12,[r15+rsi*4]
            r12 = r15.wrapping_add(rsi.wrapping_mul(4));
            // dfa22 add rbx,QWORD PTR [rbp-0xa8]
            { let (r, f) = add::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64);
              rbx = r;
              fl = f; }
            // dfa29 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // dfa2d lea r13d,[r13+0x1]
            r13 = (r13.wrapping_add(0x1)) as u32 as u64;
            // dfa31 cmp eax,DWORD PTR [rbp-0x84]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c)) as u64).1;
            // dfa37 mov r15,r12
            r15 = r12;
            // dfa3a jb 0xdf9e7
            if fl.b() { pc = 0xdf9e7; continue; }
            pc = 0xdfa3c;
        }
        0xdfa3c => {
            // block dfa3c
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfa3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfa3c mov rax,QWORD PTR [rbp-0xc8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64;
            // dfa43 imul esi,eax
            { let (r, f) = imul::<32>((rsi as u32 as u64), (rax as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // dfa46 mov rdi,r12
            rdi = r12;
            // dfa49 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa4e mov r12,QWORD PTR [rbp-0xb0]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // dfa55 test r12,r12
            fl = logic::<64>(r12 & r12);
            // dfa58 mov rbx,QWORD PTR [rbp-0xb8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // dfa5f mov r15d,DWORD PTR [rbp-0xbc]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff44)) as u64) as u32 as u64;
            // dfa66 je 0xdface
            if fl.e() { pc = 0xdface; continue; }
            pc = 0xdfa68;
        }
        0xdfa68 => {
            // block dfa68
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfa68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfa68 mov al,BYTE PTR [rbx+0x279]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x279)) as u64) as u8 as u64);
            // dfa6e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dfa70 je 0xdface
            if fl.e() { pc = 0xdface; continue; }
            pc = 0xdfa72;
        }
        0xdfa72 => {
            // block dfa72
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfa72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfa72 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfa75 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // dfa7b lea rdx,[rbp-0x80]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // dfa7f mov rdi,rbx
            rdi = rbx;
            // dfa82 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa88 mov r14d,DWORD PTR [rbp-0x80]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            // dfa8c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfa8f mov rdi,rbx
            rdi = rbx;
            // dfa92 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa98 cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // dfa9b jbe 0xdfaa8
            if fl.be() { pc = 0xdfaa8; continue; }
            pc = 0xdfa9d;
        }
        0xdfa9d => {
            // block dfa9d
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfa9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfa9d mov DWORD PTR [rbp-0x80],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u32);
            // dfaa4 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dfaa6 jmp 0xdfaab
            pc = 0xdfaab; continue;
        }
        0xdfaa8 => {
            // block dfaa8
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfaa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfaa8 mov esi,DWORD PTR [rbp-0x80]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            pc = 0xdfaab;
        }
        0xdfaab => {
            // block dfaab
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfaab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfaab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfaae imul r15d,DWORD PTR [rbp-0x78]
            { let (r, f) = imul::<32>((r15 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64); r15 = (r) as u32 as u64; fl = f; }
            // dfab3 shl r15d,0x2
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x2u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // dfab7 mov rdi,rbx
            rdi = rbx;
            // dfaba mov rdx,r12
            rdx = r12;
            // dfabd mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // dfac0 call QWORD PTR [rax+0x2c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfac6 mov rdi,r12
            rdi = r12;
            // dfac9 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdface;
        }
        0xdface => {
            // block dface
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dface rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dface mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfad1 movzx esi,BYTE PTR [rbx+0x278]
            rsi = (m.r8(rbx.wrapping_add(0x278)) as u64) as u32 as u64;
            // dfad8 mov rdi,rbx
            rdi = rbx;
            // dfadb call QWORD PTR [rax+0x570]
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfae1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfae4 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // dfaea mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // dfaef mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dfaf4 mov rdi,rbx
            rdi = rbx;
            // dfaf7 call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfafd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfb00 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // dfb06 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfb08 mov rdi,rbx
            rdi = rbx;
            // dfb0b call QWORD PTR [rax+0x4f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfb11 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfb14 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // dfb1a mov rdi,rbx
            rdi = rbx;
            // dfb1d call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfb23 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // dfb2a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dfb2b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dfb2d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dfb2f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dfb31 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dfb33 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dfb34 ret
            return Ret { rax, rdx, xmm0: x0 };
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
            if fl.g() { pc = 0xe7941; continue; }
            pc = 0xe7918;
        }
        0xe7918 => {
            // block e7918
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7918 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7918 cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // e791e jg 0xe796d
            if fl.g() { pc = 0xe796d; continue; }
            pc = 0xe7920;
        }
        0xe7920 => {
            // block e7920
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7920 cmp edi,0x102447ff
            fl = sub::<32>((rdi as u32 as u64), 0x102447ffu64).1;
            // e7926 jg 0xe79b5
            if fl.g() { pc = 0xe79b5; continue; }
            pc = 0xe792c;
        }
        0xe792c => {
            // block e792c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e792c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e792c cmp edi,0xffffffff
            fl = sub::<32>((rdi as u32 as u64), 0xffffffffu64).1;
            // e792f jne 0xe7c0a
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.g() { pc = 0xe7991; continue; }
            pc = 0xe7949;
        }
        0xe7949 => {
            // block e7949
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7949 cmp edi,0x106466ff
            fl = sub::<32>((rdi as u32 as u64), 0x106466ffu64).1;
            // e794f jg 0xe79f1
            if fl.g() { pc = 0xe79f1; continue; }
            pc = 0xe7955;
        }
        0xe7955 => {
            // block e7955
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7955 cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // e795b jne 0xe7c0a
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.g() { pc = 0xe7a1c; continue; }
            pc = 0xe7979;
        }
        0xe7979 => {
            // block e7979
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7979 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7979 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // e797f jne 0xe7a4c
            if fl.ne() { pc = 0xe7a4c; continue; }
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
            if fl.g() { pc = 0xe7a34; continue; }
            pc = 0xe799d;
        }
        0xe799d => {
            // block e799d
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e799d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e799d cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // e79a3 jne 0xe7a64
            if fl.ne() { pc = 0xe7a64; continue; }
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
            if fl.g() { pc = 0xe7a7c; continue; }
            pc = 0xe79c1;
        }
        0xe79c1 => {
            // block e79c1
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79c1 cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // e79c7 jg 0xe7ad8
            if fl.g() { pc = 0xe7ad8; continue; }
            pc = 0xe79cd;
        }
        0xe79cd => {
            // block e79cd
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79cd cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // e79d3 je 0xe7b48
            if fl.e() { pc = 0xe7b48; continue; }
            pc = 0xe79d9;
        }
        0xe79d9 => {
            // block e79d9
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79d9 cmp edi,0x10266400
            fl = sub::<32>((rdi as u32 as u64), 0x10266400u64).1;
            // e79df jne 0xe7b54
            if fl.ne() { pc = 0xe7b54; continue; }
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
            if fl.a() { pc = 0xe7c0a; continue; }
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
            if fl.ne() { pc = 0xe7aa8; continue; }
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
            if fl.ne() { pc = 0xe7ac0; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.g() { pc = 0xe7af8; continue; }
            pc = 0xe7a84;
        }
        0xe7a84 => {
            // block e7a84
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a84 cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // e7a8a je 0xe7b6c
            if fl.e() { pc = 0xe7b6c; continue; }
            pc = 0xe7a90;
        }
        0xe7a90 => {
            // block e7a90
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a90 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // e7a96 jne 0xe7b78
            if fl.ne() { pc = 0xe7b78; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.g() { pc = 0xe7b18; continue; }
            pc = 0xe7ae0;
        }
        0xe7ae0 => {
            // block e7ae0
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ae0 cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // e7ae6 jne 0xe7bc6
            if fl.ne() { pc = 0xe7bc6; continue; }
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
            if fl.g() { pc = 0xe7b30; continue; }
            pc = 0xe7b00;
        }
        0xe7b00 => {
            // block e7b00
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b00 cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // e7b06 jne 0xe7bd7
            if fl.ne() { pc = 0xe7bd7; continue; }
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
            if fl.ne() { pc = 0xe7be8; continue; }
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
            if fl.ne() { pc = 0xe7bf9; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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
            if fl.ne() { pc = 0xe7c0a; continue; }
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

/// parity: P2v @0xc7c7f size=789
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c7c7f<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc7c7f;
    loop {
        match pc {
        0xc7c7f => {
            // block c7c7f
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7c7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7c7f push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c7c80 mov rbp,rsp
            rbp = rsp;
            // c7c83 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c7c85 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c7c86 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // c7c89 mov rbx,rsi
            rbx = rsi;
            // c7c8c dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // c7c8e cmp ecx,0x1c
            fl = sub::<32>((rcx as u32 as u64), 0x1cu64).1;
            // c7c91 ja 0xc7f19
            if fl.a() { pc = 0xc7f19; continue; }
            pc = 0xc7c97;
        }
        0xc7c97 => {
            // block c7c97
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7c97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7c97 lea rax,[rip+0x282] # 0xc7f20
            rax = m.env.code_va(0xc7f20);
            // c7c9e movsxd rcx,DWORD PTR [rax+rcx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // c7ca2 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // c7ca5 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc7ca7 => {
            // block c7ca7
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ca7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ca7 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7cae sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7cb0 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7cb3 mov rsi,rbx
            rsi = rbx;
            // c7cb6 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7cb9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7cba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7cbc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7cbd jmp 0xc70f4
            { let r = { let t = m.env.code_va(0xc70f4); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xc7cc2 => {
            // block c7cc2
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cc2 mov rsi,rbx
            rsi = rbx;
            // c7cc5 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7cc8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7cc9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7ccb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7ccc jmp 0xc5d09
            { let r = { let t = m.env.code_va(0xc5d09); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7cd1 => {
            // block c7cd1
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cd1 test r14d,0x7fffffff
            fl = logic::<32>((r14 as u32 as u64) & 0x7fffffffu64);
            // c7cd8 je 0xc7f19
            if fl.e() { pc = 0xc7f19; continue; }
            pc = 0xc7cde;
        }
        0xc7cde => {
            // block c7cde
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cde add r14d,r14d
            { let (r, f) = add::<32>((r14 as u32 as u64), (r14 as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7ce1;
        }
        0xc7ce1 => {
            // block c7ce1
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ce1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ce1 movzx eax,WORD PTR [rdi]
            rax = (m.r16(rdi) as u64) as u32 as u64;
            // c7ce4 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c7ce7 mov BYTE PTR [rbx],al
            m.w8(rbx, ((rax as u8 as u64)) as u8);
            // c7ce9 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // c7cec add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // c7cf0 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // c7cf3 jne 0xc7ce1
            if fl.ne() { pc = 0xc7ce1; continue; }
            pc = 0xc7cf5;
        }
        0xc7cf5 => {
            // block c7cf5
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cf5 jmp 0xc7f19
            pc = 0xc7f19; continue;
        }
        0xc7cfa => {
            // block c7cfa
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cfa cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d01 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d03 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d06 mov rsi,rbx
            rsi = rbx;
            // c7d09 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7d0c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d0d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d0f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d10 jmp 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xc7d15 => {
            // block c7d15
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d15 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d1c sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d1e and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d21 mov rsi,rbx
            rsi = rbx;
            // c7d24 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7d27 call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7d2c mov rdi,rbx
            rdi = rbx;
            // c7d2f mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7d32 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d33 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d35 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d36 jmp 0xc7a37
            { let r = { let t = m.env.code_va(0xc7a37); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7d3b => {
            // block c7d3b
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d3b cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d42 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d44 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d47 mov rsi,rbx
            rsi = rbx;
            // c7d4a mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7d4d call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7d52 mov rdi,rbx
            rdi = rbx;
            // c7d55 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7d58 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d59 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d5b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d5c jmp 0xc601b
            { let r = { let t = m.env.code_va(0xc601b); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0xc7d61 => {
            // block c7d61
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d61 test r14d,0x7fffffff
            fl = logic::<32>((r14 as u32 as u64) & 0x7fffffffu64);
            // c7d68 je 0xc7d84
            if fl.e() { pc = 0xc7d84; continue; }
            pc = 0xc7d6a;
        }
        0xc7d6a => {
            // block c7d6a
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d6a lea eax,[r14+r14*1]
            rax = (r14.wrapping_add(r14)) as u32 as u64;
            // c7d6e mov rcx,rbx
            rcx = rbx;
            pc = 0xc7d71;
        }
        0xc7d71 => {
            // block c7d71
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d71 movzx edx,WORD PTR [rdi]
            rdx = (m.r16(rdi) as u64) as u32 as u64;
            // c7d74 shr edx,0x2
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c7d77 mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // c7d79 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // c7d7c add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // c7d80 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // c7d82 jne 0xc7d71
            if fl.ne() { pc = 0xc7d71; continue; }
            pc = 0xc7d84;
        }
        0xc7d84 => {
            // block c7d84
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d84 mov rdi,rbx
            rdi = rbx;
            // c7d87 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7d8a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d8b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d8d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d8e jmp 0xc7bec
            { let r = { let t = m.env.code_va(0xc7bec); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
        0xc7d93 => {
            // block c7d93
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d93 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d9a sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d9c and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d9f mov rsi,rbx
            rsi = rbx;
            // c7da2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7da5 call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7daa shl r14d,0x2
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c7dae test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7db1 je 0xc7f19
            if fl.e() { pc = 0xc7f19; continue; }
            pc = 0xc7db7;
        }
        0xc7db7 => {
            // block c7db7
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7db7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7db7 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7dba xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7dbc;
        }
        0xc7dbc => {
            // block c7dbc
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7dbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7dbc mov dl,BYTE PTR [rbx+rcx*1]
            rdx = (rdx & !0xff) | ((m.r8(rbx.wrapping_add(rcx)) as u64) as u8 as u64);
            // c7dbf mov al,BYTE PTR [rbx+rcx*1+0x2]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(rcx).wrapping_add(0x2)) as u64) as u8 as u64);
            // c7dc3 mov BYTE PTR [rbx+rcx*1],al
            m.w8(rbx.wrapping_add(rcx), ((rax as u8 as u64)) as u8);
            // c7dc6 mov BYTE PTR [rbx+rcx*1+0x2],dl
            m.w8(rbx.wrapping_add(rcx).wrapping_add(0x2), ((rdx as u8 as u64)) as u8);
            // c7dca add rcx,0x4
            { let (r, f) = add::<64>(rcx, 0x4u64);
              rcx = r;
              fl = f; }
            // c7dce cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // c7dd1 jb 0xc7dbc
            if fl.b() { pc = 0xc7dbc; continue; }
            pc = 0xc7dd3;
        }
        0xc7dd3 => {
            // block c7dd3
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7dd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7dd3 jmp 0xc7f19
            pc = 0xc7f19; continue;
        }
        0xc7dd8 => {
            // block c7dd8
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7dd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7dd8 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7ddf sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7de1 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7de4 mov rsi,rbx
            rsi = rbx;
            // c7de7 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7dea call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7def mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // c7df4 jmp 0xc7e75
            pc = 0xc7e75; continue;
        }
        0xc7df6 => {
            // block c7df6
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7df6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7df6 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // c7dfb mov rsi,rbx
            rsi = rbx;
            // c7dfe mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e01 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e02 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e04 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e05 jmp 0xc5ddf
            { let r = { let t = m.env.code_va(0xc5ddf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xc7e0a => {
            // block c7e0a
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e0a cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e11 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e13 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e16 mov rsi,rbx
            rsi = rbx;
            // c7e19 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e1c call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e21 mov rdi,rbx
            rdi = rbx;
            // c7e24 mov rsi,rbx
            rsi = rbx;
            // c7e27 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e2a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e2b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e2d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e2e jmp 0xc7a9b
            { let r = { let t = m.env.code_va(0xc7a9b); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7e33 => {
            // block c7e33
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e33 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e3a sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e3c and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e3f mov rsi,rbx
            rsi = rbx;
            // c7e42 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e45 call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e4a mov rdi,rbx
            rdi = rbx;
            // c7e4d mov rsi,rbx
            rsi = rbx;
            // c7e50 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e53 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e54 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e56 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e57 jmp 0xc7acf
            { let r = { let t = m.env.code_va(0xc7acf); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7e5c => {
            // block c7e5c
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e5c cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e63 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e65 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e68 mov rsi,rbx
            rsi = rbx;
            // c7e6b mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e6e call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e73 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7e75;
        }
        0xc7e75 => {
            // block c7e75
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e75 mov rdi,rbx
            rdi = rbx;
            // c7e78 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7e7b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e7c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e7e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e7f jmp 0xc60bc
            { let r = { let t = m.env.code_va(0xc60bc); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7e84 => {
            // block c7e84
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e84 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e8b sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e8d and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e90 mov rsi,rbx
            rsi = rbx;
            // c7e93 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e96 call 0xc70f4
            { let r = { let t = m.env.code_va(0xc70f4); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e9b mov rdi,rbx
            rdi = rbx;
            // c7e9e mov rsi,rbx
            rsi = rbx;
            // c7ea1 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7ea4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7ea5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7ea7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7ea8 jmp 0xc7b35
            { let r = { let t = m.env.code_va(0xc7b35); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7ead => {
            // block c7ead
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ead rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ead cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7eb4 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7eb6 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7eb9 mov rsi,rbx
            rsi = rbx;
            // c7ebc mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7ebf call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7ec4 mov rdi,rbx
            rdi = rbx;
            // c7ec7 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7eca pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7ecb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7ecd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7ece jmp 0xc604d
            { let r = { let t = m.env.code_va(0xc604d); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0xc7ed3 => {
            // block c7ed3
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ed3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ed3 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7eda sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7edc and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7edf mov rsi,rbx
            rsi = rbx;
            // c7ee2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7ee5 call 0xc70f4
            { let r = { let t = m.env.code_va(0xc70f4); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7eea test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7eed je 0xc7f19
            if fl.e() { pc = 0xc7f19; continue; }
            pc = 0xc7eef;
        }
        0xc7eef => {
            // block c7eef
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7eef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7eef mov rax,rbx
            rax = rbx;
            // c7ef2 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            pc = 0xc7ef6;
        }
        0xc7ef6 => {
            // block c7ef6
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ef6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ef6 mov cx,WORD PTR [rax-0x4]
            rcx = (rcx & !0xffff) | ((m.r16(rax.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u16 as u64);
            // c7efa mov dx,WORD PTR [rax-0x2]
            rdx = (rdx & !0xffff) | ((m.r16(rax.wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u16 as u64);
            // c7efe mov si,WORD PTR [rax]
            rsi = (rsi & !0xffff) | ((m.r16(rax) as u64) as u16 as u64);
            // c7f01 mov WORD PTR [rbx],si
            m.w16(rbx, ((rsi as u16 as u64)) as u16);
            // c7f04 mov WORD PTR [rbx+0x2],dx
            m.w16(rbx.wrapping_add(0x2), ((rdx as u16 as u64)) as u16);
            // c7f08 mov WORD PTR [rbx+0x4],cx
            m.w16(rbx.wrapping_add(0x4), ((rcx as u16 as u64)) as u16);
            // c7f0c add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c7f10 add rbx,0x6
            { let (r, f) = add::<64>(rbx, 0x6u64);
              rbx = r;
              fl = f; }
            // c7f14 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // c7f17 jne 0xc7ef6
            if fl.ne() { pc = 0xc7ef6; continue; }
            pc = 0xc7f19;
        }
        0xc7f19 => {
            // block c7f19
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7f19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7f19 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7f1a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7f1c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7f1d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x26804 size=760
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00026804<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x26804;
    loop {
        match pc {
        0x26804 => {
            // block 26804
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26804 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26804 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 26805 mov rbp,rsp
            rbp = rsp;
            // 26808 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2680a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2680b sub rsp,0xb0
            { let (r, f) = sub::<64>(rsp, 0xb0u64);
              rsp = r;
              fl = f; }
            // 26812 mov rbx,rsi
            rbx = rsi;
            // 26815 mov r14,rdi
            r14 = rdi;
            // 26818 lea rsi,[rip+0x103c37] # 0x12a456
            rsi = m.env.data_va(257059);
            // 2681f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 26824 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26829 cmp DWORD PTR [rbx+0x4],0x6e747632
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x4)) as u64, 0x6e747632u64).1;
            // 26830 jne 0x26872
            if fl.ne() { pc = 0x26872; continue; }
            pc = 0x26832;
        }
        0x26832 => {
            // block 26832
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26832 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26832 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 26836 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 2683b mov edx,0x6e
            rdx = (0x6eu64) as u32 as u64;
            // 26840 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26845 movsx edx,BYTE PTR [rbx+0x6]
            rdx = (sx::<8>(m.r8(rbx.wrapping_add(0x6)) as u64) as u64) as u32 as u64;
            // 26849 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 26850 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 26855 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2685a movzx edx,BYTE PTR [rbp-0x88]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // 26861 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 26864 jne 0x268c0
            if fl.ne() { pc = 0x268c0; continue; }
            pc = 0x26866;
        }
        0x26866 => {
            // block 26866
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26866 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26866 lea rsi,[rbp-0x87]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff79);
            // 2686d shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 26870 jmp 0x268c8
            pc = 0x268c8; continue;
        }
        0x26872 => {
            // block 26872
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26872 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26872 lea rsi,[rip+0x103bdf] # 0x12a458
            rsi = m.env.data_va(257061);
            // 26879 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 2687e mov rdi,r14
            rdi = r14;
            // 26881 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26886 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26889 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2688d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26891 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26894 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26897 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2689b mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 2689e mov rdi,rax
            rdi = rax;
            // 268a1 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 268a6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 268a9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 268ad mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 268b1 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 268b4 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 268b7 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 268bb jmp 0x26a4d
            pc = 0x26a4d; continue;
        }
        0x268c0 => {
            // block 268c0
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 268c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 268c0 mov rdx,QWORD PTR [rbp-0x80]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 268c4 mov rsi,QWORD PTR [rbp-0x78]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            pc = 0x268c8;
        }
        0x268c8 => {
            // block 268c8
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 268c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 268c8 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 268cc call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 268d1 mov rcx,QWORD PTR [rax+0x10]
            rcx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 268d5 mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 268d9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 268dc mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 268e0 mov QWORD PTR [rbp-0x50],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdx) as u64);
            // 268e4 mov QWORD PTR [rbp-0x58],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rcx) as u64);
            // 268e8 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 268f0 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 268f8 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 268ff movsx edx,BYTE PTR [rbx+0x5]
            rdx = (sx::<8>(m.r8(rbx.wrapping_add(0x5)) as u64) as u64) as u32 as u64;
            // 26903 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 2690a mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 2690f call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26914 movzx edx,BYTE PTR [rbp-0xa0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64) as u32 as u64;
            // 2691b test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 2691e jne 0x2692c
            if fl.ne() { pc = 0x2692c; continue; }
            pc = 0x26920;
        }
        0x26920 => {
            // block 26920
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26920 lea rsi,[rbp-0x9f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff61);
            // 26927 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2692a jmp 0x2693a
            pc = 0x2693a; continue;
        }
        0x2692c => {
            // block 2692c
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 2692c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2692c mov rdx,QWORD PTR [rbp-0x98]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 26933 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            pc = 0x2693a;
        }
        0x2693a => {
            // block 2693a
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 2693a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2693a lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 2693e call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26943 mov rcx,QWORD PTR [rax+0x10]
            rcx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 26947 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 2694b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2694e mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 26952 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 26956 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 2695a mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 26962 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2696a mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 26971 movsx edx,BYTE PTR [rbx+0x4]
            rdx = (sx::<8>(m.r8(rbx.wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // 26975 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 2697c mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 26981 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26986 movzx edx,BYTE PTR [rbp-0xb8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // 2698d test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 26990 jne 0x2699e
            if fl.ne() { pc = 0x2699e; continue; }
            pc = 0x26992;
        }
        0x26992 => {
            // block 26992
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26992 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26992 lea rsi,[rbp-0xb7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff49);
            // 26999 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2699c jmp 0x269ac
            pc = 0x269ac; continue;
        }
        0x2699e => {
            // block 2699e
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 2699e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2699e mov rdx,QWORD PTR [rbp-0xb0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 269a5 mov rsi,QWORD PTR [rbp-0xa8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            pc = 0x269ac;
        }
        0x269ac => {
            // block 269ac
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 269ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 269ac lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 269b0 call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 269b5 mov rcx,QWORD PTR [rax+0x10]
            rcx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 269b9 mov QWORD PTR [rbp-0x18],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (rcx) as u64);
            // 269bd mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 269c0 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 269c4 mov QWORD PTR [rbp-0x20],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (rdx) as u64);
            // 269c8 mov QWORD PTR [rbp-0x28],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rcx) as u64);
            // 269cc mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 269d4 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 269dc mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 269e3 movzx edx,BYTE PTR [rbp-0x28]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 269e7 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 269ea jne 0x269f5
            if fl.ne() { pc = 0x269f5; continue; }
            pc = 0x269ec;
        }
        0x269ec => {
            // block 269ec
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 269ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 269ec lea rsi,[rbp-0x27]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd9);
            // 269f0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 269f3 jmp 0x269fd
            pc = 0x269fd; continue;
        }
        0x269f5 => {
            // block 269f5
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 269f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 269f5 mov rdx,QWORD PTR [rbp-0x20]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 269f9 mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            pc = 0x269fd;
        }
        0x269fd => {
            // block 269fd
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 269fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 269fd mov rdi,r14
            rdi = r14;
            // 26a00 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a05 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 26a09 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a0e lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 26a15 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a1a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 26a1e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a23 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 26a2a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a2f lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 26a33 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a38 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 26a3f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a44 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 26a48 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26a4d;
        }
        0x26a4d => {
            // block 26a4d
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26a4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26a4d lea rsi,[rip+0x103a0f] # 0x12a463
            rsi = m.env.data_va(257072);
            // 26a54 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 26a59 mov rdi,r14
            rdi = r14;
            // 26a5c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a61 mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 26a63 mov rdi,rax
            rdi = rax;
            // 26a66 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a6b lea rsi,[rip+0x1039e2] # 0x12a454
            rsi = m.env.data_va(257057);
            // 26a72 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 26a77 mov rdi,rax
            rdi = rax;
            // 26a7a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a7f add rsp,0xb0
            { let (r, f) = add::<64>(rsp, 0xb0u64);
              rsp = r;
              fl = f; }
            // 26a86 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 26a87 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 26a89 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 26a8a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x26a8b => {
            // block 26a8b
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26a8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26a8b mov rbx,rax
            rbx = rax;
            // 26a8e jmp 0x26aeb
            pc = 0x26aeb; continue;
        }
        0x26a90 => {
            // block 26a90
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26a90 mov rbx,rax
            rbx = rax;
            // 26a93 jmp 0x26adf
            pc = 0x26adf; continue;
        }
        0x26a95 => {
            // block 26a95
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26a95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26a95 mov rbx,rax
            rbx = rax;
            // 26a98 jmp 0x26ad6
            pc = 0x26ad6; continue;
        }
        0x26a9a => {
            // block 26a9a
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26a9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26a9a mov rbx,rax
            rbx = rax;
            // 26a9d jmp 0x26aca
            pc = 0x26aca; continue;
        }
        0x26a9f => {
            // block 26a9f
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26a9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26a9f mov rbx,rax
            rbx = rax;
            // 26aa2 jmp 0x26ac1
            pc = 0x26ac1; continue;
        }
        0x26aa4 => {
            // block 26aa4
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26aa4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26aa4 mov rbx,rax
            rbx = rax;
            // 26aa7 jmp 0x26ab5
            pc = 0x26ab5; continue;
        }
        0x26aa9 => {
            // block 26aa9
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26aa9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26aa9 mov rbx,rax
            rbx = rax;
            // 26aac lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 26ab0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26ab5;
        }
        0x26ab5 => {
            // block 26ab5
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26ab5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26ab5 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 26abc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26ac1;
        }
        0x26ac1 => {
            // block 26ac1
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26ac1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26ac1 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 26ac5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26aca;
        }
        0x26aca => {
            // block 26aca
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26aca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26aca lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 26ad1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26ad6;
        }
        0x26ad6 => {
            // block 26ad6
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26ad6 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 26ada call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26adf;
        }
        0x26adf => {
            // block 26adf
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26adf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26adf lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 26ae6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26aeb;
        }
        0x26aeb => {
            // block 26aeb
            #[cfg(feature = "trace")] eprintln!("f_00026804 pc 26aeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26aeb lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 26aef call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26af4 mov rdi,rbx
            rdi = rbx;
            // 26af7 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29a94 size=754
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029a94<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x29a94;
    loop {
        match pc {
        0x29a94 => {
            // block 29a94
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29a94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29a94 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29a95 mov rbp,rsp
            rbp = rsp;
            // 29a98 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 29a9a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 29a9c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 29a9e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 29aa0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29aa1 sub rsp,0x1f8
            { let (r, f) = sub::<64>(rsp, 0x1f8u64);
              rsp = r;
              fl = f; }
            // 29aa8 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 29aab mov r14,rsi
            r14 = rsi;
            // 29aae mov QWORD PTR [rbp-0x218],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fde8), (rdi) as u64);
            // 29ab5 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 29abc lea rbx,[rbp-0x140]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 29ac3 mov rax,QWORD PTR [rip+0x116706] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 29aca lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 29ace mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // 29ad5 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 29ad9 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 29ae0 mov rsi,rbx
            rsi = rbx;
            // 29ae3 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29ae8 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 29af0 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // 29af7 mov r12,QWORD PTR [rip+0x1168c2] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 29afe lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 29b03 mov QWORD PTR [rbp-0x220],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fde0), (rax) as u64);
            // 29b0a mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 29b11 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 29b15 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // 29b1c mov rdi,rbx
            rdi = rbx;
            // 29b1f call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29b24 mov rdi,rbx
            rdi = rbx;
            // 29b27 mov r13,QWORD PTR [rip+0x11687a] # 0x1403a8
            r13 = m.r64(m.env.data_va(346997)) as u64;
            // 29b2e add r13,0x10
            { let (r, f) = add::<64>(r13, 0x10u64);
              r13 = r;
              fl = f; }
            // 29b32 mov QWORD PTR [rbp-0x140],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r13) as u64);
            // 29b39 lea rbx,[rbp-0x100]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 29b40 mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // 29b4b mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 29b56 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 29b61 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 29b6c mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // 29b76 xorps xmm0,xmm0
            x0 = 0;
            // 29b79 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 29b7d mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 29b85 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 29b89 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29b8e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 29b92 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29b97 mov rax,QWORD PTR [r14+0x50]
            rax = m.r64(r14.wrapping_add(0x50)) as u64;
            // 29b9b mov DWORD PTR [rbp-0x158],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (0xffffffffu64) as u32);
            // 29ba5 mov DWORD PTR [rbp-0x154],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feac), (0xffffffffu64) as u32);
            // 29baf mov DWORD PTR [rbp-0x150],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0xffffffffu64) as u32);
            // 29bb9 cmp r15d,0x1a
            fl = sub::<32>((r15 as u32 as u64), 0x1au64).1;
            // 29bbd ja 0x29c1b
            if fl.a() { pc = 0x29c1b; continue; }
            pc = 0x29bbf;
        }
        0x29bbf => {
            // block 29bbf
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29bbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29bbf test rax,rax
            fl = logic::<64>(rax & rax);
            // 29bc2 je 0x29c1b
            if fl.e() { pc = 0x29c1b; continue; }
            pc = 0x29bc4;
        }
        0x29bc4 => {
            // block 29bc4
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29bc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29bc4 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 29bc7 lea rcx,[rcx+rcx*2]
            rcx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 29bcb mov edx,DWORD PTR [rax+rcx*4+0x8]
            rdx = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // 29bcf mov DWORD PTR [rbp-0x150],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb0), ((rdx as u32 as u64)) as u32);
            // 29bd5 mov rax,QWORD PTR [rax+rcx*4]
            rax = m.r64(rax.wrapping_add(rcx.wrapping_mul(4))) as u64;
            // 29bd9 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 29be0 mov eax,DWORD PTR [rbp-0x154]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_feac)) as u64) as u32 as u64;
            // 29be6 and eax,DWORD PTR [rbp-0x158]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 29bec cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // 29bef mov rbx,QWORD PTR [rbp-0x218]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde8)) as u64;
            // 29bf6 jne 0x29c42
            if fl.ne() { pc = 0x29c42; continue; }
            pc = 0x29bf8;
        }
        0x29bf8 => {
            // block 29bf8
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29bf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29bf8 cmp DWORD PTR [rbp-0x150],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64, 0xffffffffu64).1;
            // 29bff jne 0x29c42
            if fl.ne() { pc = 0x29c42; continue; }
            pc = 0x29c01;
        }
        0x29c01 => {
            // block 29c01
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c01 lea rsi,[rip+0x100cf1] # 0x12a8f9
            rsi = m.env.data_va(258246);
            // 29c08 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29c0f mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 29c14 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c19 jmp 0x29c79
            pc = 0x29c79; continue;
        }
        0x29c1b => {
            // block 29c1b
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c1b cmp r15d,0x1a
            fl = sub::<32>((r15 as u32 as u64), 0x1au64).1;
            // 29c1f mov rbx,QWORD PTR [rbp-0x218]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde8)) as u64;
            // 29c26 ja 0x29c79
            if fl.a() { pc = 0x29c79; continue; }
            pc = 0x29c28;
        }
        0x29c28 => {
            // block 29c28
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c28 lea rsi,[rip+0x100cca] # 0x12a8f9
            rsi = m.env.data_va(258246);
            // 29c2f lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29c36 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 29c3b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c40 jmp 0x29c79
            pc = 0x29c79; continue;
        }
        0x29c42 => {
            // block 29c42
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c42 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 29c49 lea rsi,[rbp-0x158]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 29c50 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 29c55 call 0xe5324
            { let r = { let t = m.env.code_va(0xe5324); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c5a lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29c61 lea rsi,[rbp-0x210]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 29c68 call 0xe7554
            { let r = { let t = m.env.code_va(0xe7554); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c6d lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 29c74 call 0xe554e
            { let r = { let t = m.env.code_va(0xe554e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x29c79;
        }
        0x29c79 => {
            // block 29c79
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c79 mov rdi,rbx
            rdi = rbx;
            // 29c7c lea rsi,[rbp-0x140]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 29c83 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c88 mov rax,QWORD PTR [rbp-0x220]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde0)) as u64;
            // 29c8f mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 29c96 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // 29c9d mov QWORD PTR [rbp-0x140],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r13) as u64);
            // 29ca4 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 29cab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29cb0 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 29cb7 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29cbc mov rsi,QWORD PTR [rip+0x116545] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 29cc3 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 29cc7 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29cce call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29cd3 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 29cda call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29cdf mov rax,rbx
            rax = rbx;
            // 29ce2 add rsp,0x1f8
            { let (r, f) = add::<64>(rsp, 0x1f8u64);
              rsp = r;
              fl = f; }
            // 29ce9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29cea pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 29cec pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 29cee pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 29cf0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 29cf2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29cf3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x29cf4 => {
            // block 29cf4
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29cf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29cf4 mov r14,rax
            r14 = rax;
            pc = 0x29cf7;
        }
        0x29cf7 => {
            // block 29cf7
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29cf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29cf7 mov rax,QWORD PTR [rbp-0x220]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde0)) as u64;
            // 29cfe mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 29d05 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // 29d0c mov QWORD PTR [rbp-0x140],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r13) as u64);
            // 29d13 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 29d1a jmp 0x29d35
            pc = 0x29d35; continue;
        }
        0x29d1c => {
            // block 29d1c
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d1c mov r14,rax
            r14 = rax;
            // 29d1f jmp 0x29d5d
            pc = 0x29d5d; continue;
        }
        0x29d21 => {
            // block 29d21
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d21 mov r14,rax
            r14 = rax;
            // 29d24 jmp 0x29d46
            pc = 0x29d46; continue;
        }
        0x29d26 => {
            // block 29d26
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d26 mov r14,rax
            r14 = rax;
            // 29d29 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 29d2d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d32 mov rdi,rbx
            rdi = rbx;
            pc = 0x29d35;
        }
        0x29d35 => {
            // block 29d35
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d35 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d3a lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 29d41 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x29d46;
        }
        0x29d46 => {
            // block 29d46
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d46 mov rsi,QWORD PTR [rip+0x1164bb] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 29d4d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 29d51 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29d58 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x29d5d;
        }
        0x29d5d => {
            // block 29d5d
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d5d lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 29d64 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d69 mov rdi,r14
            rdi = r14;
            // 29d6c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d71 mov r14,rax
            r14 = rax;
            // 29d74 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 29d7b call 0xe554e
            { let r = { let t = m.env.code_va(0xe554e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d80 jmp 0x29cf7
            pc = 0x29cf7; continue;
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

/// parity: P2v @0x5b7c0 size=699
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005b7c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5b7c0;
    loop {
        match pc {
        0x5b7c0 => {
            // block 5b7c0
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b7c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b7c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5b7c1 mov rbp,rsp
            rbp = rsp;
            // 5b7c4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5b7c6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5b7c8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 5b7ca push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5b7cc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5b7cd sub rsp,0x138
            { let (r, f) = sub::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // 5b7d4 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // 5b7d7 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 5b7d9 mov r14,rsi
            r14 = rsi;
            // 5b7dc mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 5b7e3 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 5b7ea lea r15,[rbp-0x140]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 5b7f1 mov rax,QWORD PTR [rip+0xe49d8] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 5b7f8 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 5b7fc mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // 5b803 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 5b807 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 5b80e mov rsi,r15
            rsi = r15;
            // 5b811 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b816 mov DWORD PTR [rbp-0x15c],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea4), ((rbx as u32 as u64)) as u32);
            // 5b81c mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 5b824 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // 5b82b mov r13,QWORD PTR [rip+0xe4b8e] # 0x1403c0
            r13 = m.r64(m.env.data_va(347021)) as u64;
            // 5b832 lea rax,[r13+0x18]
            rax = r13.wrapping_add(0x18);
            // 5b836 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 5b83d mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 5b844 add r13,0x40
            { let (r, f) = add::<64>(r13, 0x40u64);
              r13 = r;
              fl = f; }
            // 5b848 mov QWORD PTR [rbp-0xd8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r13) as u64);
            // 5b84f mov rdi,r15
            rdi = r15;
            // 5b852 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b857 mov rbx,QWORD PTR [rip+0xe4b4a] # 0x1403a8
            rbx = m.r64(m.env.data_va(346997)) as u64;
            // 5b85e add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 5b862 mov QWORD PTR [rbp-0x140],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rbx) as u64);
            // 5b869 mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // 5b874 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 5b87f mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 5b88a mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 5b895 mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // 5b89f xorps xmm0,xmm0
            x0 = 0;
            // 5b8a2 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 5b8a6 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 5b8ae lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5b8b2 mov rdi,r15
            rdi = r15;
            // 5b8b5 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b8ba lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5b8be call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b8c3 cmp r12d,0x1
            fl = sub::<32>((r12 as u32 as u64), 0x1u64).1;
            // 5b8c7 ja 0x5b92a
            if fl.a() { pc = 0x5b92a; continue; }
            pc = 0x5b8c9;
        }
        0x5b8c9 => {
            // block 5b8c9
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b8c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b8c9 mov eax,DWORD PTR [r14+0xc]
            rax = (m.r32(r14.wrapping_add(0xc)) as u64) as u32 as u64;
            // 5b8cd cmp eax,0xb
            fl = sub::<32>((rax as u32 as u64), 0xbu64).1;
            // 5b8d0 ja 0x5b8dc
            if fl.a() { pc = 0x5b8dc; continue; }
            pc = 0x5b8d2;
        }
        0x5b8d2 => {
            // block 5b8d2
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b8d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b8d2 mov ecx,0xf52
            rcx = (0xf52u64) as u32 as u64;
            // 5b8d7 bt ecx,eax
            fl.cf = bit::<32>((rcx as u32 as u64), (rax as u32 as u64));
            // 5b8da jb 0x5b92a
            if fl.b() { pc = 0x5b92a; continue; }
            pc = 0x5b8dc;
        }
        0x5b8dc => {
            // block 5b8dc
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b8dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b8dc lea rsi,[rip+0xcf3c3] # 0x12aca6
            rsi = m.env.data_va(259187);
            // 5b8e3 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 5b8ea mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 5b8ef call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b8f4 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // 5b8f7 lea rcx,[rip+0xcf3ac] # 0x12acaa
            rcx = m.env.data_va(259191);
            // 5b8fe lea rsi,[rip+0xcf3a3] # 0x12aca8
            rsi = m.env.data_va(259189);
            // 5b905 cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 5b909 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 5b90e mov rdi,rax
            rdi = rax;
            // 5b911 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b916 lea rsi,[rip+0xce977] # 0x12a294
            rsi = m.env.data_va(256609);
            // 5b91d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 5b922 mov rdi,rax
            rdi = rax;
            // 5b925 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5b92a;
        }
        0x5b92a => {
            // block 5b92a
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b92a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b92a lea rsi,[rip+0xcf37b] # 0x12acac
            rsi = m.env.data_va(259193);
            // 5b931 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 5b938 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 5b93d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b942 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5b945 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 5b949 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 5b94d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5b950 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5b953 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 5b957 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5b959 cmp r12d,0x1
            fl = sub::<32>((r12 as u32 as u64), 0x1u64).1;
            // 5b95d ja 0x5b97c
            if fl.a() { pc = 0x5b97c; continue; }
            pc = 0x5b95f;
        }
        0x5b95f => {
            // block 5b95f
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b95f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b95f mov cl,BYTE PTR [r14+0x8]
            rcx = (rcx & !0xff) | ((m.r8(r14.wrapping_add(0x8)) as u64) as u8 as u64);
            // 5b963 lea rdx,[r14+0x4]
            rdx = r14.wrapping_add(0x4);
            // 5b967 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // 5b96a je 0x5b974
            if fl.e() { pc = 0x5b974; continue; }
            pc = 0x5b96c;
        }
        0x5b96c => {
            // block 5b96c
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b96c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b96c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 5b96e cmove rdx,r14
            if fl.e() { rdx = r14; }
            // 5b972 jmp 0x5b97a
            pc = 0x5b97a; continue;
        }
        0x5b974 => {
            // block 5b974
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b974 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b974 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 5b976 cmovne rdx,r14
            if fl.ne() { rdx = r14; }
            pc = 0x5b97a;
        }
        0x5b97a => {
            // block 5b97a
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b97a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b97a mov esi,DWORD PTR [rdx]
            rsi = (m.r32(rdx) as u64) as u32 as u64;
            pc = 0x5b97c;
        }
        0x5b97c => {
            // block 5b97c
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5b97c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b97c add esi,DWORD PTR [rbp-0x15c]
            { let (r, f) = add::<32>((rsi as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea4)) as u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5b982 mov rdi,rax
            rdi = rax;
            // 5b985 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b98a mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 5b991 mov rsi,r15
            rsi = r15;
            // 5b994 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b999 mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 5b9a0 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 5b9a7 mov QWORD PTR [rbp-0xd8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r13) as u64);
            // 5b9ae mov QWORD PTR [rbp-0x140],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rbx) as u64);
            // 5b9b5 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 5b9bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b9c1 mov rdi,r15
            rdi = r15;
            // 5b9c4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b9c9 mov rsi,QWORD PTR [rip+0xe4838] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 5b9d0 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 5b9d4 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 5b9db call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b9e0 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 5b9e7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b9ec mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 5b9f3 add rsp,0x138
            { let (r, f) = add::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // 5b9fa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5b9fb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5b9fd pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5b9ff pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5ba01 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5ba03 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5ba04 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5ba05 => {
            // block 5ba05
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5ba05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ba05 mov r14,rax
            r14 = rax;
            // 5ba08 mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 5ba0f mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 5ba16 mov QWORD PTR [rbp-0xd8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r13) as u64);
            // 5ba1d mov QWORD PTR [rbp-0x140],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rbx) as u64);
            // 5ba24 jmp 0x5ba3c
            pc = 0x5ba3c; continue;
        }
        0x5ba26 => {
            // block 5ba26
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5ba26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ba26 mov r14,rax
            r14 = rax;
            // 5ba29 jmp 0x5ba67
            pc = 0x5ba67; continue;
        }
        0x5ba2b => {
            // block 5ba2b
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5ba2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ba2b mov r14,rax
            r14 = rax;
            // 5ba2e jmp 0x5ba50
            pc = 0x5ba50; continue;
        }
        0x5ba30 => {
            // block 5ba30
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5ba30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ba30 mov r14,rax
            r14 = rax;
            // 5ba33 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5ba37 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5ba3c;
        }
        0x5ba3c => {
            // block 5ba3c
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5ba3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ba3c lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 5ba43 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5ba48 mov rdi,r15
            rdi = r15;
            // 5ba4b call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5ba50;
        }
        0x5ba50 => {
            // block 5ba50
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5ba50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ba50 mov rsi,QWORD PTR [rip+0xe47b1] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 5ba57 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 5ba5b lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 5ba62 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5ba67;
        }
        0x5ba67 => {
            // block 5ba67
            #[cfg(feature = "trace")] eprintln!("f_0005b7c0 pc 5ba67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ba67 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 5ba6e call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5ba73 mov rdi,r14
            rdi = r14;
            // 5ba76 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18ca1 size=682
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018ca1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x18ca1;
    loop {
        match pc {
        0x18ca1 => {
            // block 18ca1
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18ca1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18ca1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18ca2 mov rbp,rsp
            rbp = rsp;
            // 18ca5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18ca7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18ca9 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18cab push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18cac sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 18cb0 mov r15,rsi
            r15 = rsi;
            // 18cb3 mov r14,rdi
            r14 = rdi;
            // 18cb6 lea rsi,[rip+0x1116e5] # 0x12a3a2
            rsi = m.env.data_va(256879);
            // 18cbd mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 18cc2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18cc7 mov rbx,rax
            rbx = rax;
            // 18cca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18ccd mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18cd1 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18cd4 lea r12,[rbp-0x48]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18cd8 mov rdi,r12
            rdi = r12;
            // 18cdb call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ce0 mov rsi,QWORD PTR [rip+0x1274a9] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18ce7 mov rdi,r12
            rdi = r12;
            // 18cea call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18cef mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18cf2 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18cf6 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18cfb mov rdi,rax
            rdi = rax;
            // 18cfe call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d00 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18d03 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18d07 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d0c movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18d10 mov rdi,rbx
            rdi = rbx;
            // 18d13 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d18 mov rdi,rbx
            rdi = rbx;
            // 18d1b call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d20 lea rsi,[rip+0x111690] # 0x12a3b7
            rsi = m.env.data_va(256900);
            // 18d27 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18d2c mov rdi,rbx
            rdi = rbx;
            // 18d2f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d34 mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 18d37 mov rdi,rax
            rdi = rax;
            // 18d3a call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d3f mov rbx,rax
            rbx = rax;
            // 18d42 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18d45 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18d49 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18d4c lea r12,[rbp-0x40]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18d50 mov rdi,r12
            rdi = r12;
            // 18d53 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d58 mov rsi,QWORD PTR [rip+0x127431] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18d5f mov rdi,r12
            rdi = r12;
            // 18d62 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d67 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18d6a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18d6e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18d73 mov rdi,rax
            rdi = rax;
            // 18d76 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d78 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18d7b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18d7f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d84 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18d88 mov rdi,rbx
            rdi = rbx;
            // 18d8b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d90 mov rdi,rbx
            rdi = rbx;
            // 18d93 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d98 lea rsi,[rip+0x11162a] # 0x12a3c9
            rsi = m.env.data_va(256918);
            // 18d9f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18da4 mov rdi,rbx
            rdi = rbx;
            // 18da7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18dac mov esi,DWORD PTR [r15+0x4]
            rsi = (m.r32(r15.wrapping_add(0x4)) as u64) as u32 as u64;
            // 18db0 mov rdi,rax
            rdi = rax;
            // 18db3 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18db8 mov rbx,rax
            rbx = rax;
            // 18dbb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18dbe mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18dc2 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18dc5 lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18dc9 mov rdi,r12
            rdi = r12;
            // 18dcc call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18dd1 mov rsi,QWORD PTR [rip+0x1273b8] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18dd8 mov rdi,r12
            rdi = r12;
            // 18ddb call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18de0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18de3 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18de7 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18dec mov rdi,rax
            rdi = rax;
            // 18def call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18df1 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18df4 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18df8 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18dfd movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18e01 mov rdi,rbx
            rdi = rbx;
            // 18e04 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e09 mov rdi,rbx
            rdi = rbx;
            // 18e0c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e11 lea rsi,[rip+0x1115c3] # 0x12a3db
            rsi = m.env.data_va(256936);
            // 18e18 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18e1d mov rdi,rbx
            rdi = rbx;
            // 18e20 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e25 mov esi,DWORD PTR [r15+0x8]
            rsi = (m.r32(r15.wrapping_add(0x8)) as u64) as u32 as u64;
            // 18e29 mov rdi,rax
            rdi = rax;
            // 18e2c call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e31 mov rbx,rax
            rbx = rax;
            // 18e34 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18e37 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18e3b add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18e3e lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18e42 mov rdi,r12
            rdi = r12;
            // 18e45 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e4a mov rsi,QWORD PTR [rip+0x12733f] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18e51 mov rdi,r12
            rdi = r12;
            // 18e54 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e59 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18e5c mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18e60 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18e65 mov rdi,rax
            rdi = rax;
            // 18e68 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e6a mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18e6d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18e71 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e76 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18e7a mov rdi,rbx
            rdi = rbx;
            // 18e7d call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e82 mov rdi,rbx
            rdi = rbx;
            // 18e85 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e8a lea rsi,[rip+0x11155c] # 0x12a3ed
            rsi = m.env.data_va(256954);
            // 18e91 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18e96 mov rdi,rbx
            rdi = rbx;
            // 18e99 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e9e mov esi,DWORD PTR [r15+0xc]
            rsi = (m.r32(r15.wrapping_add(0xc)) as u64) as u32 as u64;
            // 18ea2 mov rdi,rax
            rdi = rax;
            // 18ea5 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18eaa mov rbx,rax
            rbx = rax;
            // 18ead mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18eb0 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18eb4 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18eb7 lea r15,[rbp-0x28]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 18ebb mov rdi,r15
            rdi = r15;
            // 18ebe call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ec3 mov rsi,QWORD PTR [rip+0x1272c6] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18eca mov rdi,r15
            rdi = r15;
            // 18ecd call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ed2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18ed5 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18ed9 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18ede mov rdi,rax
            rdi = rax;
            // 18ee1 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ee3 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18ee6 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 18eea call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18eef movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // 18ef3 mov rdi,rbx
            rdi = rbx;
            // 18ef6 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18efb mov rdi,rbx
            rdi = rbx;
            // 18efe call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18f03 mov rax,r14
            rax = r14;
            // 18f06 add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 18f0a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18f0b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18f0d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18f0f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18f11 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18f12 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x18f13 => {
            // block 18f13
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f13 mov rbx,rax
            rbx = rax;
            // 18f16 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18f1a jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f1c => {
            // block 18f1c
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f1c mov rbx,rax
            rbx = rax;
            // 18f1f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18f23 jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f25 => {
            // block 18f25
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f25 mov rbx,rax
            rbx = rax;
            // 18f28 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18f2c jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f2e => {
            // block 18f2e
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f2e mov rbx,rax
            rbx = rax;
            // 18f31 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18f35 jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f37 => {
            // block 18f37
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f37 mov rbx,rax
            rbx = rax;
            // 18f3a lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            pc = 0x18f3e;
        }
        0x18f3e => {
            // block 18f3e
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f3e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18f43 mov rdi,rbx
            rdi = rbx;
            // 18f46 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7d3c size=644
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007d3c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x7d3c;
    loop {
        match pc {
        0x7d3c => {
            // block 7d3c
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d3c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7d3d mov rbp,rsp
            rbp = rsp;
            // 7d40 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7d42 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7d44 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7d46 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7d48 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7d49 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7d4d mov r12,rsi
            r12 = rsi;
            // 7d50 mov r13,rdi
            r13 = rdi;
            // 7d53 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7d57 mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 7d5c sub rsi,rax
            { let (r, f) = sub::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 7d5f sar rsi,0x3
            { let (r, f) = sar::<64>(rsi, 0x3u64, fl); rsi = r; fl = f; }
            // 7d63 cmp rsi,0x400
            fl = sub::<64>(rsi, 0x400u64).1;
            // 7d6a jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7d70;
        }
        0x7d70 => {
            // block 7d70
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d70 mov rsi,QWORD PTR [rdx+0x8]
            rsi = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 7d74 sub rsi,QWORD PTR [rdx]
            { let (r, f) = sub::<64>(rsi, m.r64(rdx) as u64);
              rsi = r;
              fl = f; }
            // 7d77 sar rsi,0x3
            { let (r, f) = sar::<64>(rsi, 0x3u64, fl); rsi = r; fl = f; }
            // 7d7b cmp rsi,0x400
            fl = sub::<64>(rsi, 0x400u64).1;
            // 7d82 jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7d88;
        }
        0x7d88 => {
            // block 7d88
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d88 mov rsi,rcx
            rsi = rcx;
            // 7d8b mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 7d8f sub rcx,QWORD PTR [rsi]
            { let (r, f) = sub::<64>(rcx, m.r64(rsi) as u64);
              rcx = r;
              fl = f; }
            // 7d92 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 7d96 cmp rcx,0x400
            fl = sub::<64>(rcx, 0x400u64).1;
            // 7d9d jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7da3;
        }
        0x7da3 => {
            // block 7da3
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7da3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7da3 mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // 7da7 mov QWORD PTR [rbp-0x38],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rsi) as u64);
            // 7dab mov ebx,0x8
            rbx = (0x8u64) as u32 as u64;
            // 7db0 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 7db6 movsd xmm1,QWORD PTR [rip+0xf6b1a] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7dbe mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            // 7dc4 jmp 0x7dd9
            pc = 0x7dd9; continue;
        }
        0x7dc6 => {
            // block 7dc6
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc6 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7dca add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 7dce inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 7dd1 movsd xmm1,QWORD PTR [rip+0xf6aff] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            pc = 0x7dd9;
        }
        0x7dd9 => {
            // block 7dd9
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd9 movsd xmm0,QWORD PTR [rax+rbx*1-0x8]
            x0 = m.r64(rax.wrapping_add(rbx).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7ddf addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7de3 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7de8 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7dea mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7def js 0x7dfd
            if fl.s() { pc = 0x7dfd; continue; }
            pc = 0x7df1;
        }
        0x7df1 => {
            // block 7df1
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7df1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7df1 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7df7 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7dfb mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7dfd;
        }
        0x7dfd => {
            // block 7dfd
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dfd lea rsi,[r14+0x1ff]
            rsi = r14.wrapping_add(0x1ff);
            // 7e04 movsd xmm0,QWORD PTR [rax+rbx*1]
            x0 = m.r64(rax.wrapping_add(rbx)) as u128;
            // 7e09 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7e0d cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7e12 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7e14 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7e16 js 0x7e24
            if fl.s() { pc = 0x7e24; continue; }
            pc = 0x7e18;
        }
        0x7e18 => {
            // block 7e18
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e18 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7e1e cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7e22 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7e24;
        }
        0x7e24 => {
            // block 7e24
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e24 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7e27 shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7e2a add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e2c mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7e30 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7e35 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7e38 mov rdi,r13
            rdi = r13;
            // 7e3b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7e3e cmp r14,0x1ff
            fl = sub::<64>(r14, 0x1ffu64).1;
            // 7e45 ja 0x7e4f
            if fl.a() { pc = 0x7e4f; continue; }
            pc = 0x7e47;
        }
        0x7e47 => {
            // block 7e47
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e47 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7e49 jne 0x7dc6
            if fl.ne() { pc = 0x7dc6; continue; }
            pc = 0x7e4f;
        }
        0x7e4f => {
            // block 7e4f
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e4f mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // 7e55 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 7e5a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7e5c je 0x7efc
            if fl.e() { pc = 0x7efc; continue; }
            pc = 0x7e62;
        }
        0x7e62 => {
            // block 7e62
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e62 mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            // 7e68 mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7e6c;
        }
        0x7e6c => {
            // block 7e6c
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e6c mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7e70 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 7e73 movsd xmm0,QWORD PTR [rax+r14*1-0x8]
            x0 = m.r64(rax.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7e7a movsd xmm1,QWORD PTR [rip+0xf6a56] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7e82 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7e86 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7e8b test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7e8d mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7e92 js 0x7ea0
            if fl.s() { pc = 0x7ea0; continue; }
            pc = 0x7e94;
        }
        0x7e94 => {
            // block 7e94
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e94 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7e9a cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7e9e mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7ea0;
        }
        0x7ea0 => {
            // block 7ea0
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ea0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ea0 lea rsi,[rbx+0x3ff]
            rsi = rbx.wrapping_add(0x3ff);
            // 7ea7 movsd xmm0,QWORD PTR [rax+r14*1]
            x0 = m.r64(rax.wrapping_add(r14)) as u128;
            // 7ead addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7eb1 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7eb6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7eb8 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7eba js 0x7ec8
            if fl.s() { pc = 0x7ec8; continue; }
            pc = 0x7ebc;
        }
        0x7ebc => {
            // block 7ebc
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ebc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ebc cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7ec2 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7ec6 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7ec8;
        }
        0x7ec8 => {
            // block 7ec8
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ec8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ec8 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7ecb shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7ece add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ed0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7ed4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7ed9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7edc mov rdi,r13
            rdi = r13;
            // 7edf call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7ee2 cmp rbx,0x1ff
            fl = sub::<64>(rbx, 0x1ffu64).1;
            // 7ee9 ja 0x7f00
            if fl.a() { pc = 0x7f00; continue; }
            pc = 0x7eeb;
        }
        0x7eeb => {
            // block 7eeb
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7eeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eeb add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // 7eef inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7ef2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7ef4 jne 0x7e6c
            if fl.ne() { pc = 0x7e6c; continue; }
            pc = 0x7efa;
        }
        0x7efa => {
            // block 7efa
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7efa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7efa jmp 0x7f00
            pc = 0x7f00; continue;
        }
        0x7efc => {
            // block 7efc
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7efc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7efc mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7f00;
        }
        0x7f00 => {
            // block 7f00
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f00 movsd xmm1,QWORD PTR [rip+0xf69d0] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7f08 mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // 7f0e mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 7f13 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7f15 je 0x7fae
            if fl.e() { pc = 0x7fae; continue; }
            pc = 0x7f1b;
        }
        0x7f1b => {
            // block 7f1b
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f1b mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            pc = 0x7f21;
        }
        0x7f21 => {
            // block 7f21
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f21 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7f25 movsd xmm0,QWORD PTR [rax+r14*1-0x8]
            x0 = m.r64(rax.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7f2c addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7f30 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7f35 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7f37 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7f3c js 0x7f4a
            if fl.s() { pc = 0x7f4a; continue; }
            pc = 0x7f3e;
        }
        0x7f3e => {
            // block 7f3e
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f3e cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7f44 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7f48 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7f4a;
        }
        0x7f4a => {
            // block 7f4a
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f4a lea rsi,[rbx+0x5ff]
            rsi = rbx.wrapping_add(0x5ff);
            // 7f51 movsd xmm0,QWORD PTR [rax+r14*1]
            x0 = m.r64(rax.wrapping_add(r14)) as u128;
            // 7f57 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7f5b cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7f60 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7f62 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7f64 js 0x7f72
            if fl.s() { pc = 0x7f72; continue; }
            pc = 0x7f66;
        }
        0x7f66 => {
            // block 7f66
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f66 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7f6c cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7f70 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7f72;
        }
        0x7f72 => {
            // block 7f72
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f72 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7f75 shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7f78 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f7a mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7f7e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7f83 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7f86 mov rdi,r13
            rdi = r13;
            // 7f89 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7f8c cmp rbx,0x1ff
            fl = sub::<64>(rbx, 0x1ffu64).1;
            // 7f93 ja 0x7fb0
            if fl.a() { pc = 0x7fb0; continue; }
            pc = 0x7f95;
        }
        0x7f95 => {
            // block 7f95
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f95 add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // 7f99 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7f9c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7f9e movsd xmm1,QWORD PTR [rip+0xf6932] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7fa6 jne 0x7f21
            if fl.ne() { pc = 0x7f21; continue; }
            pc = 0x7fac;
        }
        0x7fac => {
            // block 7fac
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fac jmp 0x7fb0
            pc = 0x7fb0; continue;
        }
        0x7fae => {
            // block 7fae
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fae xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x7fb0;
        }
        0x7fb0 => {
            // block 7fb0
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fb0 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7fb4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7fb5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7fb7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7fb9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7fbb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7fbd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7fbe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x147d7 size=641
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000147d7<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x147d7;
    loop {
        match pc {
        0x147d7 => {
            // block 147d7
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 147d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 147d7 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 147d8 mov rbp,rsp
            rbp = rsp;
            // 147db push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 147dc push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 147dd mov rbx,rdi
            rbx = rdi;
            // 147e0 cmp esi,0x1032294f
            fl = sub::<32>((rsi as u32 as u64), 0x1032294fu64).1;
            // 147e6 jg 0x14808
            if fl.g() { pc = 0x14808; continue; }
            pc = 0x147e8;
        }
        0x147e8 => {
            // block 147e8
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 147e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 147e8 cmp esi,0x102802ff
            fl = sub::<32>((rsi as u32 as u64), 0x102802ffu64).1;
            // 147ee jg 0x1484c
            if fl.g() { pc = 0x1484c; continue; }
            pc = 0x147f0;
        }
        0x147f0 => {
            // block 147f0
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 147f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 147f0 cmp esi,0x10244800
            fl = sub::<32>((rsi as u32 as u64), 0x10244800u64).1;
            // 147f6 jne 0x14889
            if fl.ne() { pc = 0x14889; continue; }
            pc = 0x147fc;
        }
        0x147fc => {
            // block 147fc
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 147fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 147fc lea rsi,[rip+0x115972] # 0x12a175
            rsi = m.env.data_va(256322);
            // 14803 jmp 0x148f6
            pc = 0x148f6; continue;
        }
        0x14808 => {
            // block 14808
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14808 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14808 cmp esi,0x106344ff
            fl = sub::<32>((rsi as u32 as u64), 0x106344ffu64).1;
            // 1480e jg 0x1486c
            if fl.g() { pc = 0x1486c; continue; }
            pc = 0x14810;
        }
        0x14810 => {
            // block 14810
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14810 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14810 cmp esi,0x105183ff
            fl = sub::<32>((rsi as u32 as u64), 0x105183ffu64).1;
            // 14816 jg 0x148a1
            if fl.g() { pc = 0x148a1; continue; }
            pc = 0x1481c;
        }
        0x1481c => {
            // block 1481c
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1481c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1481c cmp esi,0x104020ff
            fl = sub::<32>((rsi as u32 as u64), 0x104020ffu64).1;
            // 14822 jg 0x14900
            if fl.g() { pc = 0x14900; continue; }
            pc = 0x14828;
        }
        0x14828 => {
            // block 14828
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14828 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14828 cmp esi,0x10322950
            fl = sub::<32>((rsi as u32 as u64), 0x10322950u64).1;
            // 1482e je 0x149f2
            if fl.e() { pc = 0x149f2; continue; }
            pc = 0x14834;
        }
        0x14834 => {
            // block 14834
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14834 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14834 cmp esi,0x10352300
            fl = sub::<32>((rsi as u32 as u64), 0x10352300u64).1;
            // 1483a jne 0x14a00
            if fl.ne() { pc = 0x14a00; continue; }
            pc = 0x14840;
        }
        0x14840 => {
            // block 14840
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14840 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14840 lea rsi,[rip+0x1159d6] # 0x12a21d
            rsi = m.env.data_va(256490);
            // 14847 jmp 0x148f6
            pc = 0x148f6; continue;
        }
        0x1484c => {
            // block 1484c
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1484c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1484c cmp esi,0x102946ff
            fl = sub::<32>((rsi as u32 as u64), 0x102946ffu64).1;
            // 14852 jg 0x148c2
            if fl.g() { pc = 0x148c2; continue; }
            pc = 0x14854;
        }
        0x14854 => {
            // block 14854
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14854 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14854 cmp esi,0x10280300
            fl = sub::<32>((rsi as u32 as u64), 0x10280300u64).1;
            // 1485a jne 0x14929
            if fl.ne() { pc = 0x14929; continue; }
            pc = 0x14860;
        }
        0x14860 => {
            // block 14860
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14860 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14860 lea rsi,[rip+0x11599d] # 0x12a204
            rsi = m.env.data_va(256465);
            // 14867 jmp 0x1496d
            pc = 0x1496d; continue;
        }
        0x1486c => {
            // block 1486c
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1486c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1486c cmp esi,0x106681ff
            fl = sub::<32>((rsi as u32 as u64), 0x106681ffu64).1;
            // 14872 jg 0x148d7
            if fl.g() { pc = 0x148d7; continue; }
            pc = 0x14874;
        }
        0x14874 => {
            // block 14874
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14874 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14874 cmp esi,0x10634500
            fl = sub::<32>((rsi as u32 as u64), 0x10634500u64).1;
            // 1487a jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x14880;
        }
        0x14880 => {
            // block 14880
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14880 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14880 lea rsi,[rip+0x115933] # 0x12a1ba
            rsi = m.env.data_va(256391);
            // 14887 jmp 0x148f6
            pc = 0x148f6; continue;
        }
        0x14889 => {
            // block 14889
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14889 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14889 cmp esi,0x10266400
            fl = sub::<32>((rsi as u32 as u64), 0x10266400u64).1;
            // 1488f jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x14895;
        }
        0x14895 => {
            // block 14895
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14895 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14895 lea rsi,[rip+0x11598d] # 0x12a229
            rsi = m.env.data_va(256502);
            // 1489c jmp 0x149f9
            pc = 0x149f9; continue;
        }
        0x148a1 => {
            // block 148a1
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 148a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148a1 cmp esi,0x105381ff
            fl = sub::<32>((rsi as u32 as u64), 0x105381ffu64).1;
            // 148a7 jg 0x14946
            if fl.g() { pc = 0x14946; continue; }
            pc = 0x148ad;
        }
        0x148ad => {
            // block 148ad
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 148ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148ad cmp esi,0x10518400
            fl = sub::<32>((rsi as u32 as u64), 0x10518400u64).1;
            // 148b3 jne 0x14988
            if fl.ne() { pc = 0x14988; continue; }
            pc = 0x148b9;
        }
        0x148b9 => {
            // block 148b9
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 148b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148b9 lea rsi,[rip+0x115917] # 0x12a1d7
            rsi = m.env.data_va(256420);
            // 148c0 jmp 0x148f6
            pc = 0x148f6; continue;
        }
        0x148c2 => {
            // block 148c2
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 148c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148c2 cmp esi,0x10294700
            fl = sub::<32>((rsi as u32 as u64), 0x10294700u64).1;
            // 148c8 jne 0x1495a
            if fl.ne() { pc = 0x1495a; continue; }
            pc = 0x148ce;
        }
        0x148ce => {
            // block 148ce
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 148ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148ce lea rsi,[rip+0x1158f1] # 0x12a1c6
            rsi = m.env.data_va(256403);
            // 148d5 jmp 0x1493c
            pc = 0x1493c; continue;
        }
        0x148d7 => {
            // block 148d7
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 148d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148d7 cmp esi,0x1071084f
            fl = sub::<32>((rsi as u32 as u64), 0x1071084fu64).1;
            // 148dd jg 0x14977
            if fl.g() { pc = 0x14977; continue; }
            pc = 0x148e3;
        }
        0x148e3 => {
            // block 148e3
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 148e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148e3 cmp esi,0x10668200
            fl = sub::<32>((rsi as u32 as u64), 0x10668200u64).1;
            // 148e9 jne 0x1499d
            if fl.ne() { pc = 0x1499d; continue; }
            pc = 0x148ef;
        }
        0x148ef => {
            // block 148ef
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 148ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148ef lea rsi,[rip+0x115946] # 0x12a23c
            rsi = m.env.data_va(256521);
            pc = 0x148f6;
        }
        0x148f6 => {
            // block 148f6
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 148f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148f6 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 148fb jmp 0x14a46
            pc = 0x14a46; continue;
        }
        0x14900 => {
            // block 14900
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14900 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14900 cmp esi,0x104782ff
            fl = sub::<32>((rsi as u32 as u64), 0x104782ffu64).1;
            // 14906 jg 0x149ba
            if fl.g() { pc = 0x149ba; continue; }
            pc = 0x1490c;
        }
        0x1490c => {
            // block 1490c
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1490c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1490c cmp esi,0x10402100
            fl = sub::<32>((rsi as u32 as u64), 0x10402100u64).1;
            // 14912 jne 0x14a11
            if fl.ne() { pc = 0x14a11; continue; }
            pc = 0x14918;
        }
        0x14918 => {
            // block 14918
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14918 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14918 lea rsi,[rip+0x115879] # 0x12a198
            rsi = m.env.data_va(256357);
            // 1491f mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 14924 jmp 0x14a46
            pc = 0x14a46; continue;
        }
        0x14929 => {
            // block 14929
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14929 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14929 cmp esi,0x10293000
            fl = sub::<32>((rsi as u32 as u64), 0x10293000u64).1;
            // 1492f jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x14935;
        }
        0x14935 => {
            // block 14935
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14935 lea rsi,[rip+0x115854] # 0x12a190
            rsi = m.env.data_va(256349);
            pc = 0x1493c;
        }
        0x1493c => {
            // block 1493c
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1493c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1493c mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 14941 jmp 0x14a46
            pc = 0x14a46; continue;
        }
        0x14946 => {
            // block 14946
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14946 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14946 cmp esi,0x10538200
            fl = sub::<32>((rsi as u32 as u64), 0x10538200u64).1;
            // 1494c jne 0x149cb
            if fl.ne() { pc = 0x149cb; continue; }
            pc = 0x1494e;
        }
        0x1494e => {
            // block 1494e
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1494e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1494e lea rsi,[rip+0x11585b] # 0x12a1b0
            rsi = m.env.data_va(256381);
            // 14955 jmp 0x14a20
            pc = 0x14a20; continue;
        }
        0x1495a => {
            // block 1495a
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1495a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1495a cmp esi,0x10294900
            fl = sub::<32>((rsi as u32 as u64), 0x10294900u64).1;
            // 14960 jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x14966;
        }
        0x14966 => {
            // block 14966
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14966 lea rsi,[rip+0x115814] # 0x12a181
            rsi = m.env.data_va(256334);
            pc = 0x1496d;
        }
        0x1496d => {
            // block 1496d
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1496d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1496d mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 14972 jmp 0x14a46
            pc = 0x14a46; continue;
        }
        0x14977 => {
            // block 14977
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14977 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14977 cmp esi,0x10710850
            fl = sub::<32>((rsi as u32 as u64), 0x10710850u64).1;
            // 1497d jne 0x149dc
            if fl.ne() { pc = 0x149dc; continue; }
            pc = 0x1497f;
        }
        0x1497f => {
            // block 1497f
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1497f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1497f lea rsi,[rip+0x1158c8] # 0x12a24e
            rsi = m.env.data_va(256539);
            // 14986 jmp 0x149eb
            pc = 0x149eb; continue;
        }
        0x14988 => {
            // block 14988
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14988 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14988 cmp esi,0x10518450
            fl = sub::<32>((rsi as u32 as u64), 0x10518450u64).1;
            // 1498e jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x14994;
        }
        0x14994 => {
            // block 14994
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14994 lea rsi,[rip+0x115848] # 0x12a1e3
            rsi = m.env.data_va(256432);
            // 1499b jmp 0x149eb
            pc = 0x149eb; continue;
        }
        0x1499d => {
            // block 1499d
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 1499d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1499d cmp esi,0x10710800
            fl = sub::<32>((rsi as u32 as u64), 0x10710800u64).1;
            // 149a3 jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x149a9;
        }
        0x149a9 => {
            // block 149a9
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149a9 lea rsi,[rip+0x115898] # 0x12a248
            rsi = m.env.data_va(256533);
            // 149b0 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 149b5 jmp 0x14a46
            pc = 0x14a46; continue;
        }
        0x149ba => {
            // block 149ba
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149ba cmp esi,0x10478300
            fl = sub::<32>((rsi as u32 as u64), 0x10478300u64).1;
            // 149c0 jne 0x14a27
            if fl.ne() { pc = 0x14a27; continue; }
            pc = 0x149c2;
        }
        0x149c2 => {
            // block 149c2
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149c2 lea rsi,[rip+0x115825] # 0x12a1ee
            rsi = m.env.data_va(256443);
            // 149c9 jmp 0x149f9
            pc = 0x149f9; continue;
        }
        0x149cb => {
            // block 149cb
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149cb cmp esi,0x10565400
            fl = sub::<32>((rsi as u32 as u64), 0x10565400u64).1;
            // 149d1 jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x149d3;
        }
        0x149d3 => {
            // block 149d3
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149d3 lea rsi,[rip+0x1157cc] # 0x12a1a6
            rsi = m.env.data_va(256371);
            // 149da jmp 0x14a20
            pc = 0x14a20; continue;
        }
        0x149dc => {
            // block 149dc
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149dc cmp esi,0x10710851
            fl = sub::<32>((rsi as u32 as u64), 0x10710851u64).1;
            // 149e2 jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x149e4;
        }
        0x149e4 => {
            // block 149e4
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149e4 lea rsi,[rip+0x11586e] # 0x12a259
            rsi = m.env.data_va(256550);
            pc = 0x149eb;
        }
        0x149eb => {
            // block 149eb
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149eb mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 149f0 jmp 0x14a46
            pc = 0x14a46; continue;
        }
        0x149f2 => {
            // block 149f2
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149f2 lea rsi,[rip+0x1157d5] # 0x12a1ce
            rsi = m.env.data_va(256411);
            pc = 0x149f9;
        }
        0x149f9 => {
            // block 149f9
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 149f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149f9 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 149fe jmp 0x14a46
            pc = 0x14a46; continue;
        }
        0x14a00 => {
            // block 14a00
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14a00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a00 cmp esi,0x10378800
            fl = sub::<32>((rsi as u32 as u64), 0x10378800u64).1;
            // 14a06 jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x14a08;
        }
        0x14a08 => {
            // block 14a08
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14a08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a08 lea rsi,[rip+0x115804] # 0x12a213
            rsi = m.env.data_va(256480);
            // 14a0f jmp 0x14a20
            pc = 0x14a20; continue;
        }
        0x14a11 => {
            // block 14a11
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14a11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a11 cmp esi,0x10416000
            fl = sub::<32>((rsi as u32 as u64), 0x10416000u64).1;
            // 14a17 jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x14a19;
        }
        0x14a19 => {
            // block 14a19
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14a19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a19 lea rsi,[rip+0x115812] # 0x12a232
            rsi = m.env.data_va(256511);
            pc = 0x14a20;
        }
        0x14a20 => {
            // block 14a20
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14a20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a20 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 14a25 jmp 0x14a46
            pc = 0x14a46; continue;
        }
        0x14a27 => {
            // block 14a27
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14a27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a27 cmp esi,0x10478350
            fl = sub::<32>((rsi as u32 as u64), 0x10478350u64).1;
            // 14a2d jne 0x14a3d
            if fl.ne() { pc = 0x14a3d; continue; }
            pc = 0x14a2f;
        }
        0x14a2f => {
            // block 14a2f
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14a2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a2f lea rsi,[rip+0x1157c1] # 0x12a1f7
            rsi = m.env.data_va(256452);
            // 14a36 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 14a3b jmp 0x14a46
            pc = 0x14a46; continue;
        }
        0x14a3d => {
            // block 14a3d
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14a3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a3d lea rsi,[rip+0x124284] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 14a44 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x14a46;
        }
        0x14a46 => {
            // block 14a46
            #[cfg(feature = "trace")] eprintln!("f_000147d7 pc 14a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a46 mov rdi,rbx
            rdi = rbx;
            // 14a49 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14a4e mov rax,rbx
            rax = rbx;
            // 14a51 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 14a55 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14a56 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14a57 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bfcc size=620
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bfcc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2bfcc;
    loop {
        match pc {
        0x2bfcc => {
            // block 2bfcc
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2bfcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bfcc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bfcd mov rbp,rsp
            rbp = rsp;
            // 2bfd0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bfd2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bfd4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2bfd6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bfd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bfd9 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2bfdd mov QWORD PTR [rbp-0x48],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rsi) as u64);
            // 2bfe1 mov rbx,rdi
            rbx = rdi;
            // 2bfe4 call 0x26172
            { let r = { let t = m.env.code_va(0x26172); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bfe9 lea rsi,[rip+0xfe914] # 0x12a904
            rsi = m.env.data_va(258257);
            // 2bff0 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 2bff5 mov rdi,rax
            rdi = rax;
            // 2bff8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bffd mov esi,DWORD PTR [rbx+0x20]
            rsi = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2c000 mov rdi,rax
            rdi = rax;
            // 2c003 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c008 lea rsi,[rip+0xfe938] # 0x12a947
            rsi = m.env.data_va(258324);
            // 2c00f mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 2c014 mov rdi,rax
            rdi = rax;
            // 2c017 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c01c lea rdi,[rbx+0x24]
            rdi = rbx.wrapping_add(0x24);
            // 2c020 mov rsi,rax
            rsi = rax;
            // 2c023 call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c028 lea rsi,[rip+0xfe926] # 0x12a955
            rsi = m.env.data_va(258338);
            // 2c02f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 2c034 mov rdi,rax
            rdi = rax;
            // 2c037 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c03c mov esi,DWORD PTR [rbx+0x4c]
            rsi = (m.r32(rbx.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2c03f mov rdi,rax
            rdi = rax;
            // 2c042 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c047 lea rsi,[rip+0xfe919] # 0x12a967
            rsi = m.env.data_va(258356);
            // 2c04e mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 2c053 mov rdi,rax
            rdi = rax;
            // 2c056 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c05b lea rdi,[rbx+0x50]
            rdi = rbx.wrapping_add(0x50);
            // 2c05f mov rsi,rax
            rsi = rax;
            // 2c062 call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c067 lea rsi,[rip+0xfe42f] # 0x12a49d
            rsi = m.env.data_va(257130);
            // 2c06e mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 2c073 mov rdi,rax
            rdi = rax;
            // 2c076 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c07b lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 2c07f mov rdi,rax
            rdi = rax;
            // 2c082 call 0x26804
            { let r = { let t = m.env.code_va(0x26804); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c087 mov rax,QWORD PTR [rbx+0x24]
            rax = m.r64(rbx.wrapping_add(0x24)) as u64;
            // 2c08b mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 2c08f test rax,rax
            fl = logic::<64>(rax & rax);
            // 2c092 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c098;
        }
        0x2c098 => {
            // block 2c098
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c098 mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 2c09c mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 2c0a0 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2c0a3 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c0a9;
        }
        0x2c0a9 => {
            // block 2c0a9
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c0a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c0a9 mov r14d,DWORD PTR [rbx+0x58]
            r14 = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 2c0ad shr r14d,1
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x1u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 2c0b0 test r14w,r14w
            fl = logic::<16>((r14 as u16 as u64) & (r14 as u16 as u64));
            // 2c0b4 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c0ba;
        }
        0x2c0ba => {
            // block 2c0ba
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c0ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c0ba mov r12d,DWORD PTR [rbx+0x2c]
            r12 = (m.r32(rbx.wrapping_add(0x2c)) as u64) as u32 as u64;
            // 2c0be shr r12d,0x4
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x4u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // 2c0c2 test r12w,r12w
            fl = logic::<16>((r12 as u16 as u64) & (r12 as u16 as u64));
            // 2c0c6 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c0cc;
        }
        0x2c0cc => {
            // block 2c0cc
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c0cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c0cc mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2c0d0 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 2c0d3 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c0d7 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // 2c0da mov rbx,rcx
            rbx = rcx;
            // 2c0dd lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2c0e1 mov rdi,r15
            rdi = r15;
            // 2c0e4 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c0e9 mov rsi,QWORD PTR [rip+0x1140a0] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2c0f0 mov rdi,r15
            rdi = r15;
            // 2c0f3 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c0f8 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c0fb mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2c0ff mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2c104 mov rdi,rax
            rdi = rax;
            // 2c107 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c109 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2c10c lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2c110 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c115 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // 2c119 mov rdi,rbx
            rdi = rbx;
            // 2c11c call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c121 mov rdi,rbx
            rdi = rbx;
            // 2c124 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c129 mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // 2c12c and r13d,0xffff
            { let r = (r13 as u32 as u64) & 0xffffu64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 2c133 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c139;
        }
        0x2c139 => {
            // block 2c139
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c139 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c139 movzx ecx,r12w
            rcx = ((r12 as u16 as u64)) as u32 as u64;
            // 2c13d mov DWORD PTR [rbp-0x54],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rcx as u32 as u64)) as u32);
            // 2c140 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 2c143 lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0x2c147;
        }
        0x2c147 => {
            // block 2c147
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c147 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c147 movzx r15d,r12w
            r15 = ((r12 as u16 as u64)) as u32 as u64;
            // 2c14b mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2c14f movzx ebx,WORD PTR [rax+r15*2]
            rbx = (m.r16(rax.wrapping_add(r15.wrapping_mul(2))) as u64) as u32 as u64;
            // 2c154 cmp ebx,ecx
            fl = sub::<32>((rbx as u32 as u64), (rcx as u32 as u64)).1;
            // 2c156 jae 0x2c1f8
            if fl.ae() { pc = 0x2c1f8; continue; }
            pc = 0x2c15c;
        }
        0x2c15c => {
            // block 2c15c
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c15c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c15c shl rbx,0x4
            { let (r, f) = shl::<64>(rbx, 0x4u64, fl); rbx = r; fl = f; }
            // 2c160 add rbx,QWORD PTR [rbp-0x50]
            { let (r, f) = add::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64);
              rbx = r;
              fl = f; }
            // 2c164 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 2c169 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2c16d lea rsi,[rip+0xfe807] # 0x12a97b
            rsi = m.env.data_va(258376);
            // 2c174 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c179 mov rdi,rax
            rdi = rax;
            // 2c17c mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2c17f call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c184 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 2c189 mov rdi,rax
            rdi = rax;
            // 2c18c lea rsi,[rip+0xfe7ed] # 0x12a980
            rsi = m.env.data_va(258381);
            // 2c193 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c198 mov rdi,rax
            rdi = rax;
            // 2c19b mov rsi,rbx
            rsi = rbx;
            // 2c19e call 0x2c238
            { let r = { let t = m.env.code_va(0x2c238); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1a3 mov r15,rax
            r15 = rax;
            // 2c1a6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2c1a9 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c1ad add rsi,r15
            { let (r, f) = add::<64>(rsi, r15);
              rsi = r;
              fl = f; }
            // 2c1b0 mov rdi,r14
            rdi = r14;
            // 2c1b3 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1b8 mov rdi,r14
            rdi = r14;
            // 2c1bb mov rsi,QWORD PTR [rip+0x113fce] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2c1c2 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1c7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c1ca mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2c1ce mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2c1d3 mov rdi,rax
            rdi = rax;
            // 2c1d6 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1d8 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2c1da mov rdi,r14
            rdi = r14;
            // 2c1dd call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1e2 movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // 2c1e5 mov rdi,r15
            rdi = r15;
            // 2c1e8 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1ed mov rdi,r15
            rdi = r15;
            // 2c1f0 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1f5 mov ecx,DWORD PTR [rbp-0x54]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64) as u32 as u64;
            pc = 0x2c1f8;
        }
        0x2c1f8 => {
            // block 2c1f8
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c1f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c1f8 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // 2c1fb movzx eax,r12w
            rax = ((r12 as u16 as u64)) as u32 as u64;
            // 2c1ff cmp eax,r13d
            fl = sub::<32>((rax as u32 as u64), (r13 as u32 as u64)).1;
            // 2c202 jb 0x2c147
            if fl.b() { pc = 0x2c147; continue; }
            pc = 0x2c208;
        }
        0x2c208 => {
            // block 2c208
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c208 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2c20c add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2c210 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2c211 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2c213 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2c215 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2c217 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2c219 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2c21a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2c21b => {
            // block 2c21b
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c21b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c21b mov rbx,rax
            rbx = rax;
            // 2c21e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0x2c222;
        }
        0x2c222 => {
            // block 2c222
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c222 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c222 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c227 mov rdi,rbx
            rdi = rbx;
            // 2c22a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c22f mov rbx,rax
            rbx = rax;
            // 2c232 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2c236 jmp 0x2c222
            pc = 0x2c222; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9d80 size=600
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_000e9d80<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9d80;
    loop {
        match pc {
        0xe9d80 => {
            // block e9d80
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9d80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d80 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9d81 mov rbp,rsp
            rbp = rsp;
            // e9d84 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9d86 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9d88 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9d8a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9d8c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9d8d sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // e9d91 mov rbx,rdi
            rbx = rdi;
            // e9d94 mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // e9d9a cmp QWORD PTR [rbx+0x78],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x78)) as u64, 0x0u64).1;
            // e9d9f je 0xe9f9b
            if fl.e() { pc = 0xe9f9b; continue; }
            pc = 0xe9da5;
        }
        0xe9da5 => {
            // block e9da5
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9da5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9da5 mov rdi,rbx
            rdi = rbx;
            // e9da8 call 0xeb900
            { let r = { let t = m.env.stub_va(0xcd); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9dad mov rsi,QWORD PTR [rbx+0x18]
            rsi = m.r64(rbx.wrapping_add(0x18)) as u64;
            // e9db1 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // e9db4 jne 0xe9dca
            if fl.ne() { pc = 0xe9dca; continue; }
            pc = 0xe9db6;
        }
        0xe9db6 => {
            // block e9db6
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9db6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9db6 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // e9dba lea rcx,[rbp-0x29]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // e9dbe mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // e9dc2 mov QWORD PTR [rbx+0x18],rsi
            m.w64(rbx.wrapping_add(0x18), (rsi) as u64);
            // e9dc6 mov QWORD PTR [rbx+0x20],rsi
            m.w64(rbx.wrapping_add(0x20), (rsi) as u64);
            pc = 0xe9dca;
        }
        0xe9dca => {
            // block e9dca
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9dca mov rcx,QWORD PTR [rbx+0x20]
            rcx = m.r64(rbx.wrapping_add(0x20)) as u64;
            // e9dce xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e9dd1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e9dd3 jne 0xe9df7
            if fl.ne() { pc = 0xe9df7; continue; }
            pc = 0xe9dd5;
        }
        0xe9dd5 => {
            // block e9dd5
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9dd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9dd5 mov rax,rcx
            rax = rcx;
            // e9dd8 sub rax,QWORD PTR [rbx+0x10]
            { let (r, f) = sub::<64>(rax, m.r64(rbx.wrapping_add(0x10)) as u64);
              rax = r;
              fl = f; }
            // e9ddc mov rdx,rax
            rdx = rax;
            // e9ddf shr rdx,0x3f
            { let (r, f) = shr::<64>(rdx, 0x3fu64, fl); rdx = r; fl = f; }
            // e9de3 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // e9de6 sar rdx,1
            { let (r, f) = sar::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e9de9 cmp rdx,0x4
            fl = sub::<64>(rdx, 0x4u64).1;
            // e9ded mov r14d,0x4
            r14 = (0x4u64) as u32 as u64;
            // e9df3 cmovbe r14,rdx
            if fl.be() { r14 = rdx; }
            pc = 0xe9df7;
        }
        0xe9df7 => {
            // block e9df7
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9df7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9df7 cmp rsi,rcx
            fl = sub::<64>(rsi, rcx).1;
            // e9dfa je 0xe9e09
            if fl.e() { pc = 0xe9e09; continue; }
            pc = 0xe9dfc;
        }
        0xe9dfc => {
            // block e9dfc
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9dfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9dfc movzx r15d,BYTE PTR [rsi]
            r15 = (m.r8(rsi) as u64) as u32 as u64;
            // e9e00 lea r13,[rbx+0x10]
            r13 = rbx.wrapping_add(0x10);
            // e9e04 jmp 0xe9f82
            pc = 0xe9f82; continue;
        }
        0xe9e09 => {
            // block e9e09
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9e09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9e09 lea r13,[rbx+0x10]
            r13 = rbx.wrapping_add(0x10);
            // e9e0d mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e9e11 sub rsi,r14
            { let (r, f) = sub::<64>(rsi, r14);
              rsi = r;
              fl = f; }
            // e9e14 mov rdx,r14
            rdx = r14;
            // e9e17 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9e1c cmp BYTE PTR [rbx+0x192],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x192)) as u64, 0x0u64).1;
            // e9e23 je 0xe9e73
            if fl.e() { pc = 0xe9e73; continue; }
            pc = 0xe9e25;
        }
        0xe9e25 => {
            // block e9e25
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9e25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9e25 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e9e29 mov rdx,QWORD PTR [rbx+0x20]
            rdx = m.r64(rbx.wrapping_add(0x20)) as u64;
            // e9e2d sub rdx,r14
            { let (r, f) = sub::<64>(rdx, r14);
              rdx = r;
              fl = f; }
            // e9e30 sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // e9e33 add rdi,r14
            { let (r, f) = add::<64>(rdi, r14);
              rdi = r;
              fl = f; }
            // e9e36 mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // e9e3a mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e9e3f call 0xec1ca
            { let r = { let t = m.env.stub_va(0x997); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9e44 mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // e9e4a test rax,rax
            fl = logic::<64>(rax & rax);
            // e9e4d je 0xe9f82
            if fl.e() { pc = 0xe9f82; continue; }
            pc = 0xe9e53;
        }
        0xe9e53 => {
            // block e9e53
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9e53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9e53 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e9e57 lea rdx,[rcx+r14*1]
            rdx = rcx.wrapping_add(r14);
            // e9e5b add rax,r14
            { let (r, f) = add::<64>(rax, r14);
              rax = r;
              fl = f; }
            // e9e5e add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // e9e61 mov QWORD PTR [rbx+0x18],rdx
            m.w64(rbx.wrapping_add(0x18), (rdx) as u64);
            // e9e65 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // e9e69 movzx r15d,BYTE PTR [rcx+r14*1]
            r15 = (m.r8(rcx.wrapping_add(r14)) as u64) as u32 as u64;
            // e9e6e jmp 0xe9f82
            pc = 0xe9f82; continue;
        }
        0xe9e73 => {
            // block e9e73
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9e73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9e73 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // e9e77 mov rsi,QWORD PTR [rbx+0x48]
            rsi = m.r64(rbx.wrapping_add(0x48)) as u64;
            // e9e7b mov rdx,QWORD PTR [rbx+0x50]
            rdx = m.r64(rbx.wrapping_add(0x50)) as u64;
            // e9e7f sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // e9e82 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9e87 mov rdx,QWORD PTR [rbx+0x40]
            rdx = m.r64(rbx.wrapping_add(0x40)) as u64;
            // e9e8b mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // e9e8f sub rax,QWORD PTR [rbx+0x48]
            { let (r, f) = sub::<64>(rax, m.r64(rbx.wrapping_add(0x48)) as u64);
              rax = r;
              fl = f; }
            // e9e93 add rax,rdx
            { let (r, f) = add::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // e9e96 mov QWORD PTR [rbx+0x48],rax
            m.w64(rbx.wrapping_add(0x48), (rax) as u64);
            // e9e9a lea rsi,[rbx+0x58]
            rsi = rbx.wrapping_add(0x58);
            // e9e9e mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // e9ea3 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // e9ea6 je 0xe9eac
            if fl.e() { pc = 0xe9eac; continue; }
            pc = 0xe9ea8;
        }
        0xe9ea8 => {
            // block e9ea8
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9ea8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ea8 mov rcx,QWORD PTR [rbx+0x60]
            rcx = m.r64(rbx.wrapping_add(0x60)) as u64;
            pc = 0xe9eac;
        }
        0xe9eac => {
            // block e9eac
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9eac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9eac add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // e9eaf mov QWORD PTR [rbx+0x50],rdx
            m.w64(rbx.wrapping_add(0x50), (rdx) as u64);
            // e9eb3 mov rcx,QWORD PTR [rbx+0x70]
            rcx = m.r64(rbx.wrapping_add(0x70)) as u64;
            // e9eb7 sub rcx,r14
            { let (r, f) = sub::<64>(rcx, r14);
              rcx = r;
              fl = f; }
            // e9eba sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // e9ebd cmp rdx,rcx
            fl = sub::<64>(rdx, rcx).1;
            // e9ec0 cmovae rdx,rcx
            if fl.ae() { rdx = rcx; }
            // e9ec4 lea rdi,[rbx+0x108]
            rdi = rbx.wrapping_add(0x108);
            // e9ecb lea r12,[rbx+0x88]
            r12 = rbx.wrapping_add(0x88);
            // e9ed2 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e9ed7 mov rsi,r12
            rsi = r12;
            // e9eda rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            // e9edd mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // e9ee1 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e9ee6 mov rdi,rax
            rdi = rax;
            // e9ee9 call 0xec1ca
            { let r = { let t = m.env.stub_va(0x997); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9eee mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // e9ef4 test rax,rax
            fl = logic::<64>(rax & rax);
            // e9ef7 je 0xe9f82
            if fl.e() { pc = 0xe9f82; continue; }
            pc = 0xe9efd;
        }
        0xe9efd => {
            // block e9efd
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9efd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9efd mov rdi,QWORD PTR [rbx+0x80]
            rdi = m.r64(rbx.wrapping_add(0x80)) as u64;
            // e9f04 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e9f07 je 0xe9fad
            if fl.e() { pc = 0xe9fad; continue; }
            pc = 0xe9f0d;
        }
        0xe9f0d => {
            // block e9f0d
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f0d lea r8,[rbx+0x48]
            r8 = rbx.wrapping_add(0x48);
            // e9f11 add rax,QWORD PTR [rbx+0x48]
            { let (r, f) = add::<64>(rax, m.r64(rbx.wrapping_add(0x48)) as u64);
              rax = r;
              fl = f; }
            // e9f15 mov QWORD PTR [rbx+0x50],rax
            m.w64(rbx.wrapping_add(0x50), (rax) as u64);
            // e9f19 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e9f1d mov rdx,QWORD PTR [rbx+0x40]
            rdx = m.r64(rbx.wrapping_add(0x40)) as u64;
            // e9f21 lea r9,[rcx+r14*1]
            r9 = rcx.wrapping_add(r14);
            // e9f25 add rcx,QWORD PTR [rbx+0x70]
            { let (r, f) = add::<64>(rcx, m.r64(rbx.wrapping_add(0x70)) as u64);
              rcx = r;
              fl = f; }
            // e9f29 mov r10,QWORD PTR [rdi]
            r10 = m.r64(rdi) as u64;
            // e9f2c lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // e9f30 mov QWORD PTR [rsp+0x8],rsi
            m.w64(rsp.wrapping_add(0x8), (rsi) as u64);
            // e9f35 mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // e9f39 mov rsi,r12
            rsi = r12;
            // e9f3c mov rcx,rax
            rcx = rax;
            // e9f3f call QWORD PTR [r10+0x20]
            { let r = { let t = m.r64(r10.wrapping_add(0x20)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9f43 cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e9f46 jne 0xe9f62
            if fl.ne() { pc = 0xe9f62; continue; }
            pc = 0xe9f48;
        }
        0xe9f48 => {
            // block e9f48
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f48 mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // e9f4c mov rcx,QWORD PTR [rbx+0x50]
            rcx = m.r64(rbx.wrapping_add(0x50)) as u64;
            // e9f50 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e9f54 mov QWORD PTR [rbx+0x18],rax
            m.w64(rbx.wrapping_add(0x18), (rax) as u64);
            // e9f58 mov QWORD PTR [rbx+0x20],rcx
            m.w64(rbx.wrapping_add(0x20), (rcx) as u64);
            // e9f5c movzx r15d,BYTE PTR [rax]
            r15 = (m.r8(rax) as u64) as u32 as u64;
            // e9f60 jmp 0xe9f82
            pc = 0xe9f82; continue;
        }
        0xe9f62 => {
            // block e9f62
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f62 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e9f66 mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // e9f6a add r14,rcx
            { let (r, f) = add::<64>(r14, rcx);
              r14 = r;
              fl = f; }
            // e9f6d cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // e9f70 je 0xe9f82
            if fl.e() { pc = 0xe9f82; continue; }
            pc = 0xe9f72;
        }
        0xe9f72 => {
            // block e9f72
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f72 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // e9f76 mov QWORD PTR [rbx+0x18],r14
            m.w64(rbx.wrapping_add(0x18), (r14) as u64);
            // e9f7a mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // e9f7e movzx r15d,BYTE PTR [r14]
            r15 = (m.r8(r14) as u64) as u32 as u64;
            pc = 0xe9f82;
        }
        0xe9f82 => {
            // block e9f82
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f82 lea rax,[rbp-0x29]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // e9f86 cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // e9f8a jne 0xe9f9b
            if fl.ne() { pc = 0xe9f9b; continue; }
            pc = 0xe9f8c;
        }
        0xe9f8c => {
            // block e9f8c
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f8c mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // e9f94 xorps xmm0,xmm0
            x0 = 0;
            // e9f97 movups XMMWORD PTR [rbx+0x18],xmm0
            m.w128(rbx.wrapping_add(0x18), x0);
            pc = 0xe9f9b;
        }
        0xe9f9b => {
            // block e9f9b
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f9b mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // e9f9e add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // e9fa2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9fa3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9fa5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9fa7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9fa9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e9fab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9fac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe9fad => {
            // block e9fad
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9fad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9fad mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // e9fb2 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9fb7 mov rbx,rax
            rbx = rax;
            // e9fba mov rdi,rbx
            rdi = rbx;
            // e9fbd call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9fc2 mov rsi,QWORD PTR [rip+0x56217] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // e9fc9 mov rdx,QWORD PTR [rip+0x561d8] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // e9fd0 mov rdi,rbx
            rdi = rbx;
            // e9fd3 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4cd0 size=592
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4cd0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4cd0;
    loop {
        match pc {
        0xe4cd0 => {
            // block e4cd0
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4cd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4cd0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4cd1 mov rbp,rsp
            rbp = rsp;
            // e4cd4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4cd6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4cd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4cd9 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // e4cdd mov r11d,edx
            r11 = ((rdx as u32 as u64)) as u32 as u64;
            // e4ce0 mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // e4ce3 mov rbx,rdi
            rbx = rdi;
            // e4ce6 mov r14,QWORD PTR [rip+0x5b6eb] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // e4ced mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // e4cf0 mov QWORD PTR [rbp-0x20],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (r14) as u64);
            // e4cf4 test r9d,r9d
            fl = logic::<32>((r9 as u32 as u64) & (r9 as u32 as u64));
            // e4cf7 je 0xe4cff
            if fl.e() { pc = 0xe4cff; continue; }
            pc = 0xe4cf9;
        }
        0xe4cf9 => {
            // block e4cf9
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4cf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4cf9 mov DWORD PTR [rbx+0x8],r9d
            m.w32(rbx.wrapping_add(0x8), ((r9 as u32 as u64)) as u32);
            // e4cfd jmp 0xe4d03
            pc = 0xe4d03; continue;
        }
        0xe4cff => {
            // block e4cff
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4cff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4cff mov r9d,DWORD PTR [rbx+0x8]
            r9 = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe4d03;
        }
        0xe4d03 => {
            // block e4d03
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d03 cmp r9d,0x4
            fl = sub::<32>((r9 as u32 as u64), 0x4u64).1;
            // e4d07 je 0xe4d0f
            if fl.e() { pc = 0xe4d0f; continue; }
            pc = 0xe4d09;
        }
        0xe4d09 => {
            // block e4d09
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d09 cmp r9d,0x8
            fl = sub::<32>((r9 as u32 as u64), 0x8u64).1;
            // e4d0d jne 0xe4d14
            if fl.ne() { pc = 0xe4d14; continue; }
            pc = 0xe4d0f;
        }
        0xe4d0f => {
            // block e4d0f
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d0f mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // e4d12 jmp 0xe4d17
            pc = 0xe4d17; continue;
        }
        0xe4d14 => {
            // block e4d14
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d14 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4d17;
        }
        0xe4d17 => {
            // block e4d17
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d17 lea eax,[r9-0x1]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // e4d1b mov esi,0x1e
            rsi = (0x1eu64) as u32 as u64;
            // e4d20 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e4d23 ja 0xe4d31
            if fl.a() { pc = 0xe4d31; continue; }
            pc = 0xe4d25;
        }
        0xe4d25 => {
            // block e4d25
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d25 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e4d27 lea rdx,[rip+0x44fb2] # 0x129ce0
            rdx = m.env.data_va(255149);
            // e4d2e mov esi,DWORD PTR [rdx+rax*4]
            rsi = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe4d31;
        }
        0xe4d31 => {
            // block e4d31
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d31 cmp esi,r10d
            fl = sub::<32>((rsi as u32 as u64), (r10 as u32 as u64)).1;
            // e4d34 ja 0xe4d43
            if fl.a() { pc = 0xe4d43; continue; }
            pc = 0xe4d36;
        }
        0xe4d36 => {
            // block e4d36
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d36 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4d38 mov eax,r10d
            rax = ((r10 as u32 as u64)) as u32 as u64;
            // e4d3b div esi
            { let (q, r) = div::<32>(rdx, rax, (rsi as u32 as u64)); rax = q; rdx = r; }
            // e4d3d add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // e4d40 mov r10d,edx
            r10 = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0xe4d43;
        }
        0xe4d43 => {
            // block e4d43
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d43 cmp r11d,0x3c
            fl = sub::<32>((r11 as u32 as u64), 0x3cu64).1;
            // e4d47 jb 0xe4d61
            if fl.b() { pc = 0xe4d61; continue; }
            pc = 0xe4d49;
        }
        0xe4d49 => {
            // block e4d49
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d49 mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // e4d4c mov edx,0x88888889
            rdx = (0x88888889u64) as u32 as u64;
            // e4d51 imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e4d55 shr rdx,0x25
            { let (r, f) = shr::<64>(rdx, 0x25u64, fl); rdx = r; fl = f; }
            // e4d59 add ecx,edx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4d5b imul eax,edx,0x3c
            { let (r, f) = imul::<32>((rdx as u32 as u64), 0x3cu64); rax = (r) as u32 as u64; fl = f; }
            // e4d5e sub r11d,eax
            { let (r, f) = sub::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4d61;
        }
        0xe4d61 => {
            // block e4d61
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d61 cmp ecx,0x3c
            fl = sub::<32>((rcx as u32 as u64), 0x3cu64).1;
            // e4d64 jb 0xe4d7d
            if fl.b() { pc = 0xe4d7d; continue; }
            pc = 0xe4d66;
        }
        0xe4d66 => {
            // block e4d66
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d66 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e4d68 mov edx,0x88888889
            rdx = (0x88888889u64) as u32 as u64;
            // e4d6d imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e4d71 shr rdx,0x25
            { let (r, f) = shr::<64>(rdx, 0x25u64, fl); rdx = r; fl = f; }
            // e4d75 add r8d,edx
            { let (r, f) = add::<32>((r8 as u32 as u64), (rdx as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4d78 imul eax,edx,0x3c
            { let (r, f) = imul::<32>((rdx as u32 as u64), 0x3cu64); rax = (r) as u32 as u64; fl = f; }
            // e4d7b sub ecx,eax
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4d7d;
        }
        0xe4d7d => {
            // block e4d7d
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d7d cmp r8d,0x18
            fl = sub::<32>((r8 as u32 as u64), 0x18u64).1;
            // e4d81 jb 0xe4d9c
            if fl.b() { pc = 0xe4d9c; continue; }
            pc = 0xe4d83;
        }
        0xe4d83 => {
            // block e4d83
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d83 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // e4d86 mov edx,0xaaaaaaab
            rdx = (0xaaaaaaabu64) as u32 as u64;
            // e4d8b imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e4d8f shr rdx,0x24
            { let (r, f) = shr::<64>(rdx, 0x24u64, fl); rdx = r; fl = f; }
            // e4d93 shl edx,0x3
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x3u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e4d96 lea eax,[rdx+rdx*2]
            rax = (rdx.wrapping_add(rdx.wrapping_mul(2))) as u32 as u64;
            // e4d99 sub r8d,eax
            { let (r, f) = sub::<32>((r8 as u32 as u64), (rax as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4d9c;
        }
        0xe4d9c => {
            // block e4d9c
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d9c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // e4d9f je 0xe4dfc
            if fl.e() { pc = 0xe4dfc; continue; }
            pc = 0xe4da1;
        }
        0xe4da1 => {
            // block e4da1
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4da1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4da1 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e4da3 mov edx,0xcccccccd
            rdx = (0xcccccccdu64) as u32 as u64;
            // e4da8 imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e4dac shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // e4db0 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4db2 cmp r9d,0x4
            fl = sub::<32>((r9 as u32 as u64), 0x4u64).1;
            // e4db6 jne 0xe4dd7
            if fl.ne() { pc = 0xe4dd7; continue; }
            pc = 0xe4db8;
        }
        0xe4db8 => {
            // block e4db8
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4db8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4db8 lea eax,[rdx+rdx*4]
            rax = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e4dbb cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // e4dbd mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // e4dc2 cmove eax,r10d
            if fl.e() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4dc6 test r11d,r11d
            fl = logic::<32>((r11 as u32 as u64) & (r11 as u32 as u64));
            // e4dc9 cmovne eax,r10d
            if fl.ne() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4dcd cmp r10d,0x1
            fl = sub::<32>((r10 as u32 as u64), 0x1u64).1;
            // e4dd1 cmovbe r10d,eax
            if fl.be() { r10 = ((rax as u32 as u64)) as u32 as u64; } else { r10 = r10 as u32 as u64; }
            // e4dd5 jmp 0xe4dfc
            pc = 0xe4dfc; continue;
        }
        0xe4dd7 => {
            // block e4dd7
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4dd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4dd7 cmp r9d,0x8
            fl = sub::<32>((r9 as u32 as u64), 0x8u64).1;
            // e4ddb mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            // e4de0 cmovne eax,r10d
            if fl.ne() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4de4 test r11d,r11d
            fl = logic::<32>((r11 as u32 as u64) & (r11 as u32 as u64));
            // e4de7 cmovne eax,r10d
            if fl.ne() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4deb cmp r10d,0x3
            fl = sub::<32>((r10 as u32 as u64), 0x3u64).1;
            // e4def lea edx,[rdx+rdx*4]
            rdx = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e4df2 cmova eax,r10d
            if fl.a() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4df6 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // e4df8 cmovne r10d,eax
            if fl.ne() { r10 = ((rax as u32 as u64)) as u32 as u64; } else { r10 = r10 as u32 as u64; }
            pc = 0xe4dfc;
        }
        0xe4dfc => {
            // block e4dfc
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4dfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4dfc mov DWORD PTR [rbx+0x50],r10d
            m.w32(rbx.wrapping_add(0x50), ((r10 as u32 as u64)) as u32);
            // e4e00 mov DWORD PTR [rbx+0x54],r11d
            m.w32(rbx.wrapping_add(0x54), ((r11 as u32 as u64)) as u32);
            // e4e04 mov DWORD PTR [rbx+0x58],ecx
            m.w32(rbx.wrapping_add(0x58), ((rcx as u32 as u64)) as u32);
            // e4e07 mov DWORD PTR [rbx+0x5c],r8d
            m.w32(rbx.wrapping_add(0x5c), ((r8 as u32 as u64)) as u32);
            // e4e0b test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // e4e0e je 0xe4e19
            if fl.e() { pc = 0xe4e19; continue; }
            pc = 0xe4e10;
        }
        0xe4e10 => {
            // block e4e10
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4e10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4e10 lea rsi,[rip+0x546c0] # 0x1394d7
            rsi = m.env.data_va(318628);
            // e4e17 jmp 0xe4e20
            pc = 0xe4e20; continue;
        }
        0xe4e19 => {
            // block e4e19
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4e19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4e19 lea rsi,[rip+0x546cb] # 0x1394eb
            rsi = m.env.data_va(318648);
            pc = 0xe4e20;
        }
        0xe4e20 => {
            // block e4e20
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4e20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4e20 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e4e24 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4e26 mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // e4e29 mov r8d,r11d
            r8 = ((r11 as u32 as u64)) as u32 as u64;
            // e4e2c mov r9d,r10d
            r9 = ((r10 as u32 as u64)) as u32 as u64;
            // e4e2f call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4e34 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // e4e38 lea rsi,[rbp-0x70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e4e3c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4e41 mov rdi,rbx
            rdi = rbx;
            // e4e44 call 0xe5ab6
            { let r = { let t = m.env.code_va(0xe5ab6); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4e49 mov WORD PTR [rbx+0xc],0x1
            m.w16(rbx.wrapping_add(0xc), (0x1u64) as u16);
            // e4e4f mov BYTE PTR [rbx+0xe],0x0
            m.w8(rbx.wrapping_add(0xe), (0x0u64) as u8);
            // e4e53 mov eax,0xfffffbcf
            rax = (0xfffffbcfu64) as u32 as u64;
            // e4e58 and eax,DWORD PTR [rbx+0x84]
            { let r = (rax as u32 as u64) & m.r32(rbx.wrapping_add(0x84)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4e5e mov BYTE PTR [rbx+0xf],0x0
            m.w8(rbx.wrapping_add(0xf), (0x0u64) as u8);
            // e4e62 mov BYTE PTR [rbx+0x14],r15b
            m.w8(rbx.wrapping_add(0x14), ((r15 as u8 as u64)) as u8);
            // e4e66 lea edx,[rax+0x400]
            rdx = (rax.wrapping_add(0x400)) as u32 as u64;
            // e4e6c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // e4e6f cmove edx,eax
            if fl.e() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e4e72 mov BYTE PTR [rbx+0x15],0x0
            m.w8(rbx.wrapping_add(0x15), (0x0u64) as u8);
            // e4e76 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e4e78 and eax,0xfffff7cf
            { let r = (rax as u32 as u64) & 0xfffff7cfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4e7d mov DWORD PTR [rbx+0x84],eax
            m.w32(rbx.wrapping_add(0x84), ((rax as u32 as u64)) as u32);
            // e4e83 mov eax,DWORD PTR [rbx+0x8]
            rax = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e4e86 lea esi,[rax-0x1]
            rsi = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // e4e89 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4e8b cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // e4e8e ja 0xe4eb1
            if fl.a() { pc = 0xe4eb1; continue; }
            pc = 0xe4e90;
        }
        0xe4e90 => {
            // block e4e90
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4e90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4e90 movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // e4e93 lea rdi,[rip+0x44ea6] # 0x129d40
            rdi = m.env.data_va(255245);
            // e4e9a mov esi,DWORD PTR [rdi+rsi*4]
            rsi = (m.r32(rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // e4e9d dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e4e9f cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // e4ea2 ja 0xe4eb1
            if fl.a() { pc = 0xe4eb1; continue; }
            pc = 0xe4ea4;
        }
        0xe4ea4 => {
            // block e4ea4
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4ea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4ea4 movsxd rcx,esi
            rcx = sx::<32>((rsi as u32 as u64)) as u64;
            // e4ea7 lea rsi,[rip+0x44e52] # 0x129d00
            rsi = m.env.data_va(255181);
            // e4eae mov ecx,DWORD PTR [rsi+rcx*4]
            rcx = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe4eb1;
        }
        0xe4eb1 => {
            // block e4eb1
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4eb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4eb1 and edx,0xf0ff7cf
            { let r = (rdx as u32 as u64) & 0xf0ff7cfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4eb7 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // e4eb9 and esi,0xd
            { let r = (rsi as u32 as u64) & 0xdu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e4ebc shl esi,0x14
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x14u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e4ebf or esi,edx
            { let r = (rsi as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e4ec1 and ecx,0x70
            { let r = (rcx as u32 as u64) & 0x70u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4ec4 shl ecx,0x18
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x18u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e4ec7 or ecx,esi
            { let r = (rcx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4ec9 mov DWORD PTR [rbx+0x84],ecx
            m.w32(rbx.wrapping_add(0x84), ((rcx as u32 as u64)) as u32);
            // e4ecf lea edx,[rax-0x5]
            rdx = (rax.wrapping_add(0xffff_ffff_ffff_fffb)) as u32 as u64;
            // e4ed2 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // e4ed5 jb 0xe4efb
            if fl.b() { pc = 0xe4efb; continue; }
            pc = 0xe4ed7;
        }
        0xe4ed7 => {
            // block e4ed7
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4ed7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4ed7 mov DWORD PTR [rbx+0x10],0x0
            m.w32(rbx.wrapping_add(0x10), (0x0u64) as u32);
            // e4ede and eax,0xfffffffb
            { let r = (rax as u32 as u64) & 0xfffffffbu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4ee1 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // e4ee4 jne 0xe4eef
            if fl.ne() { pc = 0xe4eef; continue; }
            pc = 0xe4ee6;
        }
        0xe4ee6 => {
            // block e4ee6
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4ee6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4ee6 and BYTE PTR [rbx+0x8b],0xf7
            { let r = m.r8(rbx.wrapping_add(0x8b)) as u64 & 0xf7u64; let f = logic::<8>(r);
              m.w8(rbx.wrapping_add(0x8b), (r) as u8);
              fl = f; }
            // e4eed jmp 0xe4efb
            pc = 0xe4efb; continue;
        }
        0xe4eef => {
            // block e4eef
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4eef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4eef and ecx,0x77dff7cf
            { let r = (rcx as u32 as u64) & 0x77dff7cfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4ef5 mov DWORD PTR [rbx+0x84],ecx
            m.w32(rbx.wrapping_add(0x84), ((rcx as u32 as u64)) as u32);
            pc = 0xe4efb;
        }
        0xe4efb => {
            // block e4efb
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4efb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4efb xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e4efd xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4eff xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4f01 mov rdi,rbx
            rdi = rbx;
            // e4f04 call 0xe5efc
            { let r = { let t = m.env.code_va(0xe5efc); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4f09 cmp r14,QWORD PTR [rbp-0x20]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // e4f0d jne 0xe4f1a
            if fl.ne() { pc = 0xe4f1a; continue; }
            pc = 0xe4f0f;
        }
        0xe4f0f => {
            // block e4f0f
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4f0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4f0f add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // e4f13 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4f14 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4f16 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4f18 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4f19 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4f1a => {
            // block e4f1a
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4f1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4f1a call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1e2e size=586
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e1e2e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe1e2e;
    loop {
        match pc {
        0xe1e2e => {
            // block e1e2e
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1e2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1e2e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1e2f mov rbp,rsp
            rbp = rsp;
            // e1e32 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e1e34 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1e36 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1e37 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e1e3e mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e1e41 mov r14,rdi
            r14 = rdi;
            // e1e44 call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1e49 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // e1e4b test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e1e4d je 0xe1ffb
            if fl.e() { pc = 0xe1ffb; continue; }
            pc = 0xe1e53;
        }
        0xe1e53 => {
            // block e1e53
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1e53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1e53 lea rdi,[r14+0x34]
            rdi = r14.wrapping_add(0x34);
            // e1e57 lea rsi,[rip+0x575d9] # 0x139437
            rsi = m.env.data_va(318468);
            // e1e5e mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e1e63 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e1e65 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // e1e68 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1e6d mov esi,0xfa
            rsi = (0xfau64) as u32 as u64;
            // e1e72 mov rdi,r14
            rdi = r14;
            // e1e75 call 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1e7a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e1e7c je 0xe1fca
            if fl.e() { pc = 0xe1fca; continue; }
            pc = 0xe1e82;
        }
        0xe1e82 => {
            // block e1e82
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1e82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1e82 xorps xmm0,xmm0
            x0 = 0;
            // e1e85 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // e1e89 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // e1e91 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e1e95 mov rdi,r14
            rdi = r14;
            // e1e98 call 0xe4a2e
            { let r = { let t = m.env.code_va(0xe4a2e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1e9d xorps xmm0,xmm0
            x0 = 0;
            // e1ea0 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // e1ea4 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // e1eac lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // e1eb0 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e1eb4 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1eb9 lea rsi,[rbp-0x68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // e1ebd lea rdx,[rbp-0x50]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e1ec1 call 0xe171e
            { let r = { let t = m.env.code_va(0xe171e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1ec6 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // e1eca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1ecf mov rbx,QWORD PTR [rbp-0x50]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e1ed3 cmp QWORD PTR [rbp-0x48],rbx
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, rbx).1;
            // e1ed7 je 0xe1fdf
            if fl.e() { pc = 0xe1fdf; continue; }
            pc = 0xe1edd;
        }
        0xe1edd => {
            // block e1edd
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1edd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1edd xorps xmm0,xmm0
            x0 = 0;
            // e1ee0 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // e1ee4 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // e1eec lea rsi,[rip+0x57520] # 0x139413
            rsi = m.env.data_va(318432);
            // e1ef3 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // e1efa mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e1eff call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1f04 lea rdx,[rbp-0x98]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // e1f0b lea rcx,[rbp-0x80]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // e1f0f mov rsi,rbx
            rsi = rbx;
            // e1f12 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1f17 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e1f19 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // e1f20 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1f25 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e1f27 je 0xe1fd6
            if fl.e() { pc = 0xe1fd6; continue; }
            pc = 0xe1f2d;
        }
        0xe1f2d => {
            // block e1f2d
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1f2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1f2d lea rsi,[rip+0x574e6] # 0x13941a
            rsi = m.env.data_va(318439);
            // e1f34 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // e1f38 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1f3d test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e1f3f je 0xe200b
            if fl.e() { pc = 0xe200b; continue; }
            pc = 0xe1f45;
        }
        0xe1f45 => {
            // block e1f45
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1f45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1f45 lea rsi,[rip+0x574d1] # 0x13941d
            rsi = m.env.data_va(318442);
            // e1f4c lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // e1f50 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1f55 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e1f57 jne 0xe1fd6
            if fl.ne() { pc = 0xe1fd6; continue; }
            pc = 0xe1f59;
        }
        0xe1f59 => {
            // block e1f59
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1f59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1f59 mov rbx,QWORD PTR [rbp-0x50]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e1f5d mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e1f61 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e1f64 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e1f68 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // e1f72 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e1f76 cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // e1f7a jb 0xe1fd6
            if fl.b() { pc = 0xe1fd6; continue; }
            pc = 0xe1f7c;
        }
        0xe1f7c => {
            // block e1f7c
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1f7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1f7c lea rsi,[rip+0x5749f] # 0x139422
            rsi = m.env.data_va(318447);
            // e1f83 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e1f8a mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e1f8f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1f94 add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e1f98 lea rcx,[r14+0x10]
            rcx = r14.wrapping_add(0x10);
            // e1f9c lea rdx,[rbp-0xb0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e1fa3 mov rsi,rbx
            rsi = rbx;
            // e1fa6 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1fab lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e1fb2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1fb7 mov DWORD PTR [r14+0x8],0x1a
            m.w32(r14.wrapping_add(0x8), (0x1au64) as u32);
            // e1fbf lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // e1fc3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1fc8 jmp 0xe1fe7
            pc = 0xe1fe7; continue;
        }
        0xe1fca => {
            // block e1fca
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1fca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1fca mov DWORD PTR [r14+0x8],0x15
            m.w32(r14.wrapping_add(0x8), (0x15u64) as u32);
            // e1fd2 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1fd4 jmp 0xe1ffb
            pc = 0xe1ffb; continue;
        }
        0xe1fd6 => {
            // block e1fd6
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1fd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1fd6 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // e1fda call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1fdf;
        }
        0xe1fdf => {
            // block e1fdf
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1fdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1fdf mov DWORD PTR [r14+0x8],0x17
            m.w32(r14.wrapping_add(0x8), (0x17u64) as u32);
            pc = 0xe1fe7;
        }
        0xe1fe7 => {
            // block e1fe7
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1fe7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1fe7 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe1fe9;
        }
        0xe1fe9 => {
            // block e1fe9
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1fe9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1fe9 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e1fed call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1ff2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e1ff6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1ffb;
        }
        0xe1ffb => {
            // block e1ffb
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e1ffb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1ffb mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e1ffd add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e2004 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2005 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2007 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2009 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e200a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe200b => {
            // block e200b
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e200b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e200b lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // e200f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2014 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // e2016 jmp 0xe1fe9
            pc = 0xe1fe9; continue;
        }
        0xe2018 => {
            // block e2018
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e2018 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2018 mov rbx,rax
            rbx = rax;
            // e201b jmp 0xe2056
            pc = 0xe2056; continue;
        }
        0xe201d => {
            // block e201d
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e201d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e201d mov rbx,rax
            rbx = rax;
            // e2020 jmp 0xe204d
            pc = 0xe204d; continue;
        }
        0xe2022 => {
            // block e2022
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e2022 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2022 mov rbx,rax
            rbx = rax;
            // e2025 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // e2029 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e202e jmp 0xe204d
            pc = 0xe204d; continue;
        }
        0xe2030 => {
            // block e2030
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e2030 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2030 mov rbx,rax
            rbx = rax;
            // e2033 jmp 0xe2044
            pc = 0xe2044; continue;
        }
        0xe2035 => {
            // block e2035
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e2035 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2035 mov rbx,rax
            rbx = rax;
            // e2038 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // e203f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2044;
        }
        0xe2044 => {
            // block e2044
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e2044 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2044 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // e2048 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe204d;
        }
        0xe204d => {
            // block e204d
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e204d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e204d lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e2051 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2056;
        }
        0xe2056 => {
            // block e2056
            #[cfg(feature = "trace")] eprintln!("f_000e1e2e pc e2056 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2056 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e205a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e205f mov rdi,rbx
            rdi = rbx;
            // e2062 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2067 mov rbx,rax
            rbx = rax;
            // e206a lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e2071 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2076 jmp 0xe2044
            pc = 0xe2044; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8380 size=536
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8380<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc8380;
    loop {
        match pc {
        0xc8380 => {
            // block c8380
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8380 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8380 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8381 mov rbp,rsp
            rbp = rsp;
            // c8384 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c8386 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c8388 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c838a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c838c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c838d mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // c838f mov DWORD PTR [rdi-0x4],eax
            m.w32(rdi.wrapping_add(0xffff_ffff_ffff_fffc), ((rax as u32 as u64)) as u32);
            // c8392 movsxd rbx,r8d
            rbx = sx::<32>((r8 as u32 as u64)) as u64;
            // c8395 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // c8398 movsxd r10,eax
            r10 = sx::<32>((rax as u32 as u64)) as u64;
            // c839b lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // c839e cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // c83a0 mov eax,DWORD PTR [rdi+rax*4]
            rax = (m.r32(rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // c83a3 mov DWORD PTR [rdi+r10*4],eax
            m.w32(rdi.wrapping_add(r10.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // c83a7 mov DWORD PTR [rdi+rbx*4],eax
            m.w32(rdi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // c83aa imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // c83ae mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // c83b0 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c83b1 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // c83b3 mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // c83b6 imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c83ba mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c83bc cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c83bd idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // c83bf mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c83c1 cmp r11d,ecx
            fl = sub::<32>((r11 as u32 as u64), (rcx as u32 as u64)).1;
            // c83c4 jge 0xc858d
            if fl.ge() { pc = 0xc858d; continue; }
            pc = 0xc83ca;
        }
        0xc83ca => {
            // block c83ca
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c83ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c83ca shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // c83ce mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // c83d1 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c83d2 idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // c83d5 mov DWORD PTR [rbp-0x3c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((rax as u32 as u64)) as u32);
            // c83d8 sub ecx,r11d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r11 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c83db movsxd rdx,r11d
            rdx = sx::<32>((r11 as u32 as u64)) as u64;
            // c83de lea rdx,[rsi+rdx*4]
            rdx = rsi.wrapping_add(rdx.wrapping_mul(4));
            // c83e2 imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            pc = 0xc83e6;
        }
        0xc83e6 => {
            // block c83e6
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c83e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c83e6 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // c83ea mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // c83ed mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // c83f0 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c83f3 mov ecx,r11d
            rcx = ((r11 as u32 as u64)) as u32 as u64;
            // c83f6 shr ecx,0xb
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0xbu64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c83f9 and ecx,0x1f
            { let r = (rcx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c83fc movsxd r9,eax
            r9 = sx::<32>((rax as u32 as u64)) as u64;
            // c83ff movzx eax,BYTE PTR [rdi+r9*4-0x2]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u32 as u64;
            // c8405 mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // c840a sub edx,ecx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c840c lea rbx,[rip+0x80fad] # 0x1493c0
            rbx = m.env.data_va(383885);
            // c8413 mov rsi,rbx
            rsi = rbx;
            // c8416 mov r10d,DWORD PTR [rsi+rdx*4]
            r10 = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // c841a imul eax,r10d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r10 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c841e movzx ebx,BYTE PTR [rdi+r9*4+0x2]
            rbx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x2)) as u64) as u32 as u64;
            // c8424 mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // c8429 sub edx,ecx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c842b mov r13d,DWORD PTR [rsi+rdx*4]
            r13 = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // c842f imul ebx,r13d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r13 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // c8433 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c8435 movzx edx,BYTE PTR [rdi+r9*4+0x6]
            rdx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x6)) as u64) as u32 as u64;
            // c843b mov eax,0x60
            rax = (0x60u64) as u32 as u64;
            // c8440 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c8442 mov r8d,DWORD PTR [rsi+rax*4]
            r8 = (m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // c8446 imul edx,r8d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r8 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // c844a add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c844c movzx eax,BYTE PTR [rdi+r9*4+0xa]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xa)) as u64) as u32 as u64;
            // c8452 mov ebx,0x80
            rbx = (0x80u64) as u32 as u64;
            // c8457 sub ebx,ecx
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c8459 mov r14d,DWORD PTR [rsi+rbx*4]
            r14 = (m.r32(rsi.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // c845d imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c8461 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c8463 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8465 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c8467 sar ecx,0x10
            { let (r, f) = sar::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c846a mov esi,0x0
            rsi = (0x0u64) as u32 as u64;
            // c846f js 0xc8486
            if fl.s() { pc = 0xc8486; continue; }
            pc = 0xc8471;
        }
        0xc8471 => {
            // block c8471
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8471 and eax,0xff0000
            { let r = (rax as u32 as u64) & 0xff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c8476 cmp ecx,0xff
            fl = sub::<32>((rcx as u32 as u64), 0xffu64).1;
            // c847c mov ecx,0xff0000
            rcx = (0xff0000u64) as u32 as u64;
            // c8481 cmovg eax,ecx
            if fl.g() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // c8484 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xc8486;
        }
        0xc8486 => {
            // block c8486
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8486 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8486 movzx eax,BYTE PTR [rdi+r9*4-0x3]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffd)) as u64) as u32 as u64;
            // c848c imul eax,r10d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r10 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c8490 movzx ecx,BYTE PTR [rdi+r9*4+0x1]
            rcx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x1)) as u64) as u32 as u64;
            // c8496 imul ecx,r13d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r13 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c849a add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c849c movzx ebx,BYTE PTR [rdi+r9*4+0x5]
            rbx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x5)) as u64) as u32 as u64;
            // c84a2 imul ebx,r8d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r8 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // c84a6 add ebx,ecx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c84a8 movzx eax,BYTE PTR [rdi+r9*4+0x9]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x9)) as u64) as u32 as u64;
            // c84ae imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c84b2 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c84b4 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c84b7 js 0xc84ce
            if fl.s() { pc = 0xc84ce; continue; }
            pc = 0xc84b9;
        }
        0xc84b9 => {
            // block c84b9
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c84b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c84b9 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c84bb shl ecx,0x8
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c84be cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // c84c3 movzx edx,cx
            rdx = ((rcx as u16 as u64)) as u32 as u64;
            // c84c6 mov eax,0xff00
            rax = (0xff00u64) as u32 as u64;
            // c84cb cmovg edx,eax
            if fl.g() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            pc = 0xc84ce;
        }
        0xc84ce => {
            // block c84ce
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c84ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c84ce movzx eax,BYTE PTR [rdi+r9*4-0x4]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // c84d4 imul eax,r10d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r10 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c84d8 movzx ecx,BYTE PTR [rdi+r9*4]
            rcx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4))) as u64) as u32 as u64;
            // c84dd imul ecx,r13d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r13 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c84e1 add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c84e3 movzx ebx,BYTE PTR [rdi+r9*4+0x4]
            rbx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x4)) as u64) as u32 as u64;
            // c84e9 imul ebx,r8d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r8 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // c84ed add ebx,ecx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c84ef movzx eax,BYTE PTR [rdi+r9*4+0x8]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // c84f5 imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c84f9 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c84fb xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c84fe sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c8501 mov r12d,0x0
            r12 = (0x0u64) as u32 as u64;
            // c8507 js 0xc851b
            if fl.s() { pc = 0xc851b; continue; }
            pc = 0xc8509;
        }
        0xc8509 => {
            // block c8509
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8509 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8509 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // c850e movzx r12d,al
            r12 = ((rax as u8 as u64)) as u32 as u64;
            // c8512 mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // c8517 cmovg r12d,eax
            if fl.g() { r12 = ((rax as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            pc = 0xc851b;
        }
        0xc851b => {
            // block c851b
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c851b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c851b movzx eax,BYTE PTR [rdi+r9*4-0x1]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // c8521 imul eax,r10d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r10 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c8525 movzx ecx,BYTE PTR [rdi+r9*4+0x3]
            rcx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x3)) as u64) as u32 as u64;
            // c852b imul ecx,r13d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r13 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c852f add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8531 movzx ebx,BYTE PTR [rdi+r9*4+0x7]
            rbx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x7)) as u64) as u32 as u64;
            // c8537 imul ebx,r8d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r8 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // c853b add ebx,ecx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c853d movzx eax,BYTE PTR [rdi+r9*4+0xb]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xb)) as u64) as u32 as u64;
            // c8543 imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c8547 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c8549 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c854c js 0xc8563
            if fl.s() { pc = 0xc8563; continue; }
            pc = 0xc854e;
        }
        0xc854e => {
            // block c854e
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c854e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c854e mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // c8551 shl r15d,0x18
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x18u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c8555 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // c855a mov eax,0xff000000
            rax = (0xff000000u64) as u32 as u64;
            // c855f cmovg r15d,eax
            if fl.g() { r15 = ((rax as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            pc = 0xc8563;
        }
        0xc8563 => {
            // block c8563
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8563 or r15d,esi
            { let r = (r15 as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c8566 and r15d,0xffff0000
            { let r = (r15 as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c856d or r12d,edx
            { let r = (r12 as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // c8570 or r12d,r15d
            { let r = (r12 as u32 as u64) | (r15 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // c8573 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c8577 mov DWORD PTR [rdx],r12d
            m.w32(rdx, ((r12 as u32 as u64)) as u32);
            // c857a add rdx,0x4
            { let (r, f) = add::<64>(rdx, 0x4u64);
              rdx = r;
              fl = f; }
            // c857e add r11d,DWORD PTR [rbp-0x3c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // c8582 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // c8585 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // c8587 jne 0xc83e6
            if fl.ne() { pc = 0xc83e6; continue; }
            pc = 0xc858d;
        }
        0xc858d => {
            // block c858d
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c858d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c858d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c858e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c8590 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c8592 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8594 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8596 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8597 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
