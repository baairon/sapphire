//! Mechanically lifted functions (scripts/lift_run.py), module q002. Each one is checked against the original
//! in the world harness (tests/world_q002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xdb6b6 size=2096
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_000db6b6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdb6b6;
    loop {
        match pc {
        0xdb6b6 => {
            // block db6b6
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db6b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db6b6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // db6b7 mov rbp,rsp
            rbp = rsp;
            // db6ba push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // db6bc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // db6be push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // db6c0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // db6c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // db6c3 sub rsp,0x168
            { let (r, f) = sub::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // db6ca mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // db6cd mov r13,rsi
            r13 = rsi;
            // db6d0 mov r15,rdi
            r15 = rdi;
            // db6d3 mov rdi,QWORD PTR [r15+0x6b8]
            rdi = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // db6da test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // db6dd je 0xdb86b
            if fl.e() { pc = 0xdb86b; continue; }
            pc = 0xdb6e3;
        }
        0xdb6e3 => {
            // block db6e3
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db6e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db6e3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // db6e6 mov esi,0x6
            rsi = (0x6u64) as u32 as u64;
            // db6eb call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db6ee mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // db6f1 xorps xmm0,xmm0
            x0 = 0;
            // db6f4 movaps XMMWORD PTR [rbp-0x160],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fea0), x0);
            // db6fb mov QWORD PTR [rbp-0x150],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0x0u64) as u64);
            // db706 lea rsi,[rbp-0x148]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // db70d mov rdi,r13
            rdi = r13;
            // db710 call 0xec2a2
            { let r = { let t = m.env.stub_va(0xa6f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db715 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // db717 jne 0xdbd0e
            if fl.ne() { pc = 0xdbd0e; continue; }
            pc = 0xdb71d;
        }
        0xdb71d => {
            // block db71d
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db71d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db71d mov rbx,QWORD PTR [rbp-0xe8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff18)) as u64;
            // db724 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // db727 js 0xdbd0e
            if fl.s() { pc = 0xdbd0e; continue; }
            pc = 0xdb72d;
        }
        0xdb72d => {
            // block db72d
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db72d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db72d mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // db730 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // db733 jg 0xdbd4e
            if fl.g() { pc = 0xdbd4e; continue; }
            pc = 0xdb739;
        }
        0xdb739 => {
            // block db739
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db739 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db739 lea rsi,[rip+0x4eb39] # 0x12a279
            rsi = m.env.data_va(256582);
            // db740 mov rdi,r13
            rdi = r13;
            // db743 call 0xec1c4
            { let r = { let t = m.env.stub_va(0x991); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db748 mov r12,rax
            r12 = rax;
            // db74b test r12,r12
            fl = logic::<64>(r12 & r12);
            // db74e je 0xdbd78
            if fl.e() { pc = 0xdbd78; continue; }
            pc = 0xdb754;
        }
        0xdb754 => {
            // block db754
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db754 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // db75b mov rsi,rbx
            rsi = rbx;
            // db75e call 0xebed0
            { let r = { let t = m.env.stub_va(0x69d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db763 mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // db76a mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // db76f mov rdx,rbx
            rdx = rbx;
            // db772 mov rcx,r12
            rcx = r12;
            // db775 call 0xec1ca
            { let r = { let t = m.env.stub_va(0x997); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db77a test rax,rax
            fl = logic::<64>(rax & rax);
            // db77d je 0xdbd9f
            if fl.e() { pc = 0xdbd9f; continue; }
            pc = 0xdb783;
        }
        0xdb783 => {
            // block db783
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db783 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db783 mov rcx,QWORD PTR [rbp-0x158]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // db78a sub rcx,QWORD PTR [rbp-0x160]
            { let (r, f) = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64);
              rcx = r;
              fl = f; }
            // db791 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // db794 jbe 0xdb7a5
            if fl.be() { pc = 0xdb7a5; continue; }
            pc = 0xdb796;
        }
        0xdb796 => {
            // block db796
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db796 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db796 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // db79d mov rsi,rax
            rsi = rax;
            // db7a0 call 0xebed0
            { let r = { let t = m.env.stub_va(0x69d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdb7a5;
        }
        0xdb7a5 => {
            // block db7a5
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db7a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db7a5 mov rdi,r12
            rdi = r12;
            // db7a8 call 0xec1b8
            { let r = { let t = m.env.stub_va(0x985); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db7ad mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // db7b0 mov rax,QWORD PTR [rax+0x108]
            rax = m.r64(rax.wrapping_add(0x108)) as u64;
            // db7b7 mov rdi,r15
            rdi = r15;
            // db7ba call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db7bc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db7be je 0xdbdce
            if fl.e() { pc = 0xdbdce; continue; }
            pc = 0xdb7c4;
        }
        0xdb7c4 => {
            // block db7c4
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db7c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db7c4 mov ebx,DWORD PTR [rbp-0x158]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64) as u32 as u64;
            // db7ca sub ebx,DWORD PTR [rbp-0x160]
            { let (r, f) = sub::<32>((rbx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // db7d0 mov rdi,QWORD PTR [r15+0x6b8]
            rdi = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // db7d7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // db7da mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // db7de mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // db7e1 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // db7e3 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db7e5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db7e7 je 0xdbdf5
            if fl.e() { pc = 0xdbdf5; continue; }
            pc = 0xdb7ed;
        }
        0xdb7ed => {
            // block db7ed
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db7ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db7ed mov r15,QWORD PTR [r15+0x6b8]
            r15 = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // db7f4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // db7f7 mov r12,QWORD PTR [rax+0x18]
            r12 = m.r64(rax.wrapping_add(0x18)) as u64;
            // db7fb lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // db802 lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // db809 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db80e lea rdx,[rbp-0x178]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // db815 mov rdi,r15
            rdi = r15;
            // db818 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // db81b mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // db81d call r12
            { let r = { let t = r12; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db820 mov rdi,QWORD PTR [rbp-0x178]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // db827 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // db82a je 0xdb841
            if fl.e() { pc = 0xdb841; continue; }
            pc = 0xdb82c;
        }
        0xdb82c => {
            // block db82c
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db82c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db82c cmp QWORD PTR [rbp-0x170],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64, rdi).1;
            // db833 je 0xdb83c
            if fl.e() { pc = 0xdb83c; continue; }
            pc = 0xdb835;
        }
        0xdb835 => {
            // block db835
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db835 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db835 mov QWORD PTR [rbp-0x170],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rdi) as u64);
            pc = 0xdb83c;
        }
        0xdb83c => {
            // block db83c
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db83c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db83c call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdb841;
        }
        0xdb841 => {
            // block db841
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db841 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db841 mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // db848 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // db84b je 0xdbcfc
            if fl.e() { pc = 0xdbcfc; continue; }
            pc = 0xdb851;
        }
        0xdb851 => {
            // block db851
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db851 cmp QWORD PTR [rbp-0x158],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64, rdi).1;
            // db858 je 0xdb861
            if fl.e() { pc = 0xdb861; continue; }
            pc = 0xdb85a;
        }
        0xdb85a => {
            // block db85a
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db85a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db85a mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            pc = 0xdb861;
        }
        0xdb861 => {
            // block db861
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db861 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db861 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db866 jmp 0xdbcfc
            pc = 0xdbcfc; continue;
        }
        0xdb86b => {
            // block db86b
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db86b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db86b cmp QWORD PTR [r15+0x290],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x290)) as u64, 0x0u64).1;
            // db873 jne 0xdb886
            if fl.ne() { pc = 0xdb886; continue; }
            pc = 0xdb875;
        }
        0xdb875 => {
            // block db875
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db875 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db875 mov edi,0x800000
            rdi = (0x800000u64) as u32 as u64;
            // db87a call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db87f mov QWORD PTR [r15+0x290],rax
            m.w64(r15.wrapping_add(0x290), (rax) as u64);
            pc = 0xdb886;
        }
        0xdb886 => {
            // block db886
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db886 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db886 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db888 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // db88b div DWORD PTR [r15+0x338]
            { let (q, r) = div::<32>(rdx, rax, m.r32(r15.wrapping_add(0x338)) as u64); rax = q; rdx = r; }
            // db892 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // db895 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // db898 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db89a mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // db89d div DWORD PTR [r15+0x33c]
            { let (q, r) = div::<32>(rdx, rax, m.r32(r15.wrapping_add(0x33c)) as u64); rax = q; rdx = r; }
            // db8a4 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // db8a6 jne 0xdbe91
            if fl.ne() { pc = 0xdbe91; continue; }
            pc = 0xdb8ac;
        }
        0xdb8ac => {
            // block db8ac
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db8ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db8ac lea rsi,[rbp-0xb8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // db8b3 mov rdi,r13
            rdi = r13;
            // db8b6 call 0xec2a2
            { let r = { let t = m.env.stub_va(0xa6f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db8bb test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // db8bd jne 0xdbd38
            if fl.ne() { pc = 0xdbd38; continue; }
            pc = 0xdb8c3;
        }
        0xdb8c3 => {
            // block db8c3
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db8c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db8c3 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // db8c7 test rax,rax
            fl = logic::<64>(rax & rax);
            // db8ca js 0xdbd38
            if fl.s() { pc = 0xdbd38; continue; }
            pc = 0xdb8d0;
        }
        0xdb8d0 => {
            // block db8d0
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db8d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db8d0 cmp rax,0x800001
            fl = sub::<64>(rax, 0x800001u64).1;
            // db8d6 jge 0xdbea4
            if fl.ge() { pc = 0xdbea4; continue; }
            pc = 0xdb8dc;
        }
        0xdb8dc => {
            // block db8dc
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db8dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db8dc lea rsi,[rip+0x4e996] # 0x12a279
            rsi = m.env.data_va(256582);
            // db8e3 mov rdi,r13
            rdi = r13;
            // db8e6 call 0xec1c4
            { let r = { let t = m.env.stub_va(0x991); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db8eb mov r13,rax
            r13 = rax;
            // db8ee test r13,r13
            fl = logic::<64>(r13 & r13);
            // db8f1 je 0xdbeba
            if fl.e() { pc = 0xdbeba; continue; }
            pc = 0xdb8f7;
        }
        0xdb8f7 => {
            // block db8f7
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db8f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db8f7 mov rdi,QWORD PTR [r15+0x290]
            rdi = m.r64(r15.wrapping_add(0x290)) as u64;
            // db8fe mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // db903 mov edx,0x800000
            rdx = (0x800000u64) as u32 as u64;
            // db908 mov rcx,r13
            rcx = r13;
            // db90b call 0xec1ca
            { let r = { let t = m.env.stub_va(0x997); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db910 mov rbx,rax
            rbx = rax;
            // db913 mov rdi,r13
            rdi = r13;
            // db916 call 0xec1b8
            { let r = { let t = m.env.stub_va(0x985); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db91b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // db91e je 0xdbed0
            if fl.e() { pc = 0xdbed0; continue; }
            pc = 0xdb924;
        }
        0xdb924 => {
            // block db924
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db924 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db924 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // db927 mov rdi,r15
            rdi = r15;
            // db92a call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db930 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db932 je 0xdbe1a
            if fl.e() { pc = 0xdbe1a; continue; }
            pc = 0xdb938;
        }
        0xdb938 => {
            // block db938
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db938 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db938 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // db93b lea rcx,[rip+0x2761e] # 0x102f60
            rcx = m.env.data_va(96045);
            // db942 mov esi,DWORD PTR [rcx+rax*4]
            rsi = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // db945 mov rdi,r15
            rdi = r15;
            // db948 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db94d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // db950 mov r12d,0xffffffff
            r12 = (0xffffffffu64) as u32 as u64;
            // db956 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // db95b mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // db960 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db965 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db968 mov rdi,r15
            rdi = r15;
            // db96b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db96e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // db971 mov rdi,r15
            rdi = r15;
            // db974 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db97a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // db97d mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // db982 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db984 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db989 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db98c mov rdi,r15
            rdi = r15;
            // db98f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db992 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // db995 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // db99a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // db99f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db9a4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db9a7 mov rdi,r15
            rdi = r15;
            // db9aa call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db9ad mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // db9b0 mov rdi,r15
            rdi = r15;
            // db9b3 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db9b9 mov esi,DWORD PTR [r15+0x33c]
            rsi = (m.r32(r15.wrapping_add(0x33c)) as u64) as u32 as u64;
            // db9c0 lea rdi,[rbx+rsi*1-0x1]
            rdi = rbx.wrapping_add(rsi).wrapping_add(0xffff_ffff_ffff_ffff);
            // db9c5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db9c7 mov rax,rdi
            rax = rdi;
            // db9ca div rsi
            { let (q, r) = div::<64>(rdx, rax, rsi); rax = q; rdx = r; }
            // db9cd mov r13,rax
            r13 = rax;
            // db9d0 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // db9d3 mov DWORD PTR [rbp-0x17c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe84), ((r14 as u32 as u64)) as u32);
            // db9da jle 0xdba2e
            if fl.le() { pc = 0xdba2e; continue; }
            pc = 0xdb9dc;
        }
        0xdb9dc => {
            // block db9dc
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db9dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db9dc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db9de mov rax,rdi
            rax = rdi;
            // db9e1 div rsi
            { let (q, r) = div::<64>(rdx, rax, rsi); rax = q; rdx = r; }
            // db9e4 mov r14,rax
            r14 = rax;
            // db9e7 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            pc = 0xdb9ea;
        }
        0xdb9ea => {
            // block db9ea
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc db9ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db9ea inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // db9ed xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db9ef lea rdi,[rip+0x5d5ed] # 0x138fe3
            rdi = m.env.data_va(317360);
            // db9f6 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // db9f9 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // db9fc call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dba01 mov rax,QWORD PTR [rip+0x649d8] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // dba08 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // dba0b call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dba10 mov esi,DWORD PTR [r15+0x33c]
            rsi = (m.r32(r15.wrapping_add(0x33c)) as u64) as u32 as u64;
            // dba17 imul esi,r12d
            { let (r, f) = imul::<32>((rsi as u32 as u64), (r12 as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // dba1b add esi,DWORD PTR [rbp-0x17c]
            { let (r, f) = add::<32>((rsi as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dba21 mov rdi,r15
            rdi = r15;
            // dba24 call 0xd81de
            { let r = { let t = m.env.code_va(0xd81de); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dba29 cmp r12d,r14d
            fl = sub::<32>((r12 as u32 as u64), (r14 as u32 as u64)).1;
            // dba2c jne 0xdb9ea
            if fl.ne() { pc = 0xdb9ea; continue; }
            pc = 0xdba2e;
        }
        0xdba2e => {
            // block dba2e
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dba2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dba2e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dba31 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // dba36 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // dba3b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dba40 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dba43 mov rdi,r15
            rdi = r15;
            // dba46 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dba49 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dba4c mov rdi,r15
            rdi = r15;
            // dba4f call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dba55 lea r14,[rbx+0x1ff]
            r14 = rbx.wrapping_add(0x1ff);
            // dba5c shr r14,0x9
            { let (r, f) = shr::<64>(r14, 0x9u64, fl); r14 = r; fl = f; }
            // dba60 mov QWORD PTR [rbp-0x188],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (r14) as u64);
            // dba67 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dba6a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // dba6f mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // dba74 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dba79 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dba7c mov rdi,r15
            rdi = r15;
            // dba7f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dba82 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dba85 mov rdi,r15
            rdi = r15;
            // dba88 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dba8e test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // dba91 je 0xdbc47
            if fl.e() { pc = 0xdbc47; continue; }
            pc = 0xdba97;
        }
        0xdba97 => {
            // block dba97
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dba97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dba97 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // dba9a xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xdba9d;
        }
        0xdba9d => {
            // block dba9d
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dba9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dba9d mov r14d,0x80
            r14 = (0x80u64) as u32 as u64;
            pc = 0xdbaa3;
        }
        0xdbaa3 => {
            // block dbaa3
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbaa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbaa3 cmp ebx,0x4
            fl = sub::<32>((rbx as u32 as u64), 0x4u64).1;
            // dbaa6 jl 0xdbadd
            if fl.l() { pc = 0xdbadd; continue; }
            pc = 0xdbaa8;
        }
        0xdbaa8 => {
            // block dbaa8
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbaa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbaa8 movsxd rax,r12d
            rax = sx::<32>((r12 as u32 as u64)) as u64;
            // dbaab mov rcx,QWORD PTR [r15+0x290]
            rcx = m.r64(r15.wrapping_add(0x290)) as u64;
            // dbab2 movzx edx,BYTE PTR [rcx+rax*1]
            rdx = (m.r8(rcx.wrapping_add(rax)) as u64) as u32 as u64;
            // dbab6 shl edx,0x18
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x18u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // dbab9 movzx esi,BYTE PTR [rcx+rax*1+0x1]
            rsi = (m.r8(rcx.wrapping_add(rax).wrapping_add(0x1)) as u64) as u32 as u64;
            // dbabe shl esi,0x10
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // dbac1 or esi,edx
            { let r = (rsi as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dbac3 movzx edi,BYTE PTR [rcx+rax*1+0x2]
            rdi = (m.r8(rcx.wrapping_add(rax).wrapping_add(0x2)) as u64) as u32 as u64;
            // dbac8 shl edi,0x8
            { let (r, f) = shl::<32>((rdi as u32 as u64), 0x8u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // dbacb or edi,esi
            { let r = (rdi as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // dbacd movzx edx,BYTE PTR [rcx+rax*1+0x3]
            rdx = (m.r8(rcx.wrapping_add(rax).wrapping_add(0x3)) as u64) as u32 as u64;
            // dbad2 or edx,edi
            { let r = (rdx as u32 as u64) | (rdi as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbad4 add r12d,0x4
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x4u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // dbad8 add ebx,0xfffffffc
            { let (r, f) = add::<32>((rbx as u32 as u64), 0xfffffffcu64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dbadb jmp 0xdbb5a
            pc = 0xdbb5a; continue;
        }
        0xdbadd => {
            // block dbadd
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbadd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbadd cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // dbae0 je 0xdbb0a
            if fl.e() { pc = 0xdbb0a; continue; }
            pc = 0xdbae2;
        }
        0xdbae2 => {
            // block dbae2
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbae2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbae2 cmp ebx,0x2
            fl = sub::<32>((rbx as u32 as u64), 0x2u64).1;
            // dbae5 jne 0xdbb23
            if fl.ne() { pc = 0xdbb23; continue; }
            pc = 0xdbae7;
        }
        0xdbae7 => {
            // block dbae7
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbae7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbae7 movsxd rax,r12d
            rax = sx::<32>((r12 as u32 as u64)) as u64;
            // dbaea mov rcx,QWORD PTR [r15+0x290]
            rcx = m.r64(r15.wrapping_add(0x290)) as u64;
            // dbaf1 movzx esi,BYTE PTR [rcx+rax*1]
            rsi = (m.r8(rcx.wrapping_add(rax)) as u64) as u32 as u64;
            // dbaf5 shl esi,0x18
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x18u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // dbaf8 movzx edx,BYTE PTR [rcx+rax*1+0x1]
            rdx = (m.r8(rcx.wrapping_add(rax).wrapping_add(0x1)) as u64) as u32 as u64;
            // dbafd shl edx,0x10
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // dbb00 or edx,esi
            { let r = (rdx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbb02 or edx,0xffff
            { let r = (rdx as u32 as u64) | 0xffffu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbb08 jmp 0xdbb58
            pc = 0xdbb58; continue;
        }
        0xdbb0a => {
            // block dbb0a
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbb0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbb0a movsxd rax,r12d
            rax = sx::<32>((r12 as u32 as u64)) as u64;
            // dbb0d mov rcx,QWORD PTR [r15+0x290]
            rcx = m.r64(r15.wrapping_add(0x290)) as u64;
            // dbb14 movzx edx,BYTE PTR [rcx+rax*1]
            rdx = (m.r8(rcx.wrapping_add(rax)) as u64) as u32 as u64;
            // dbb18 shl edx,0x18
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x18u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // dbb1b or edx,0xffffff
            { let r = (rdx as u32 as u64) | 0xffffffu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbb21 jmp 0xdbb58
            pc = 0xdbb58; continue;
        }
        0xdbb23 => {
            // block dbb23
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbb23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbb23 mov edx,0xffffffff
            rdx = (0xffffffffu64) as u32 as u64;
            // dbb28 cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // dbb2b jne 0xdbb58
            if fl.ne() { pc = 0xdbb58; continue; }
            pc = 0xdbb2d;
        }
        0xdbb2d => {
            // block dbb2d
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbb2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbb2d movsxd rax,r12d
            rax = sx::<32>((r12 as u32 as u64)) as u64;
            // dbb30 mov rcx,QWORD PTR [r15+0x290]
            rcx = m.r64(r15.wrapping_add(0x290)) as u64;
            // dbb37 movzx esi,BYTE PTR [rcx+rax*1]
            rsi = (m.r8(rcx.wrapping_add(rax)) as u64) as u32 as u64;
            // dbb3b shl esi,0x18
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x18u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // dbb3e movzx edx,BYTE PTR [rcx+rax*1+0x1]
            rdx = (m.r8(rcx.wrapping_add(rax).wrapping_add(0x1)) as u64) as u32 as u64;
            // dbb43 shl edx,0x10
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // dbb46 movzx eax,BYTE PTR [rcx+rax*1+0x2]
            rax = (m.r8(rcx.wrapping_add(rax).wrapping_add(0x2)) as u64) as u32 as u64;
            // dbb4b shl eax,0x8
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // dbb4e or edx,esi
            { let r = (rdx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbb50 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbb52 or edx,0xff
            { let r = (rdx as u32 as u64) | 0xffu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xdbb58;
        }
        0xdbb58 => {
            // block dbb58
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbb58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbb58 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xdbb5a;
        }
        0xdbb5a => {
            // block dbb5a
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbb5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbb5a bswap edx
            rdx = (rdx as u32).swap_bytes() as u64;
            // dbb5c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbb5f mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // dbb64 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbb69 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbb6c mov rdi,r15
            rdi = r15;
            // dbb6f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbb72 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // dbb75 jne 0xdbaa3
            if fl.ne() { pc = 0xdbaa3; continue; }
            pc = 0xdbb7b;
        }
        0xdbb7b => {
            // block dbb7b
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbb7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbb7b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbb7e mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // dbb83 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbb88 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbb8b mov rdi,r15
            rdi = r15;
            // dbb8e mov r14d,DWORD PTR [rbp-0x17c]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // dbb95 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // dbb98 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbb9b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbb9e mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // dbba3 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // dbba8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbbad xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbbb0 mov rdi,r15
            rdi = r15;
            // dbbb3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbbb6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbbb9 mov rdi,r15
            rdi = r15;
            // dbbbc call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbbc2 cmp BYTE PTR [r15+0x378],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x378)) as u64, 0x0u64).1;
            // dbbca jne 0xdbbff
            if fl.ne() { pc = 0xdbbff; continue; }
            pc = 0xdbbcc;
        }
        0xdbbcc => {
            // block dbbcc
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbbcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbbcc imul eax,r13d,0x64
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x64u64); rax = (r) as u32 as u64; fl = f; }
            // dbbd0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbbd2 mov rcx,QWORD PTR [rbp-0x188]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // dbbd9 div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // dbbdb mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // dbbdd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dbbdf lea rdi,[rip+0x5d41b] # 0x139001
            rdi = m.env.data_va(317390);
            // dbbe6 mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // dbbe8 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // dbbeb call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbbf0 mov rax,QWORD PTR [rip+0x647e9] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // dbbf7 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // dbbfa call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdbbff;
        }
        0xdbbff => {
            // block dbbff
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbbff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbbff add r14d,0x200
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x200u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // dbc06 mov DWORD PTR [rbp-0x17c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe84), ((r14 as u32 as u64)) as u32);
            // dbc0d inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // dbc10 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbc13 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // dbc18 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // dbc1d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbc22 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbc25 mov rdi,r15
            rdi = r15;
            // dbc28 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbc2b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbc2e mov rdi,r15
            rdi = r15;
            // dbc31 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbc37 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // dbc3e cmp r13d,eax
            fl = sub::<32>((r13 as u32 as u64), (rax as u32 as u64)).1;
            // dbc41 jne 0xdba9d
            if fl.ne() { pc = 0xdba9d; continue; }
            pc = 0xdbc47;
        }
        0xdbc47 => {
            // block dbc47
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbc47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbc47 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbc4a mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // dbc4f mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // dbc54 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbc59 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbc5c mov rdi,r15
            rdi = r15;
            // dbc5f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbc62 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbc65 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // dbc6a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // dbc6f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbc74 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbc77 mov rdi,r15
            rdi = r15;
            // dbc7a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbc7d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbc80 mov rdi,r15
            rdi = r15;
            // dbc83 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbc89 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbc8c mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // dbc91 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // dbc96 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbc9b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbc9e mov rdi,r15
            rdi = r15;
            // dbca1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbca4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbca7 mov rdi,r15
            rdi = r15;
            // dbcaa call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbcb0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbcb3 mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // dbcb8 mov edx,0x9c
            rdx = (0x9cu64) as u32 as u64;
            // dbcbd mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbcc2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbcc5 mov rdi,r15
            rdi = r15;
            // dbcc8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbccb mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbcce mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // dbcd3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // dbcd8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbcdd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbce0 mov rdi,r15
            rdi = r15;
            // dbce3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbce6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbce9 mov rdi,r15
            rdi = r15;
            // dbcec call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbcf2 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dbcf4 mov rdi,r15
            rdi = r15;
            // dbcf7 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdbcfc;
        }
        0xdbcfc => {
            // block dbcfc
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbcfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbcfc add rsp,0x168
            { let (r, f) = add::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // dbd03 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dbd04 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dbd06 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dbd08 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dbd0a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dbd0c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dbd0d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdbd0e => {
            // block dbd0e
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbd0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbd0e mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbd13 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbd18 lea rcx,[rip+0x5d1fe] # 0x138f1d
            rcx = m.env.data_va(317162);
            // dbd1f mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dbd22 mov rsi,QWORD PTR [rip+0x644af] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // dbd29 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbd2b mov rdi,rax
            rdi = rax;
            // dbd2e call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
        0xdbd38 => {
            // block dbd38
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbd38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbd38 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbd3d call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbd42 lea rcx,[rip+0x5d1d4] # 0x138f1d
            rcx = m.env.data_va(317162);
            // dbd49 jmp 0xdbe2b
            pc = 0xdbe2b; continue;
        }
        0xdbd4e => {
            // block dbd4e
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbd4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbd4e mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbd53 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbd58 lea rcx,[rip+0x5d1d6] # 0x138f35
            rcx = m.env.data_va(317186);
            // dbd5f mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dbd62 mov rsi,QWORD PTR [rip+0x6446f] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // dbd69 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbd6b mov rdi,rax
            rdi = rax;
            // dbd6e call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
        0xdbd78 => {
            // block dbd78
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbd78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbd78 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbd7d call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbd82 lea rcx,[rip+0x5d1db] # 0x138f64
            rcx = m.env.data_va(317233);
            // dbd89 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dbd8c mov rsi,QWORD PTR [rip+0x64445] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // dbd93 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbd95 mov rdi,rax
            rdi = rax;
            // dbd98 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
        0xdbd9f => {
            // block dbd9f
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbd9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbd9f mov rdi,r12
            rdi = r12;
            // dbda2 call 0xec1b8
            { let r = { let t = m.env.stub_va(0x985); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbda7 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbdac call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbdb1 lea rcx,[rip+0x5d1c0] # 0x138f78
            rcx = m.env.data_va(317253);
            // dbdb8 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dbdbb mov rsi,QWORD PTR [rip+0x64416] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // dbdc2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbdc4 mov rdi,rax
            rdi = rax;
            // dbdc7 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
        0xdbdce => {
            // block dbdce
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbdce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbdce mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbdd3 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbdd8 lea rcx,[rip+0x5d341] # 0x139120
            rcx = m.env.data_va(317677);
            // dbddf mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dbde2 mov rsi,QWORD PTR [rip+0x643ef] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // dbde9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbdeb mov rdi,rax
            rdi = rax;
            // dbdee call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
        0xdbdf5 => {
            // block dbdf5
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbdf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbdf5 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbdfa call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbdff lea rcx,[rip+0x5d19a] # 0x138fa0
            rcx = m.env.data_va(317293);
            // dbe06 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dbe09 mov rsi,QWORD PTR [rip+0x643c8] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // dbe10 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbe12 mov rdi,rax
            rdi = rax;
            // dbe15 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
        0xdbe1a => {
            // block dbe1a
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe1a mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbe1f call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbe24 lea rcx,[rip+0x5d2f5] # 0x139120
            rcx = m.env.data_va(317677);
            pc = 0xdbe2b;
        }
        0xdbe2b => {
            // block dbe2b
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe2b mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dbe2e mov rsi,QWORD PTR [rip+0x643a3] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // dbe35 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbe37 mov rdi,rax
            rdi = rax;
            // dbe3a call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
        0xdbe42 => {
            // block dbe42
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe42 mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // dbe49 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // dbe4c je 0xdbe63
            if fl.e() { pc = 0xdbe63; continue; }
            pc = 0xdbe4e;
        }
        0xdbe4e => {
            // block dbe4e
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe4e cmp QWORD PTR [rbp-0x158],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64, rdi).1;
            // dbe55 je 0xdbe5e
            if fl.e() { pc = 0xdbe5e; continue; }
            pc = 0xdbe57;
        }
        0xdbe57 => {
            // block dbe57
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe57 mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            pc = 0xdbe5e;
        }
        0xdbe5e => {
            // block dbe5e
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe5e call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdbe63;
        }
        0xdbe63 => {
            // block dbe63
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe63 mov rdi,rbx
            rdi = rbx;
            // dbe66 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbe6b mov rbx,rax
            rbx = rax;
            // dbe6e mov rdi,QWORD PTR [rbp-0x178]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // dbe75 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // dbe78 je 0xdbe42
            if fl.e() { pc = 0xdbe42; continue; }
            pc = 0xdbe7a;
        }
        0xdbe7a => {
            // block dbe7a
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe7a cmp QWORD PTR [rbp-0x170],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64, rdi).1;
            // dbe81 je 0xdbe8a
            if fl.e() { pc = 0xdbe8a; continue; }
            pc = 0xdbe83;
        }
        0xdbe83 => {
            // block dbe83
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe83 mov QWORD PTR [rbp-0x170],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rdi) as u64);
            pc = 0xdbe8a;
        }
        0xdbe8a => {
            // block dbe8a
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe8a call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbe8f jmp 0xdbe42
            pc = 0xdbe42; continue;
        }
        0xdbe91 => {
            // block dbe91
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbe91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe91 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbe96 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbe9b lea rcx,[rip+0x5d114] # 0x138fb6
            rcx = m.env.data_va(317315);
            // dbea2 jmp 0xdbe2b
            pc = 0xdbe2b; continue;
        }
        0xdbea4 => {
            // block dbea4
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbea4 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbea9 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbeae lea rcx,[rip+0x5d080] # 0x138f35
            rcx = m.env.data_va(317186);
            // dbeb5 jmp 0xdbe2b
            pc = 0xdbe2b; continue;
        }
        0xdbeba => {
            // block dbeba
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbeba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbeba mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbebf call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbec4 lea rcx,[rip+0x5d099] # 0x138f64
            rcx = m.env.data_va(317233);
            // dbecb jmp 0xdbe2b
            pc = 0xdbe2b; continue;
        }
        0xdbed0 => {
            // block dbed0
            #[cfg(feature = "trace")] eprintln!("f_000db6b6 pc dbed0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbed0 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // dbed5 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbeda lea rcx,[rip+0x5d097] # 0x138f78
            rcx = m.env.data_va(317253);
            // dbee1 jmp 0xdbe2b
            pc = 0xdbe2b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x567ac size=2042
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000567ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x567ac;
    loop {
        match pc {
        0x567ac => {
            // block 567ac
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 567ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 567ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 567ad mov rbp,rsp
            rbp = rsp;
            // 567b0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 567b2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 567b4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 567b6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 567b8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 567b9 sub rsp,0x2c8
            { let (r, f) = sub::<64>(rsp, 0x2c8u64);
              rsp = r;
              fl = f; }
            // 567c0 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 567c7 mov edi,DWORD PTR [rdi+0x1d8]
            rdi = (m.r32(rdi.wrapping_add(0x1d8)) as u64) as u32 as u64;
            // 567cd lea rsi,[rbp-0x64]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 567d1 call 0xebf7e
            { let r = { let t = m.env.stub_va(0x74b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 567d6 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 567d9 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 567dc je 0x569bd
            if fl.e() { pc = 0x569bd; continue; }
            pc = 0x567e2;
        }
        0x567e2 => {
            // block 567e2
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 567e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 567e2 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 567e9 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 567f0 mov rax,QWORD PTR [rip+0xe99d9] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 567f7 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 567fb mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 56802 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 56806 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 5680d mov rsi,rbx
            rsi = rbx;
            // 56810 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56815 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 5681d mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 56824 mov r14,QWORD PTR [rip+0xe9b95] # 0x1403c0
            r14 = m.r64(m.env.data_va(347021)) as u64;
            // 5682b lea rax,[r14+0x18]
            rax = r14.wrapping_add(0x18);
            // 5682f mov QWORD PTR [rbp-0x2e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd20), (rax) as u64);
            // 56836 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 5683d add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // 56841 mov QWORD PTR [rbp-0x100],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r14) as u64);
            // 56848 mov rdi,rbx
            rdi = rbx;
            // 5684b call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56850 mov r12,QWORD PTR [rip+0xe9b51] # 0x1403a8
            r12 = m.r64(m.env.data_va(346997)) as u64;
            // 56857 add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 5685b mov QWORD PTR [rbp-0x168],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (r12) as u64);
            // 56862 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 5686d mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 56878 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 56883 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 5688e mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 56898 xorps xmm0,xmm0
            x0 = 0;
            // 5689b movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 5689f mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 568a7 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 568ab mov rdi,rbx
            rdi = rbx;
            // 568ae call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 568b3 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 568b7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 568bc call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 568c1 mov rbx,rax
            rbx = rax;
            // 568c4 lea rsi,[rip+0xd43f6] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 568cb lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 568d2 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 568d7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 568dc lea rsi,[rip+0xd43e2] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 568e3 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 568e8 mov rdi,rax
            rdi = rax;
            // 568eb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 568f0 lea rsi,[rip+0xd4089] # 0x12a980
            rsi = m.env.data_va(258381);
            // 568f7 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 568fc mov rdi,rax
            rdi = rax;
            // 568ff call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56904 lea rsi,[rip+0xd3b6a] # 0x12a475
            rsi = m.env.data_va(257090);
            // 5690b mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 56910 mov rdi,rax
            rdi = rax;
            // 56913 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56918 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5691b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 5691f mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 56923 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56926 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56929 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 5692d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56930 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56934 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 5693d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56940 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56944 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 5694f mov rdi,rax
            rdi = rax;
            // 56952 mov rsi,rbx
            rsi = rbx;
            // 56955 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5695a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5695d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56961 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 56965 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56968 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5696b mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 5696f lea rsi,[rip+0xd400a] # 0x12a980
            rsi = m.env.data_va(258381);
            // 56976 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 5697b mov rdi,rax
            rdi = rax;
            // 5697e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56983 mov rbx,rax
            rbx = rax;
            // 56986 lea rsi,[rip+0xd4864] # 0x12b1f1
            rsi = m.env.data_va(260542);
            // 5698d lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 56994 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 56999 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5699e movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 569a5 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 569a8 jne 0x56bb8
            if fl.ne() { pc = 0x56bb8; continue; }
            pc = 0x569ae;
        }
        0x569ae => {
            // block 569ae
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 569ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 569ae lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 569b5 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 569b8 jmp 0x56bc6
            pc = 0x56bc6; continue;
        }
        0x569bd => {
            // block 569bd
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 569bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 569bd mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 569bf cmp DWORD PTR [rbp-0x64],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64, 0x0u64).1;
            // 569c3 je 0x56d70
            if fl.e() { pc = 0x56d70; continue; }
            pc = 0x569c9;
        }
        0x569c9 => {
            // block 569c9
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 569c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 569c9 test BYTE PTR [rip+0x1718c7],0x40 # 0x1c8297
            fl = logic::<8>(m.r8(m.env.data_va(903780)) as u64 & 0x40u64);
            // 569d0 je 0x56d70
            if fl.e() { pc = 0x56d70; continue; }
            pc = 0x569d6;
        }
        0x569d6 => {
            // block 569d6
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 569d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 569d6 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 569dd lea r14,[rbp-0x2a0]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // 569e4 mov rax,QWORD PTR [rip+0xe97e5] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 569eb lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 569ef mov QWORD PTR [rbp-0x2a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (rcx) as u64);
            // 569f6 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 569fa mov QWORD PTR [rbp-0x238],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rax) as u64);
            // 56a01 mov rsi,r14
            rsi = r14;
            // 56a04 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56a09 mov QWORD PTR [rbp-0x1b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (0x0u64) as u64);
            // 56a14 mov DWORD PTR [rbp-0x1a8],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (0xffffffffu64) as u32);
            // 56a1e mov r12,QWORD PTR [rip+0xe999b] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 56a25 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 56a2a mov QWORD PTR [rbp-0x2e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd20), (rax) as u64);
            // 56a31 mov QWORD PTR [rbp-0x2a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (rax) as u64);
            // 56a38 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 56a3c mov QWORD PTR [rbp-0x238],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (r12) as u64);
            // 56a43 mov rdi,r14
            rdi = r14;
            // 56a46 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56a4b mov r15,QWORD PTR [rip+0xe9956] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // 56a52 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // 56a56 mov QWORD PTR [rbp-0x2a0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (r15) as u64);
            // 56a5d mov QWORD PTR [rbp-0x248],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb8), (0x0u64) as u64);
            // 56a68 mov QWORD PTR [rbp-0x250],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0), (0x0u64) as u64);
            // 56a73 mov QWORD PTR [rbp-0x258],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (0x0u64) as u64);
            // 56a7e mov QWORD PTR [rbp-0x260],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (0x0u64) as u64);
            // 56a89 mov DWORD PTR [rbp-0x240],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fdc0), (0x10u64) as u32);
            // 56a93 xorps xmm0,xmm0
            x0 = 0;
            // 56a96 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 56a9a mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 56aa2 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 56aa6 mov rdi,r14
            rdi = r14;
            // 56aa9 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56aae lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 56ab2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56ab7 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56abc mov r13,rax
            r13 = rax;
            // 56abf lea rsi,[rip+0xd41fb] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 56ac6 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 56acd mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 56ad2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56ad7 lea rsi,[rip+0xd4558] # 0x12b036
            rsi = m.env.data_va(260099);
            // 56ade mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 56ae3 mov rdi,rax
            rdi = rax;
            // 56ae6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56aeb lea rsi,[rip+0xd3e8e] # 0x12a980
            rsi = m.env.data_va(258381);
            // 56af2 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 56af7 mov rdi,rax
            rdi = rax;
            // 56afa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56aff lea rsi,[rip+0xd396f] # 0x12a475
            rsi = m.env.data_va(257090);
            // 56b06 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 56b0b mov rdi,rax
            rdi = rax;
            // 56b0e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56b13 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56b16 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56b1a mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 56b1e and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56b21 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56b24 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 56b28 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56b2b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56b2f mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 56b38 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56b3b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56b3f mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 56b4a mov rdi,rax
            rdi = rax;
            // 56b4d mov rsi,r13
            rsi = r13;
            // 56b50 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56b55 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56b58 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56b5c mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 56b60 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56b63 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56b66 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 56b6a lea rsi,[rip+0xd3e0f] # 0x12a980
            rsi = m.env.data_va(258381);
            // 56b71 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 56b76 mov rdi,rax
            rdi = rax;
            // 56b79 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56b7e mov r13,rax
            r13 = rax;
            // 56b81 lea rsi,[rip+0xd4669] # 0x12b1f1
            rsi = m.env.data_va(260542);
            // 56b88 lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 56b8f mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 56b94 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56b99 movzx edx,BYTE PTR [rbp-0x2c0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64) as u32 as u64;
            // 56ba0 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 56ba3 jne 0x56d84
            if fl.ne() { pc = 0x56d84; continue; }
            pc = 0x56ba9;
        }
        0x56ba9 => {
            // block 56ba9
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56ba9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56ba9 lea rsi,[rbp-0x2bf]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd41);
            // 56bb0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 56bb3 jmp 0x56d92
            pc = 0x56d92; continue;
        }
        0x56bb8 => {
            // block 56bb8
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56bb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56bb8 mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 56bbf mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x56bc6;
        }
        0x56bc6 => {
            // block 56bc6
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56bc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56bc6 mov rdi,rbx
            rdi = rbx;
            // 56bc9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56bce lea rsi,[rip+0xd3dab] # 0x12a980
            rsi = m.env.data_va(258381);
            // 56bd5 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 56bda mov rdi,rax
            rdi = rax;
            // 56bdd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56be2 lea rsi,[rip+0xd461b] # 0x12b204
            rsi = m.env.data_va(260561);
            // 56be9 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 56bee mov rdi,rax
            rdi = rax;
            // 56bf1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56bf6 lea rsi,[rip+0xd4173] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 56bfd mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 56c02 mov rdi,rax
            rdi = rax;
            // 56c05 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56c0a lea rsi,[rip+0xd3864] # 0x12a475
            rsi = m.env.data_va(257090);
            // 56c11 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 56c16 mov rdi,rax
            rdi = rax;
            // 56c19 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56c1e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56c21 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56c25 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 56c29 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56c2c or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56c2f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 56c33 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56c36 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56c3a mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 56c43 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56c46 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56c4a mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 56c55 mov rdi,rax
            rdi = rax;
            // 56c58 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 56c5b call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56c60 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 56c63 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 56c67 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 56c6b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56c6e or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 56c71 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 56c75 lea rsi,[rip+0xd3d0a] # 0x12a986
            rsi = m.env.data_va(258387);
            // 56c7c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 56c81 mov rdi,rax
            rdi = rax;
            // 56c84 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56c89 mov r15,rax
            r15 = rax;
            // 56c8c mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 56c8f call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56c94 mov rbx,rax
            rbx = rax;
            // 56c97 mov rdi,rbx
            rdi = rbx;
            // 56c9a call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56c9f mov rdi,r15
            rdi = r15;
            // 56ca2 mov rsi,rbx
            rsi = rbx;
            // 56ca5 mov rdx,rax
            rdx = rax;
            // 56ca8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56cad lea rsi,[rip+0xd39ce] # 0x12a682
            rsi = m.env.data_va(257615);
            // 56cb4 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 56cb9 mov rdi,rax
            rdi = rax;
            // 56cbc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56cc1 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 56cc8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56ccd lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 56cd4 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 56cdb mov rsi,rbx
            rsi = rbx;
            // 56cde call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56ce3 test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 56cea jne 0x56cf5
            if fl.ne() { pc = 0x56cf5; continue; }
            pc = 0x56cec;
        }
        0x56cec => {
            // block 56cec
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56cec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56cec lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 56cf3 jmp 0x56cfc
            pc = 0x56cfc; continue;
        }
        0x56cf5 => {
            // block 56cf5
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56cf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56cf5 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x56cfc;
        }
        0x56cfc => {
            // block 56cfc
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56cfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56cfc lea rsi,[rip+0xd4016] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 56d03 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 56d08 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 56d0a call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56d0f lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 56d16 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56d1b mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 56d22 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 56d29 mov QWORD PTR [rbp-0x100],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r14) as u64);
            // 56d30 mov QWORD PTR [rbp-0x168],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (r12) as u64);
            // 56d37 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 56d3e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56d43 mov rdi,rbx
            rdi = rbx;
            // 56d46 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56d4b mov rsi,QWORD PTR [rip+0xe94b6] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 56d52 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 56d56 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 56d5d call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56d62 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 56d69 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56d6e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x56d70;
        }
        0x56d70 => {
            // block 56d70
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56d70 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 56d72 add rsp,0x2c8
            { let (r, f) = add::<64>(rsp, 0x2c8u64);
              rsp = r;
              fl = f; }
            // 56d79 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 56d7a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 56d7c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 56d7e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 56d80 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 56d82 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 56d83 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x56d84 => {
            // block 56d84
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56d84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56d84 mov rdx,QWORD PTR [rbp-0x2b8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64;
            // 56d8b mov rsi,QWORD PTR [rbp-0x2b0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            pc = 0x56d92;
        }
        0x56d92 => {
            // block 56d92
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56d92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56d92 mov rdi,r13
            rdi = r13;
            // 56d95 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56d9a lea rsi,[rip+0xd3bdf] # 0x12a980
            rsi = m.env.data_va(258381);
            // 56da1 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 56da6 mov rdi,rax
            rdi = rax;
            // 56da9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56dae lea rsi,[rip+0xd446c] # 0x12b221
            rsi = m.env.data_va(260590);
            // 56db5 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 56dba mov rdi,rax
            rdi = rax;
            // 56dbd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56dc2 lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 56dc9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56dce lea rdi,[rbp-0x2d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd28);
            // 56dd5 mov rsi,r14
            rsi = r14;
            // 56dd8 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56ddd test BYTE PTR [rbp-0x2d8],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64 & 0x1u64);
            // 56de4 jne 0x56def
            if fl.ne() { pc = 0x56def; continue; }
            pc = 0x56de6;
        }
        0x56de6 => {
            // block 56de6
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56de6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56de6 lea rdx,[rbp-0x2d7]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fd29);
            // 56ded jmp 0x56df6
            pc = 0x56df6; continue;
        }
        0x56def => {
            // block 56def
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56def mov rdx,QWORD PTR [rbp-0x2c8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            pc = 0x56df6;
        }
        0x56df6 => {
            // block 56df6
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56df6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56df6 lea rsi,[rip+0xd3f1c] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 56dfd mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 56e02 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 56e04 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56e09 lea rdi,[rbp-0x2d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd28);
            // 56e10 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56e15 mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 56e1c mov QWORD PTR [rbp-0x2a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (rax) as u64);
            // 56e23 mov QWORD PTR [rbp-0x238],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (r12) as u64);
            // 56e2a mov QWORD PTR [rbp-0x2a0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (r15) as u64);
            // 56e31 lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 56e38 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56e3d mov rdi,r14
            rdi = r14;
            // 56e40 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56e45 mov rsi,QWORD PTR [rip+0xe93bc] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 56e4c add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 56e50 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 56e57 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56e5c lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 56e63 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56e68 jmp 0x56d70
            pc = 0x56d70; continue;
        }
        0x56e6d => {
            // block 56e6d
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56e6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56e6d mov r13,rax
            r13 = rax;
            // 56e70 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x56e77;
        }
        0x56e77 => {
            // block 56e77
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56e77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56e77 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56e7c jmp 0x56e81
            pc = 0x56e81; continue;
        }
        0x56e7e => {
            // block 56e7e
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56e7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56e7e mov r13,rax
            r13 = rax;
            pc = 0x56e81;
        }
        0x56e81 => {
            // block 56e81
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56e81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56e81 mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 56e88 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 56e8f mov QWORD PTR [rbp-0x100],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r14) as u64);
            // 56e96 mov QWORD PTR [rbp-0x168],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (r12) as u64);
            // 56e9d lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 56ea4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56ea9 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 56eb0 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x56eb5;
        }
        0x56eb5 => {
            // block 56eb5
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56eb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56eb5 mov rsi,QWORD PTR [rip+0xe934c] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 56ebc add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 56ec0 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 56ec7 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x56ecc;
        }
        0x56ecc => {
            // block 56ecc
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56ecc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56ecc lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            pc = 0x56ed3;
        }
        0x56ed3 => {
            // block 56ed3
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56ed3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56ed3 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56ed8 mov rdi,r13
            rdi = r13;
            // 56edb call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56ee0 mov r13,rax
            r13 = rax;
            // 56ee3 jmp 0x56f53
            pc = 0x56f53; continue;
        }
        0x56ee5 => {
            // block 56ee5
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56ee5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56ee5 mov r13,rax
            r13 = rax;
            // 56ee8 jmp 0x56ecc
            pc = 0x56ecc; continue;
        }
        0x56eea => {
            // block 56eea
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56eea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56eea mov r13,rax
            r13 = rax;
            // 56eed jmp 0x56eb5
            pc = 0x56eb5; continue;
        }
        0x56eef => {
            // block 56eef
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56eef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56eef mov r13,rax
            r13 = rax;
            // 56ef2 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 56ef6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56efb lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 56f02 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56f07 mov rdi,rbx
            rdi = rbx;
            // 56f0a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56f0f jmp 0x56eb5
            pc = 0x56eb5; continue;
        }
        0x56f11 => {
            // block 56f11
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f11 mov r13,rax
            r13 = rax;
            // 56f14 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 56f1b jmp 0x56e77
            pc = 0x56e77; continue;
        }
        0x56f20 => {
            // block 56f20
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f20 mov r13,rax
            r13 = rax;
            // 56f23 lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 56f2a jmp 0x56f4e
            pc = 0x56f4e; continue;
        }
        0x56f2c => {
            // block 56f2c
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f2c mov r13,rax
            r13 = rax;
            // 56f2f jmp 0x56f9a
            pc = 0x56f9a; continue;
        }
        0x56f31 => {
            // block 56f31
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f31 mov r13,rax
            r13 = rax;
            // 56f34 jmp 0x56f83
            pc = 0x56f83; continue;
        }
        0x56f36 => {
            // block 56f36
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f36 mov r13,rax
            r13 = rax;
            // 56f39 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 56f3d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56f42 jmp 0x56f6f
            pc = 0x56f6f; continue;
        }
        0x56f44 => {
            // block 56f44
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f44 mov r13,rax
            r13 = rax;
            // 56f47 lea rdi,[rbp-0x2d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd28);
            pc = 0x56f4e;
        }
        0x56f4e => {
            // block 56f4e
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f4e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x56f53;
        }
        0x56f53 => {
            // block 56f53
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f53 mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 56f5a mov QWORD PTR [rbp-0x2a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (rax) as u64);
            // 56f61 mov QWORD PTR [rbp-0x238],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (r12) as u64);
            // 56f68 mov QWORD PTR [rbp-0x2a0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (r15) as u64);
            pc = 0x56f6f;
        }
        0x56f6f => {
            // block 56f6f
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f6f lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 56f76 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56f7b mov rdi,r14
            rdi = r14;
            // 56f7e call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x56f83;
        }
        0x56f83 => {
            // block 56f83
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f83 mov rsi,QWORD PTR [rip+0xe927e] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 56f8a add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 56f8e lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 56f95 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x56f9a;
        }
        0x56f9a => {
            // block 56f9a
            #[cfg(feature = "trace")] eprintln!("f_000567ac pc 56f9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f9a lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 56fa1 jmp 0x56ed3
            pc = 0x56ed3; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4e0d2 size=2000
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004e0d2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4e0d2;
    loop {
        match pc {
        0x4e0d2 => {
            // block 4e0d2
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e0d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e0d2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4e0d3 mov rbp,rsp
            rbp = rsp;
            // 4e0d6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4e0d8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4e0da push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4e0dc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4e0de push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4e0df sub rsp,0x1c8
            { let (r, f) = sub::<64>(rsp, 0x1c8u64);
              rsp = r;
              fl = f; }
            // 4e0e6 mov DWORD PTR [rbp-0x1c8],r9d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe38), ((r9 as u32 as u64)) as u32);
            // 4e0ed mov DWORD PTR [rbp-0x1c4],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe3c), ((rcx as u32 as u64)) as u32);
            // 4e0f3 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 4e0f6 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 4e0f9 mov r13,rdi
            r13 = rdi;
            // 4e0fc mov edx,DWORD PTR [rbp+0x10]
            rdx = (m.r32(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 4e0ff mov rbx,QWORD PTR [rip+0xf22d2] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4e106 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4e109 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 4e10d mov DWORD PTR [rbp-0x84],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), (0x0u64) as u32);
            // 4e117 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 4e11a mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 4e11e mov QWORD PTR [rbp-0x58],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r8) as u64);
            // 4e122 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 4e124 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 4e128 mov eax,r9d
            rax = ((r9 as u32 as u64)) as u32 as u64;
            // 4e12b mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 4e12f mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 4e133 movzx eax,r12b
            rax = ((r12 as u8 as u64)) as u32 as u64;
            // 4e137 xor rax,0x1
            { let r = rax ^ 0x1u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 4e13b mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 4e13f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 4e143 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4e145 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e14b mov r14d,0x5
            r14 = (0x5u64) as u32 as u64;
            // 4e151 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4e153 je 0x4e191
            if fl.e() { pc = 0x4e191; continue; }
            pc = 0x4e155;
        }
        0x4e155 => {
            // block 4e155
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e155 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e155 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 4e159 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4e15b mov rdi,r13
            rdi = r13;
            // 4e15e call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e164 lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4e168 lea r9,[rbp-0x84]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff7c);
            // 4e16f mov esi,0xf
            rsi = (0xfu64) as u32 as u64;
            // 4e174 mov ecx,0x6
            rcx = (0x6u64) as u32 as u64;
            // 4e179 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4e17c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4e17e call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e183 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 4e186 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4e188 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // 4e18b je 0x4e7d9
            if fl.e() { pc = 0x4e7d9; continue; }
            pc = 0x4e191;
        }
        0x4e191 => {
            // block 4e191
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e191 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e191 test BYTE PTR [rip+0x17a0f9],0x80 # 0x1c8291
            fl = logic::<8>(m.r8(m.env.data_va(903774)) as u64 & 0x80u64);
            // 4e198 je 0x4e7d7
            if fl.e() { pc = 0x4e7d7; continue; }
            pc = 0x4e19e;
        }
        0x4e19e => {
            // block 4e19e
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e19e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e19e lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 4e1a5 lea rbx,[rbp-0x188]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4e1ac mov rax,QWORD PTR [rip+0xf201d] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4e1b3 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4e1b7 mov QWORD PTR [rbp-0x190],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rcx) as u64);
            // 4e1be add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4e1c2 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 4e1c9 mov rsi,rbx
            rsi = rbx;
            // 4e1cc call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e1d1 mov QWORD PTR [rbp-0x98],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u64);
            // 4e1dc mov DWORD PTR [rbp-0x90],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0xffffffffu64) as u32);
            // 4e1e6 mov rax,QWORD PTR [rip+0xf21d3] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 4e1ed lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4e1f1 mov QWORD PTR [rbp-0x1d8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (rcx) as u64);
            // 4e1f8 mov QWORD PTR [rbp-0x190],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rcx) as u64);
            // 4e1ff add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4e203 mov QWORD PTR [rbp-0x1d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (rax) as u64);
            // 4e20a mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 4e211 mov rdi,rbx
            rdi = rbx;
            // 4e214 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e219 mov DWORD PTR [rbp-0x1e4],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe1c), ((r12 as u32 as u64)) as u32);
            // 4e220 mov rax,QWORD PTR [rip+0xf2181] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4e227 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4e22b mov QWORD PTR [rbp-0x1e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe20), (rax) as u64);
            // 4e232 mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 4e239 mov QWORD PTR [rbp-0x130],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x0u64) as u64);
            // 4e244 mov QWORD PTR [rbp-0x138],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (0x0u64) as u64);
            // 4e24f mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // 4e25a mov QWORD PTR [rbp-0x148],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (0x0u64) as u64);
            // 4e265 mov DWORD PTR [rbp-0x128],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x10u64) as u32);
            // 4e26f xorps xmm0,xmm0
            x0 = 0;
            // 4e272 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // 4e276 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 4e27e lea rsi,[rbp-0x80]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 4e282 mov rdi,rbx
            rdi = rbx;
            // 4e285 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e28a lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 4e28e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e293 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e298 mov r12,rax
            r12 = rax;
            // 4e29b lea rsi,[rip+0xdca1f] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4e2a2 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 4e2a9 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4e2ae call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e2b3 lea rsi,[rip+0xdca0b] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4e2ba mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4e2bf mov rdi,rax
            rdi = rax;
            // 4e2c2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e2c7 lea rsi,[rip+0xdc6b2] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4e2ce mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4e2d3 mov rdi,rax
            rdi = rax;
            // 4e2d6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e2db lea rsi,[rip+0xdc193] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4e2e2 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4e2e7 mov rdi,rax
            rdi = rax;
            // 4e2ea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e2ef mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e2f2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e2f6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e2fa and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e2fd or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e300 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e304 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e307 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e30b mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4e314 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e317 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e31b mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4e326 mov rdi,rax
            rdi = rax;
            // 4e329 mov rsi,r12
            rsi = r12;
            // 4e32c call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e331 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e334 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e338 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e33c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e33f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e342 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e346 lea rsi,[rip+0xdc633] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4e34d mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4e352 mov rdi,rax
            rdi = rax;
            // 4e355 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e35a mov rbx,rax
            rbx = rax;
            // 4e35d lea rsi,[rip+0xdcbbf] # 0x12af23
            rsi = m.env.data_va(259824);
            // 4e364 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 4e36b mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4e370 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e375 movzx edx,BYTE PTR [rbp-0x1a8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64) as u32 as u64;
            // 4e37c test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4e37f jne 0x4e38d
            if fl.ne() { pc = 0x4e38d; continue; }
            pc = 0x4e381;
        }
        0x4e381 => {
            // block 4e381
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e381 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e381 lea rsi,[rbp-0x1a7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe59);
            // 4e388 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4e38b jmp 0x4e39b
            pc = 0x4e39b; continue;
        }
        0x4e38d => {
            // block 4e38d
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e38d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e38d mov rdx,QWORD PTR [rbp-0x1a0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 4e394 mov rsi,QWORD PTR [rbp-0x198]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            pc = 0x4e39b;
        }
        0x4e39b => {
            // block 4e39b
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e39b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e39b mov rdi,rbx
            rdi = rbx;
            // 4e39e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e3a3 lea rsi,[rip+0xdc5d6] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4e3aa mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4e3af mov rdi,rax
            rdi = rax;
            // 4e3b2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e3b7 lea rsi,[rip+0xdc9b2] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4e3be mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4e3c3 mov rdi,rax
            rdi = rax;
            // 4e3c6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e3cb lea rsi,[rip+0xdc0a3] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4e3d2 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4e3d7 mov rdi,rax
            rdi = rax;
            // 4e3da call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e3df mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e3e2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e3e6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e3ea and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e3ed or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e3f0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e3f4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e3f7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e3fb mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4e404 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e407 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e40b mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4e416 mov rdi,rax
            rdi = rax;
            // 4e419 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 4e41c call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e421 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e424 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e428 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e42c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e42f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e432 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e436 lea rsi,[rip+0xdc549] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4e43d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4e442 mov rdi,rax
            rdi = rax;
            // 4e445 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e44a mov r12,rax
            r12 = rax;
            // 4e44d mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // 4e450 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e455 mov rbx,rax
            rbx = rax;
            // 4e458 mov rdi,rbx
            rdi = rbx;
            // 4e45b call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e460 mov rdi,r12
            rdi = r12;
            // 4e463 mov rsi,rbx
            rsi = rbx;
            // 4e466 mov rdx,rax
            rdx = rax;
            // 4e469 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e46e lea rsi,[rip+0xdc20d] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4e475 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4e47a mov rdi,rax
            rdi = rax;
            // 4e47d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e482 lea rsi,[rip+0xdc871] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4e489 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4e48e mov rdi,rax
            rdi = rax;
            // 4e491 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e496 lea rsi,[rip+0xdbfd8] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4e49d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4e4a2 mov rdi,rax
            rdi = rax;
            // 4e4a5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e4aa mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e4ad mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e4b1 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e4b5 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e4b8 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e4bb mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e4bf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e4c2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e4c6 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4e4cf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e4d2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e4d6 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4e4e1 mov rdi,rax
            rdi = rax;
            // 4e4e4 mov rsi,r13
            rsi = r13;
            // 4e4e7 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e4ec mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e4ef mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e4f3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e4f7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e4fa or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e4fd mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e501 lea rsi,[rip+0xdc804] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4e508 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4e50d mov rdi,rax
            rdi = rax;
            // 4e510 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e515 lea rsi,[rip+0xdbf59] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4e51c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4e521 mov rdi,rax
            rdi = rax;
            // 4e524 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e529 mov rbx,rax
            rbx = rax;
            // 4e52c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4e52f mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e533 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e537 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4e53a or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4e53d mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4e541 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4e544 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e548 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4e551 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4e554 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e558 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4e563 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 4e567 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4e56e xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4e570 mov rdi,r13
            rdi = r13;
            // 4e573 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e575 mov rdi,rbx
            rdi = rbx;
            // 4e578 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4e57a call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e57f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e582 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e586 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e58a and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e58d or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e590 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e594 lea rsi,[rip+0xdc994] # 0x12af2f
            rsi = m.env.data_va(259836);
            // 4e59b mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4e5a0 mov rdi,rax
            rdi = rax;
            // 4e5a3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e5a8 mov rdi,rax
            rdi = rax;
            // 4e5ab mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 4e5ae call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e5b3 lea rsi,[rip+0xdc97c] # 0x12af36
            rsi = m.env.data_va(259843);
            // 4e5ba mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4e5bf mov rdi,rax
            rdi = rax;
            // 4e5c2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e5c7 mov rdi,rax
            rdi = rax;
            // 4e5ca mov esi,DWORD PTR [rbp-0x1c4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe3c)) as u64) as u32 as u64;
            // 4e5d0 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e5d5 lea rsi,[rip+0xdc961] # 0x12af3d
            rsi = m.env.data_va(259850);
            // 4e5dc mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4e5e1 mov rdi,rax
            rdi = rax;
            // 4e5e4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e5e9 lea rsi,[rip+0xdbe85] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4e5f0 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4e5f5 mov rdi,rax
            rdi = rax;
            // 4e5f8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e5fd mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e600 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e604 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e608 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e60b or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e60e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e612 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e615 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e619 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4e622 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e625 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e629 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4e634 mov rdi,rax
            rdi = rax;
            // 4e637 mov esi,DWORD PTR [rbp-0x1c8]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64) as u32 as u64;
            // 4e63d call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e642 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e645 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e649 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e64d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e650 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e653 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e657 lea rsi,[rip+0xdc8e6] # 0x12af44
            rsi = m.env.data_va(259857);
            // 4e65e mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4e663 mov rdi,rax
            rdi = rax;
            // 4e666 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e66b lea rsi,[rip+0xdbe03] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4e672 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4e677 mov rdi,rax
            rdi = rax;
            // 4e67a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e67f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e682 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e686 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e68a and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e68d or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e690 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e694 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e697 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e69b mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4e6a4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e6a7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e6ab mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4e6b6 mov rdi,rax
            rdi = rax;
            // 4e6b9 mov esi,DWORD PTR [rbp+0x10]
            rsi = (m.r32(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 4e6bc call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e6c1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4e6c4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4e6c8 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e6cc and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e6cf or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e6d2 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4e6d6 lea rsi,[rip+0xdbdc0] # 0x12a49d
            rsi = m.env.data_va(257130);
            // 4e6dd mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4e6e2 mov rdi,rax
            rdi = rax;
            // 4e6e5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e6ea lea rcx,[rip+0xdc2bc] # 0x12a9ad
            rcx = m.env.data_va(258426);
            // 4e6f1 lea rsi,[rip+0xdc2b3] # 0x12a9ab
            rsi = m.env.data_va(258424);
            // 4e6f8 mov edx,DWORD PTR [rbp-0x1e4]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe1c)) as u64) as u32 as u64;
            // 4e6fe test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 4e700 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // 4e704 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4e709 mov rdi,rax
            rdi = rax;
            // 4e70c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e711 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 4e718 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e71d lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 4e724 lea rsi,[rbp-0x188]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4e72b call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e730 test BYTE PTR [rbp-0x1c0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64 & 0x1u64);
            // 4e737 jne 0x4e742
            if fl.ne() { pc = 0x4e742; continue; }
            pc = 0x4e739;
        }
        0x4e739 => {
            // block 4e739
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e739 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e739 lea rdx,[rbp-0x1bf]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe41);
            // 4e740 jmp 0x4e749
            pc = 0x4e749; continue;
        }
        0x4e742 => {
            // block 4e742
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e742 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e742 mov rdx,QWORD PTR [rbp-0x1b0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            pc = 0x4e749;
        }
        0x4e749 => {
            // block 4e749
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e749 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e749 mov rbx,QWORD PTR [rip+0xf1c88] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4e750 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4e753 lea rsi,[rip+0xdc5bf] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4e75a mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4e75f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4e761 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e766 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 4e76d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e772 mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // 4e779 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // 4e780 mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // 4e787 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 4e78e mov rax,QWORD PTR [rbp-0x1e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // 4e795 mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 4e79c lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4e7a3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e7a8 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4e7af call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e7b4 mov rsi,QWORD PTR [rip+0xf1a4d] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4e7bb add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4e7bf lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 4e7c6 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e7cb lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 4e7d2 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4e7d7;
        }
        0x4e7d7 => {
            // block 4e7d7
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e7d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e7d7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4e7d9;
        }
        0x4e7d9 => {
            // block 4e7d9
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e7d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e7d9 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4e7dd jne 0x4e876
            if fl.ne() { pc = 0x4e876; continue; }
            pc = 0x4e7e3;
        }
        0x4e7e3 => {
            // block 4e7e3
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e7e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e7e3 add rsp,0x1c8
            { let (r, f) = add::<64>(rsp, 0x1c8u64);
              rsp = r;
              fl = f; }
            // 4e7ea pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4e7eb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4e7ed pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4e7ef pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4e7f1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4e7f3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4e7f4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4e7f5 => {
            // block 4e7f5
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e7f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e7f5 mov rbx,rax
            rbx = rax;
            // 4e7f8 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            pc = 0x4e7ff;
        }
        0x4e7ff => {
            // block 4e7ff
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e7ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e7ff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e804 jmp 0x4e809
            pc = 0x4e809; continue;
        }
        0x4e806 => {
            // block 4e806
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e806 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e806 mov rbx,rax
            rbx = rax;
            pc = 0x4e809;
        }
        0x4e809 => {
            // block 4e809
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e809 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e809 mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // 4e810 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // 4e817 mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // 4e81e mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 4e825 mov rax,QWORD PTR [rbp-0x1e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // 4e82c mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            pc = 0x4e833;
        }
        0x4e833 => {
            // block 4e833
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e833 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e833 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4e83a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e83f lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4e846 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4e84b;
        }
        0x4e84b => {
            // block 4e84b
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e84b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e84b mov rsi,QWORD PTR [rip+0xf19b6] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4e852 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4e856 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 4e85d call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4e862;
        }
        0x4e862 => {
            // block 4e862
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e862 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e862 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 4e869 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e86e mov rdi,rbx
            rdi = rbx;
            // 4e871 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4e876;
        }
        0x4e876 => {
            // block 4e876
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e876 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e876 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x4e880 => {
            // block 4e880
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e880 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e880 mov rbx,rax
            rbx = rax;
            // 4e883 jmp 0x4e84b
            pc = 0x4e84b; continue;
        }
        0x4e885 => {
            // block 4e885
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e885 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e885 mov rbx,rax
            rbx = rax;
            // 4e888 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 4e88c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e891 jmp 0x4e833
            pc = 0x4e833; continue;
        }
        0x4e893 => {
            // block 4e893
            #[cfg(feature = "trace")] eprintln!("f_0004e0d2 pc 4e893 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e893 mov rbx,rax
            rbx = rax;
            // 4e896 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 4e89d jmp 0x4e7ff
            pc = 0x4e7ff; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x46d3a size=1947
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00046d3a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x46d3a;
    loop {
        match pc {
        0x46d3a => {
            // block 46d3a
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46d3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46d3a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 46d3b mov rbp,rsp
            rbp = rsp;
            // 46d3e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 46d40 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 46d42 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 46d44 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 46d46 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 46d47 sub rsp,0x1a8
            { let (r, f) = sub::<64>(rsp, 0x1a8u64);
              rsp = r;
              fl = f; }
            // 46d4e mov DWORD PTR [rbp-0x1b4],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe4c), ((r8 as u32 as u64)) as u32);
            // 46d55 mov r13,rdx
            r13 = rdx;
            // 46d58 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 46d5b mov r14,rdi
            r14 = rdi;
            // 46d5e mov rax,QWORD PTR [rip+0xf9673] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // 46d65 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 46d68 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 46d6c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 46d6e test r13,r13
            fl = logic::<64>(r13 & r13);
            // 46d71 je 0x46d77
            if fl.e() { pc = 0x46d77; continue; }
            pc = 0x46d73;
        }
        0x46d73 => {
            // block 46d73
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46d73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46d73 mov ebx,DWORD PTR [r13+0x0]
            rbx = (m.r32(r13) as u64) as u32 as u64;
            pc = 0x46d77;
        }
        0x46d77 => {
            // block 46d77
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46d77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46d77 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 46d79 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 46d7d mov DWORD PTR [rbp-0x6c],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), (0x1u64) as u32);
            // 46d84 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 46d87 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 46d8b mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 46d8d mov DWORD PTR [rbp-0x1b0],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe50), ((rcx as u32 as u64)) as u32);
            // 46d93 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 46d97 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 46d9a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 46d9c mov rdi,r14
            rdi = r14;
            // 46d9f call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46da5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 46da7 je 0x46dea
            if fl.e() { pc = 0x46dea; continue; }
            pc = 0x46da9;
        }
        0x46da9 => {
            // block 46da9
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46da9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46da9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 46dac xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 46dae mov rdi,r14
            rdi = r14;
            // 46db1 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46db7 lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 46dbb lea r8,[rbp-0x68]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 46dbf lea r9,[rbp-0x6c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff94);
            // 46dc3 mov esi,0x0
            rsi = (0x0u64) as u32 as u64;
            // 46dc8 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 46dcd mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 46dcf call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46dd4 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 46dd7 mov eax,DWORD PTR [rbp-0x68]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 46dda mov DWORD PTR [r13+0x0],eax
            m.w32(r13, ((rax as u32 as u64)) as u32);
            // 46dde mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 46de0 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // 46de3 jne 0x46df4
            if fl.ne() { pc = 0x46df4; continue; }
            pc = 0x46de5;
        }
        0x46de5 => {
            // block 46de5
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46de5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46de5 jmp 0x47402
            pc = 0x47402; continue;
        }
        0x46dea => {
            // block 46dea
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46dea mov DWORD PTR [r13+0x0],ebx
            m.w32(r13, ((rbx as u32 as u64)) as u32);
            // 46dee mov r12d,0x5
            r12 = (0x5u64) as u32 as u64;
            pc = 0x46df4;
        }
        0x46df4 => {
            // block 46df4
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46df4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46df4 test BYTE PTR [rip+0x181495],0x1 # 0x1c8290
            fl = logic::<8>(m.r8(m.env.data_va(903773)) as u64 & 0x1u64);
            // 46dfb je 0x47400
            if fl.e() { pc = 0x47400; continue; }
            pc = 0x46e01;
        }
        0x46e01 => {
            // block 46e01
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46e01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46e01 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 46e08 lea rbx,[rbp-0x170]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 46e0f mov rax,QWORD PTR [rip+0xf93ba] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 46e16 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 46e1a mov QWORD PTR [rbp-0x178],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rcx) as u64);
            // 46e21 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 46e25 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 46e2c mov rsi,rbx
            rsi = rbx;
            // 46e2f call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46e34 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // 46e3c mov DWORD PTR [rbp-0x78],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0xffffffffu64) as u32);
            // 46e43 mov rax,QWORD PTR [rip+0xf9576] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 46e4a lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 46e4e mov QWORD PTR [rbp-0x1c8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (rcx) as u64);
            // 46e55 mov QWORD PTR [rbp-0x178],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rcx) as u64);
            // 46e5c add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 46e60 mov QWORD PTR [rbp-0x1c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe40), (rax) as u64);
            // 46e67 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 46e6e mov rdi,rbx
            rdi = rbx;
            // 46e71 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46e76 mov rax,QWORD PTR [rip+0xf952b] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 46e7d add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 46e81 mov QWORD PTR [rbp-0x1d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (rax) as u64);
            // 46e88 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 46e8f mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 46e9a mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 46ea5 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 46eb0 mov QWORD PTR [rbp-0x130],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x0u64) as u64);
            // 46ebb mov DWORD PTR [rbp-0x110],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x10u64) as u32);
            // 46ec5 xorps xmm0,xmm0
            x0 = 0;
            // 46ec8 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 46ecc mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 46ed4 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 46ed8 mov rdi,rbx
            rdi = rbx;
            // 46edb call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46ee0 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 46ee4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46ee9 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46eee mov r13,rax
            r13 = rax;
            // 46ef1 lea rsi,[rip+0xe3dc9] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 46ef8 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 46eff mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 46f04 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46f09 lea rsi,[rip+0xe3db5] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 46f10 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 46f15 mov rdi,rax
            rdi = rax;
            // 46f18 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46f1d lea rsi,[rip+0xe3a5c] # 0x12a980
            rsi = m.env.data_va(258381);
            // 46f24 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 46f29 mov rdi,rax
            rdi = rax;
            // 46f2c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46f31 lea rsi,[rip+0xe353d] # 0x12a475
            rsi = m.env.data_va(257090);
            // 46f38 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 46f3d mov rdi,rax
            rdi = rax;
            // 46f40 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46f45 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46f48 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46f4c mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 46f50 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 46f53 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 46f56 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 46f5a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46f5d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46f61 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 46f6a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46f6d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46f71 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 46f7c mov rdi,rax
            rdi = rax;
            // 46f7f mov rsi,r13
            rsi = r13;
            // 46f82 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46f87 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46f8a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46f8e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 46f92 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 46f95 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 46f98 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 46f9c lea rsi,[rip+0xe39dd] # 0x12a980
            rsi = m.env.data_va(258381);
            // 46fa3 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 46fa8 mov rdi,rax
            rdi = rax;
            // 46fab call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46fb0 mov rbx,rax
            rbx = rax;
            // 46fb3 lea rsi,[rip+0xe3da9] # 0x12ad63
            rsi = m.env.data_va(259376);
            // 46fba lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 46fc1 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 46fc6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46fcb movzx edx,BYTE PTR [rbp-0x190]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64) as u32 as u64;
            // 46fd2 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 46fd5 jne 0x46fe3
            if fl.ne() { pc = 0x46fe3; continue; }
            pc = 0x46fd7;
        }
        0x46fd7 => {
            // block 46fd7
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46fd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46fd7 lea rsi,[rbp-0x18f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe71);
            // 46fde shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 46fe1 jmp 0x46ff1
            pc = 0x46ff1; continue;
        }
        0x46fe3 => {
            // block 46fe3
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46fe3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46fe3 mov rdx,QWORD PTR [rbp-0x188]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 46fea mov rsi,QWORD PTR [rbp-0x180]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            pc = 0x46ff1;
        }
        0x46ff1 => {
            // block 46ff1
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 46ff1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46ff1 mov rdi,rbx
            rdi = rbx;
            // 46ff4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46ff9 lea rsi,[rip+0xe3980] # 0x12a980
            rsi = m.env.data_va(258381);
            // 47000 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 47005 mov rdi,rax
            rdi = rax;
            // 47008 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4700d lea rsi,[rip+0xe3d5c] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 47014 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 47019 mov rdi,rax
            rdi = rax;
            // 4701c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47021 lea rsi,[rip+0xe344d] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47028 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4702d mov rdi,rax
            rdi = rax;
            // 47030 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47035 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47038 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4703c mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47040 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47043 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47046 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4704a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4704d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47051 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4705a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4705d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47061 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4706c mov rdi,rax
            rdi = rax;
            // 4706f mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 47072 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47077 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4707a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4707e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47082 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47085 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47088 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4708c lea rsi,[rip+0xe38f3] # 0x12a986
            rsi = m.env.data_va(258387);
            // 47093 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 47098 mov rdi,rax
            rdi = rax;
            // 4709b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 470a0 mov r13,rax
            r13 = rax;
            // 470a3 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 470a6 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 470ab mov rbx,rax
            rbx = rax;
            // 470ae mov rdi,rbx
            rdi = rbx;
            // 470b1 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 470b6 mov rdi,r13
            rdi = r13;
            // 470b9 mov rsi,rbx
            rsi = rbx;
            // 470bc mov rdx,rax
            rdx = rax;
            // 470bf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 470c4 lea rsi,[rip+0xe35b7] # 0x12a682
            rsi = m.env.data_va(257615);
            // 470cb mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 470d0 mov rdi,rax
            rdi = rax;
            // 470d3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 470d8 lea rsi,[rip+0xe3c1b] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 470df mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 470e4 mov rdi,rax
            rdi = rax;
            // 470e7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 470ec lea rsi,[rip+0xe3382] # 0x12a475
            rsi = m.env.data_va(257090);
            // 470f3 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 470f8 mov rdi,rax
            rdi = rax;
            // 470fb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47100 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47103 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47107 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4710b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4710e or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47111 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47115 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47118 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4711c mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 47125 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47128 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4712c mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 47137 mov rdi,rax
            rdi = rax;
            // 4713a mov rsi,r14
            rsi = r14;
            // 4713d call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47142 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47145 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47149 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4714d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47150 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47153 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47157 lea rsi,[rip+0xe3ba7] # 0x12ad05
            rsi = m.env.data_va(259282);
            // 4715e mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 47163 mov rdi,rax
            rdi = rax;
            // 47166 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4716b movzx esi,WORD PTR [r14+0x8]
            rsi = (m.r16(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // 47170 mov rdi,rax
            rdi = rax;
            // 47173 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47178 lea rsi,[rip+0xe3b8d] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4717f mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 47184 mov rdi,rax
            rdi = rax;
            // 47187 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4718c lea rsi,[rip+0xe32e2] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47193 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 47198 mov rdi,rax
            rdi = rax;
            // 4719b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 471a0 mov rbx,rax
            rbx = rax;
            // 471a3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 471a6 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 471aa mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 471ae and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 471b1 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 471b4 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 471b8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 471bb mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 471bf mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 471c8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 471cb mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 471cf mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 471da mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 471dd mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 471e4 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 471e6 mov rdi,r14
            rdi = r14;
            // 471e9 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 471eb mov rdi,rbx
            rdi = rbx;
            // 471ee mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 471f0 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 471f5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 471f8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 471fc mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47200 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47203 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47206 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4720a lea rsi,[rip+0xe3b6b] # 0x12ad7c
            rsi = m.env.data_va(259401);
            // 47211 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 47216 mov rdi,rax
            rdi = rax;
            // 47219 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4721e mov rdi,rax
            rdi = rax;
            // 47221 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 47224 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47229 lea rsi,[rip+0xe3b55] # 0x12ad85
            rsi = m.env.data_va(259410);
            // 47230 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 47235 mov rdi,rax
            rdi = rax;
            // 47238 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4723d lea rsi,[rip+0xe3231] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47244 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 47249 mov rdi,rax
            rdi = rax;
            // 4724c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47251 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47254 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47258 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4725c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4725f or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47262 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47266 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47269 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4726d mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 47276 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47279 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4727d mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 47288 mov rdi,rax
            rdi = rax;
            // 4728b mov esi,DWORD PTR [rbp-0x1b0]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64) as u32 as u64;
            // 47291 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47296 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47299 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4729d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 472a1 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 472a4 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 472a7 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 472ab lea rsi,[rip+0xe3adb] # 0x12ad8d
            rsi = m.env.data_va(259418);
            // 472b2 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 472b7 mov rdi,rax
            rdi = rax;
            // 472ba call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 472bf lea rsi,[rip+0xe31af] # 0x12a475
            rsi = m.env.data_va(257090);
            // 472c6 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 472cb mov rdi,rax
            rdi = rax;
            // 472ce call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 472d3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 472d6 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 472da mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 472de and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 472e1 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 472e4 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 472e8 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 472eb mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 472ef mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 472f8 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 472fb mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 472ff mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4730a mov rdi,rax
            rdi = rax;
            // 4730d mov esi,DWORD PTR [rbp-0x1b4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe4c)) as u64) as u32 as u64;
            // 47313 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47318 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4731b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4731f mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47323 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47326 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47329 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4732d lea rsi,[rip+0xe3a62] # 0x12ad96
            rsi = m.env.data_va(259427);
            // 47334 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 47339 mov rdi,rax
            rdi = rax;
            // 4733c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47341 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 47348 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4734d lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 47354 lea rsi,[rbp-0x170]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4735b call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47360 test BYTE PTR [rbp-0x1a8],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64 & 0x1u64);
            // 47367 jne 0x47372
            if fl.ne() { pc = 0x47372; continue; }
            pc = 0x47369;
        }
        0x47369 => {
            // block 47369
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47369 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47369 lea rdx,[rbp-0x1a7]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe59);
            // 47370 jmp 0x47379
            pc = 0x47379; continue;
        }
        0x47372 => {
            // block 47372
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47372 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47372 mov rdx,QWORD PTR [rbp-0x198]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            pc = 0x47379;
        }
        0x47379 => {
            // block 47379
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47379 lea rsi,[rip+0xe3999] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 47380 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 47385 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 47387 lea rbx,[rbp-0x170]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4738e call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47393 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 4739a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4739f mov rax,QWORD PTR [rbp-0x1c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // 473a6 mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // 473ad mov rax,QWORD PTR [rbp-0x1c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // 473b4 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 473bb mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // 473c2 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 473c9 lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 473d0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 473d5 mov rdi,rbx
            rdi = rbx;
            // 473d8 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 473dd mov rsi,QWORD PTR [rip+0xf8e24] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 473e4 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 473e8 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 473ef call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 473f4 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 473fb call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x47400;
        }
        0x47400 => {
            // block 47400
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47400 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47400 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x47402;
        }
        0x47402 => {
            // block 47402
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47402 mov rcx,QWORD PTR [rip+0xf8fcf] # 0x1403d8
            rcx = m.r64(m.env.data_va(347045)) as u64;
            // 47409 mov rcx,QWORD PTR [rcx]
            rcx = m.r64(rcx) as u64;
            // 4740c cmp rcx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 47410 jne 0x474a9
            if fl.ne() { pc = 0x474a9; continue; }
            pc = 0x47416;
        }
        0x47416 => {
            // block 47416
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47416 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47416 add rsp,0x1a8
            { let (r, f) = add::<64>(rsp, 0x1a8u64);
              rsp = r;
              fl = f; }
            // 4741d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4741e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 47420 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 47422 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 47424 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 47426 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 47427 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x47428 => {
            // block 47428
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47428 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47428 mov rbx,rax
            rbx = rax;
            // 4742b lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            pc = 0x47432;
        }
        0x47432 => {
            // block 47432
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47432 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47432 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47437 jmp 0x4743c
            pc = 0x4743c; continue;
        }
        0x47439 => {
            // block 47439
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47439 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47439 mov rbx,rax
            rbx = rax;
            pc = 0x4743c;
        }
        0x4743c => {
            // block 4743c
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 4743c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4743c mov rax,QWORD PTR [rbp-0x1c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // 47443 mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // 4744a mov rax,QWORD PTR [rbp-0x1c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // 47451 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 47458 mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // 4745f mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            pc = 0x47466;
        }
        0x47466 => {
            // block 47466
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47466 lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 4746d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47472 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 47479 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4747e;
        }
        0x4747e => {
            // block 4747e
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 4747e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4747e mov rsi,QWORD PTR [rip+0xf8d83] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 47485 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 47489 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 47490 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x47495;
        }
        0x47495 => {
            // block 47495
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 47495 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47495 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 4749c call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 474a1 mov rdi,rbx
            rdi = rbx;
            // 474a4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x474a9;
        }
        0x474a9 => {
            // block 474a9
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 474a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 474a9 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x474b3 => {
            // block 474b3
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 474b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 474b3 mov rbx,rax
            rbx = rax;
            // 474b6 jmp 0x4747e
            pc = 0x4747e; continue;
        }
        0x474b8 => {
            // block 474b8
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 474b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 474b8 mov rbx,rax
            rbx = rax;
            // 474bb lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 474bf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 474c4 jmp 0x47466
            pc = 0x47466; continue;
        }
        0x474c6 => {
            // block 474c6
            #[cfg(feature = "trace")] eprintln!("f_00046d3a pc 474c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 474c6 mov rbx,rax
            rbx = rax;
            // 474c9 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 474d0 jmp 0x47432
            pc = 0x47432; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2fa0 size=1738
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00002fa0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2fa0;
    loop {
        match pc {
        0x2fa0 => {
            // block 2fa0
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 2fa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2fa1 mov rbp,rsp
            rbp = rsp;
            // 2fa4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2fa6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2fa8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2faa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2fac push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2fad sub rsp,0x358
            { let (r, f) = sub::<64>(rsp, 0x358u64);
              rsp = r;
              fl = f; }
            // 2fb4 mov rbx,rsi
            rbx = rsi;
            // 2fb7 mov QWORD PTR [rbp-0x368],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc98), (rbx) as u64);
            // 2fbe mov r12,rdi
            r12 = rdi;
            // 2fc1 mov r15,QWORD PTR [rip+0x13d410] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 2fc8 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 2fcb mov QWORD PTR [rbp-0x30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r15) as u64);
            // 2fcf lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 2fd6 call 0x12b8c
            { let r = { let t = m.env.code_va(0x12b8c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fdb mov DWORD PTR [r12+0x1c],0x0
            m.w32(r12.wrapping_add(0x1c), (0x0u64) as u32);
            // 2fe4 mov rdi,rbx
            rdi = rbx;
            // 2fe7 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fec lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 2ff3 mov rsi,rbx
            rsi = rbx;
            // 2ff6 mov rdx,rax
            rdx = rax;
            // 2ff9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ffe lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3005 lea rsi,[rbp-0x340]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 300c call 0x12c80
            { let r = { let t = m.env.code_va(0x12c80); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3011 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 3014 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 301b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3020 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 3023 je 0x30fb
            if fl.e() { pc = 0x30fb; continue; }
            pc = 0x3029;
        }
        0x3029 => {
            // block 3029
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3029 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3029 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3030 call 0x136b6
            { let r = { let t = m.env.code_va(0x136b6); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3035 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 3038 lea r13d,[r14+0x200]
            r13 = (r14.wrapping_add(0x200)) as u32 as u64;
            // 303f mov rdi,r13
            rdi = r13;
            // 3042 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3047 mov rbx,rax
            rbx = rax;
            // 304a mov esi,0xff
            rsi = (0xffu64) as u32 as u64;
            // 304f mov rdi,rbx
            rdi = rbx;
            // 3052 mov rdx,r13
            rdx = r13;
            // 3055 call 0xec22a
            { let r = { let t = m.env.stub_va(0x9f7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 305a lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3061 mov rsi,rbx
            rsi = rbx;
            // 3064 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 3067 call 0x1378a
            { let r = { let t = m.env.code_va(0x1378a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 306c mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 306f lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 3076 lea rdi,[rbp-0x358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // 307d call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3082 cmp r13d,r14d
            fl = sub::<32>((r13 as u32 as u64), (r14 as u32 as u64)).1;
            // 3085 jne 0x3102
            if fl.ne() { pc = 0x3102; continue; }
            pc = 0x3087;
        }
        0x3087 => {
            // block 3087
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3087 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3087 mov QWORD PTR [rbp-0x378],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc88), (rbx) as u64);
            // 308e mov esi,DWORD PTR [r12+0x10]
            rsi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 3093 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 309a call 0x13b36
            { let r = { let t = m.env.code_va(0x13b36); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 309f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 30a1 je 0x311d
            if fl.e() { pc = 0x311d; continue; }
            pc = 0x30a3;
        }
        0x30a3 => {
            // block 30a3
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 30a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30a3 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 30aa call 0x12b96
            { let r = { let t = m.env.code_va(0x12b96); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 30af mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 30b4 call 0xcd8c1
            { let r = { let t = m.env.code_va(0xcd8c1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 30b9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 30bb je 0x3111
            if fl.e() { pc = 0x3111; continue; }
            pc = 0x30bd;
        }
        0x30bd => {
            // block 30bd
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 30bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30bd mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 30c2 call 0xcdaff
            { let r = { let t = m.env.code_va(0xcdaff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 30c7 lea ebx,[r14+0x100]
            rbx = (r14.wrapping_add(0x100)) as u32 as u64;
            // 30ce shr ebx,0x8
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x8u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 30d1 mov DWORD PTR [rbp-0x36c],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc94), ((rbx as u32 as u64)) as u32);
            // 30d7 add r14d,0x4
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x4u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 30db shr r14d,0x2
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 30df test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 30e1 je 0x3124
            if fl.e() { pc = 0x3124; continue; }
            pc = 0x30e3;
        }
        0x30e3 => {
            // block 30e3
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 30e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30e3 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 30e5 shl eax,0x8
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 30e8 lea r14d,[rax+r14*4+0x800000]
            r14 = (rax.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x800000)) as u32 as u64;
            // 30f0 mov r13d,0x20
            r13 = (0x20u64) as u32 as u64;
            // 30f6 jmp 0x3212
            pc = 0x3212; continue;
        }
        0x30fb => {
            // block 30fb
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 30fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30fb xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 30fd jmp 0x3603
            pc = 0x3603; continue;
        }
        0x3102 => {
            // block 3102
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3102 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3102 mov rdi,rbx
            rdi = rbx;
            // 3105 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 310a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 310c jmp 0x35f7
            pc = 0x35f7; continue;
        }
        0x3111 => {
            // block 3111
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3111 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3111 mov rdi,QWORD PTR [rbp-0x378]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc88)) as u64;
            // 3118 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x311d;
        }
        0x311d => {
            // block 311d
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 311d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 311d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 311f jmp 0x35ed
            pc = 0x35ed; continue;
        }
        0x3124 => {
            // block 3124
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3124 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3124 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 3129 call 0xcdb57
            { let r = { let t = m.env.code_va(0xcdb57); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 312e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3130 je 0x3194
            if fl.e() { pc = 0x3194; continue; }
            pc = 0x3132;
        }
        0x3132 => {
            // block 3132
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3132 mov r15d,ebx
            r15 = ((rbx as u32 as u64)) as u32 as u64;
            // 3135 shl r15d,0x8
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x8u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 3139 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 313d mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3141 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3146 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3148 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 314d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3150 mov rdi,r12
            rdi = r12;
            // 3153 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3155 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3159 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 315d mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3162 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 3167 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 316c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 316f mov rdi,r12
            rdi = r12;
            // 3172 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3174 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3178 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 317f mov rdi,r12
            rdi = r12;
            // 3182 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3184 lea r14d,[r15+r14*4+0x1000000]
            r14 = (r15.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x1000000)) as u32 as u64;
            // 318c mov r13d,0x40
            r13 = (0x40u64) as u32 as u64;
            // 3192 jmp 0x3212
            pc = 0x3212; continue;
        }
        0x3194 => {
            // block 3194
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3194 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 3199 call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 319e mov r15d,ebx
            r15 = ((rbx as u32 as u64)) as u32 as u64;
            // 31a1 shl r15d,0x8
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x8u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 31a5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31a7 je 0x3204
            if fl.e() { pc = 0x3204; continue; }
            pc = 0x31a9;
        }
        0x31a9 => {
            // block 31a9
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 31a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31a9 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 31ad mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31b1 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 31b6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 31b8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 31bd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 31c0 mov rdi,r12
            rdi = r12;
            // 31c3 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31c5 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 31c9 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31cd mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 31d2 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 31d7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 31dc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 31df mov rdi,r12
            rdi = r12;
            // 31e2 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31e4 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 31e8 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 31ef mov rdi,r12
            rdi = r12;
            // 31f2 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31f4 lea r14d,[r15+r14*4+0x2000000]
            r14 = (r15.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x2000000)) as u32 as u64;
            // 31fc mov r13d,0x80
            r13 = (0x80u64) as u32 as u64;
            // 3202 jmp 0x3212
            pc = 0x3212; continue;
        }
        0x3204 => {
            // block 3204
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3204 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3204 lea r14d,[r15+r14*4+0x400000]
            r14 = (r15.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x400000)) as u32 as u64;
            // 320c mov r13d,0x10
            r13 = (0x10u64) as u32 as u64;
            pc = 0x3212;
        }
        0x3212 => {
            // block 3212
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3212 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3216 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 321a mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // 321f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3224 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3227 mov rdi,r12
            rdi = r12;
            // 322a mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 322d call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 322f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3233 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3237 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 323c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 323e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3243 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3246 mov rdi,r12
            rdi = r12;
            // 3249 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 324b mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 324f mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3253 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 3258 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 325a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 325f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3262 mov rdi,r12
            rdi = r12;
            // 3265 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3267 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 326b mov rax,QWORD PTR [rax+0x9c0]
            rax = m.r64(rax.wrapping_add(0x9c0)) as u64;
            // 3272 mov edx,0x40000
            rdx = (0x40000u64) as u32 as u64;
            // 3277 mov rdi,r12
            rdi = r12;
            // 327a mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 327d call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 327f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3281 je 0x35d8
            if fl.e() { pc = 0x35d8; continue; }
            pc = 0x3287;
        }
        0x3287 => {
            // block 3287
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3287 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3287 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 328b mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 328f mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 3294 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 3299 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 329e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 32a1 mov rdi,r12
            rdi = r12;
            // 32a4 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32a6 mov DWORD PTR [rbp-0x37c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc84), ((r14 as u32 as u64)) as u32);
            // 32ad test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 32af mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 32b4 je 0x342b
            if fl.e() { pc = 0x342b; continue; }
            pc = 0x32ba;
        }
        0x32ba => {
            // block 32ba
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 32ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ba xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 32bc xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 32bf mov r14,QWORD PTR [rbp-0x378]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc88)) as u64;
            pc = 0x32c6;
        }
        0x32c6 => {
            // block 32c6
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 32c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c6 mov DWORD PTR [rbp-0x35c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fca4), ((rcx as u32 as u64)) as u32);
            // 32cc call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32d1 cmp r13d,0x1000000
            fl = sub::<32>((r13 as u32 as u64), 0x1000000u64).1;
            // 32d8 sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32db and cl,al
            { let r = (rcx as u8 as u64) & (rax as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32dd movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 32e0 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 32e3 jne 0x3336
            if fl.ne() { pc = 0x3336; continue; }
            pc = 0x32e5;
        }
        0x32e5 => {
            // block 32e5
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 32e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32e5 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 32e9 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 32ed mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 32f2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 32f7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 32fc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 32ff mov rdi,r12
            rdi = r12;
            // 3302 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3304 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3308 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 330c mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3311 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 3316 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 331b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 331e mov rdi,r12
            rdi = r12;
            // 3321 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3323 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3327 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 332e xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 3331 mov rdi,r12
            rdi = r12;
            // 3334 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x3336;
        }
        0x3336 => {
            // block 3336
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3336 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3336 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 333a mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 333e mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3343 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 3348 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 334d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3350 mov rdi,r12
            rdi = r12;
            // 3353 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3355 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3359 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 3360 mov rdi,r12
            rdi = r12;
            // 3363 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3365 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3369 mov rbx,QWORD PTR [rax+0x30]
            rbx = m.r64(rax.wrapping_add(0x30)) as u64;
            // 336d xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x3370;
        }
        0x3370 => {
            // block 3370
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3370 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3370 mov edx,DWORD PTR [r14]
            rdx = (m.r32(r14) as u64) as u32 as u64;
            // 3373 mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // 3378 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 337d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3380 mov rdi,r12
            rdi = r12;
            // 3383 call rbx
            { let r = { let t = rbx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3385 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 3389 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 338c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3390 mov rbx,QWORD PTR [rax+0x30]
            rbx = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3394 cmp r15d,0x40
            fl = sub::<32>((r15 as u32 as u64), 0x40u64).1;
            // 3398 jb 0x3370
            if fl.b() { pc = 0x3370; continue; }
            pc = 0x339a;
        }
        0x339a => {
            // block 339a
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 339a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 339a mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 339f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 33a4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 33a7 mov rdi,r12
            rdi = r12;
            // 33aa mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // 33ad call rbx
            { let r = { let t = rbx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33af mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 33b3 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 33b7 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 33bc mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 33c1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 33c6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 33c9 mov rdi,r12
            rdi = r12;
            // 33cc call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33ce mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 33d2 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 33d9 mov rdi,r12
            rdi = r12;
            // 33dc call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33de mov edx,DWORD PTR [r12+0x1c]
            rdx = (m.r32(r12.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 33e3 mov eax,0x100
            rax = (0x100u64) as u32 as u64;
            // 33e8 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33ea mov DWORD PTR [r12+0x1c],edx
            m.w32(r12.wrapping_add(0x1c), ((rdx as u32 as u64)) as u32);
            // 33ef mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 33f3 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 33f7 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 33fc mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3401 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3404 mov rdi,r12
            rdi = r12;
            // 3407 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3409 mov ecx,DWORD PTR [rbp-0x35c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fca4)) as u64) as u32 as u64;
            // 340f inc ecx
            { let (r, f) = inc::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 3411 add r13d,0x100
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x100u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 3418 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 341d mov eax,DWORD PTR [rbp-0x36c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fc94)) as u64) as u32 as u64;
            // 3423 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 3425 jb 0x32c6
            if fl.b() { pc = 0x32c6; continue; }
            pc = 0x342b;
        }
        0x342b => {
            // block 342b
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 342b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 342b call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3430 mov ebx,DWORD PTR [rbp-0x37c]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fc84)) as u64) as u32 as u64;
            // 3436 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3438 je 0x3485
            if fl.e() { pc = 0x3485; continue; }
            pc = 0x343a;
        }
        0x343a => {
            // block 343a
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 343a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 343a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 343e mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3442 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3447 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3449 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 344e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3451 mov rdi,r12
            rdi = r12;
            // 3454 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3456 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 345a mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 345e mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3463 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 3468 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 346d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3470 mov rdi,r12
            rdi = r12;
            // 3473 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3475 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3479 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 3480 mov rdi,r12
            rdi = r12;
            // 3483 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x3485;
        }
        0x3485 => {
            // block 3485
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3485 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3489 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 348d mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3492 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 3497 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 349c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 349f mov rdi,r12
            rdi = r12;
            // 34a2 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 34a4 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 34a8 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 34af mov rdi,r12
            rdi = r12;
            // 34b2 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 34b4 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 34b8 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 34bc mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // 34c1 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 34c6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 34cb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 34ce mov rdi,r12
            rdi = r12;
            // 34d1 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 34d3 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 34d7 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 34db mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 34e0 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 34e5 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 34ea xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 34ed mov rdi,r12
            rdi = r12;
            // 34f0 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 34f2 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 34f6 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 34fd mov rdi,r12
            rdi = r12;
            // 3500 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3502 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3506 mov rax,QWORD PTR [rax+0x9d0]
            rax = m.r64(rax.wrapping_add(0x9d0)) as u64;
            // 350d mov rdi,r12
            rdi = r12;
            // 3510 mov rsi,QWORD PTR [rbp-0x368]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc98)) as u64;
            // 3517 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3519 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 351c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3520 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3524 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3529 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 352e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3533 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3536 mov rdi,r12
            rdi = r12;
            // 3539 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 353b mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 353f mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 3546 mov rdi,r12
            rdi = r12;
            // 3549 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 354b mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 354f mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3553 mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // 3558 mov edx,0x9c
            rdx = (0x9cu64) as u32 as u64;
            // 355d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3562 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3565 mov rdi,r12
            rdi = r12;
            // 3568 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 356a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 356e mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3572 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3577 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 357c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3581 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3584 mov rdi,r12
            rdi = r12;
            // 3587 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3589 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 358d mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 3594 mov rdi,r12
            rdi = r12;
            // 3597 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3599 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 359d mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 35a1 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 35a6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 35ab xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 35ae mov rdi,r12
            rdi = r12;
            // 35b1 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 35b3 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 35b5 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 35b9 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 35bd mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 35c2 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 35c7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 35cc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 35cf mov rdi,r12
            rdi = r12;
            // 35d2 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 35d4 jmp 0x35db
            pc = 0x35db; continue;
        }
        0x35d6 => {
            // block 35d6
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 35d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 35d6 jmp 0x362b
            pc = 0x362b; continue;
        }
        0x35d8 => {
            // block 35d8
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 35d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 35d8 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x35db;
        }
        0x35db => {
            // block 35db
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 35db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 35db mov rdi,QWORD PTR [rbp-0x378]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc88)) as u64;
            // 35e2 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 35e7 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 35ea setne bl
            rbx = (rbx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x35ed;
        }
        0x35ed => {
            // block 35ed
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 35ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 35ed mov r15,QWORD PTR [rip+0x13cde4] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 35f4 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            pc = 0x35f7;
        }
        0x35f7 => {
            // block 35f7
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 35f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 35f7 lea rdi,[rbp-0x358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // 35fe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x3603;
        }
        0x3603 => {
            // block 3603
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3603 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3603 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 360a call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 360f cmp r15,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 3613 jne 0x3664
            if fl.ne() { pc = 0x3664; continue; }
            pc = 0x3615;
        }
        0x3615 => {
            // block 3615
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3615 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3615 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 3617 add rsp,0x358
            { let (r, f) = add::<64>(rsp, 0x358u64);
              rsp = r;
              fl = f; }
            // 361e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 361f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 3621 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 3623 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3625 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3627 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3628 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x3629 => {
            // block 3629
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3629 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3629 jmp 0x362b
            pc = 0x362b; continue;
        }
        0x362b => {
            // block 362b
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 362b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 362b mov rbx,rax
            rbx = rax;
            // 362e lea rdi,[rbp-0x358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // 3635 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x363a;
        }
        0x363a => {
            // block 363a
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 363a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 363a lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3641 call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3646 mov rdi,rbx
            rdi = rbx;
            // 3649 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 364e mov rbx,rax
            rbx = rax;
            // 3651 jmp 0x363a
            pc = 0x363a; continue;
        }
        0x3653 => {
            // block 3653
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3653 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3653 mov rbx,rax
            rbx = rax;
            // 3656 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 365d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3662 jmp 0x363a
            pc = 0x363a; continue;
        }
        0x3664 => {
            // block 3664
            #[cfg(feature = "trace")] eprintln!("f_00002fa0 pc 3664 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3664 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x13004 size=1688
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
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
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13104 mov QWORD PTR [rbp-0x178],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rbx) as u64);
            // 1310b lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1310f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9e5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7cf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7cf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7cf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 132a6 mov esi,0xd
            rsi = (0xdu64) as u32 as u64;
            // 132ab mov rdi,rax
            rdi = rax;
            // 132ae call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 132b3 lea rsi,[rip+0x116db0] # 0x12a06a
            rsi = m.env.data_va(256055);
            // 132ba mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 132bf mov rdi,rax
            rdi = rax;
            // 132c2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 132c7 mov BYTE PTR [rbp-0x2d],r15b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), ((r15 as u8 as u64)) as u8);
            // 132cb lea rsi,[rbp-0x2d]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd3);
            // 132cf mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 132d4 mov rdi,rax
            rdi = rax;
            // 132d7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 132dc lea rsi,[rip+0x116da5] # 0x12a088
            rsi = m.env.data_va(256085);
            // 132e3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 132e8 mov rdi,rax
            rdi = rax;
            // 132eb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1330d add r15d,0xf
            { let (r, f) = add::<32>((r15 as u32 as u64), 0xfu64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 13311 mov rdi,rax
            rdi = rax;
            // 13314 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 13317 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1331c lea rsi,[rip+0x116d67] # 0x12a08a
            rsi = m.env.data_va(256087);
            // 13323 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 13328 mov rdi,rax
            rdi = rax;
            // 1332b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13330 mov BYTE PTR [rbp-0x2c],bl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rbx as u8 as u64)) as u8);
            // 13333 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 13337 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1333c mov rdi,rax
            rdi = rax;
            // 1333f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13344 lea rsi,[rip+0x116d3d] # 0x12a088
            rsi = m.env.data_va(256085);
            // 1334b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 13350 mov rdi,rax
            rdi = rax;
            // 13353 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13375 mov rdi,rax
            rdi = rax;
            // 13378 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 1337b call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13380 lea rsi,[rip+0x116d21] # 0x12a0a8
            rsi = m.env.data_va(256117);
            // 13387 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 1338c mov rdi,rax
            rdi = rax;
            // 1338f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13394 mov BYTE PTR [rbp-0x2b],r15b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd5), ((r15 as u8 as u64)) as u8);
            // 13398 lea rsi,[rbp-0x2b]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd5);
            // 1339c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 133a1 mov rdi,rax
            rdi = rax;
            // 133a4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 133a9 lea rsi,[rip+0x116cd8] # 0x12a088
            rsi = m.env.data_va(256085);
            // 133b0 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 133b5 mov rdi,rax
            rdi = rax;
            // 133b8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 133da mov rdi,rax
            rdi = rax;
            // 133dd mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 133e0 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 133e5 lea rsi,[rip+0x116cda] # 0x12a0c6
            rsi = m.env.data_va(256147);
            // 133ec mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 133f1 mov rdi,rax
            rdi = rax;
            // 133f4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 133f9 mov BYTE PTR [rbp-0x2a],r15b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd6), ((r15 as u8 as u64)) as u8);
            // 133fd lea rsi,[rbp-0x2a]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd6);
            // 13401 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 13406 mov rdi,rax
            rdi = rax;
            // 13409 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1340e lea rsi,[rip+0x116c73] # 0x12a088
            rsi = m.env.data_va(256085);
            // 13415 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1341a mov rdi,rax
            rdi = rax;
            // 1341d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1343f mov rdi,rax
            rdi = rax;
            // 13442 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 13445 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1344a lea rsi,[rip+0x116c93] # 0x12a0e4
            rsi = m.env.data_va(256177);
            // 13451 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 13456 mov rdi,rax
            rdi = rax;
            // 13459 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1345e mov BYTE PTR [rbp-0x29],r15b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7), ((r15 as u8 as u64)) as u8);
            // 13462 lea rsi,[rbp-0x29]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // 13466 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1346b mov rdi,rax
            rdi = rax;
            // 1346e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13473 lea rsi,[rip+0x116c0e] # 0x12a088
            rsi = m.env.data_va(256085);
            // 1347a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1347f mov rdi,rax
            rdi = rax;
            // 13482 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 135bc lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 135c3 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 135c8 mov rsi,QWORD PTR [rip+0x12cc39] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 135cf add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 135d3 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 135da call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 135df lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 135e6 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1363d lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 13644 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13661 mov rdi,r14
            rdi = r14;
            // 13664 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13669 mov rdi,r12
            rdi = r12;
            // 1366c call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x13688;
        }
        0x13688 => {
            // block 13688
            #[cfg(feature = "trace")] eprintln!("f_00013004 pc 13688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13688 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 1368f call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13694 mov rdi,rbx
            rdi = rbx;
            // 13697 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2c3f7 size=1621
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002c3f7<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2c3f7;
    loop {
        match pc {
        0x2c3f7 => {
            // block 2c3f7
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c3f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c3f7 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2c3f8 mov rbp,rsp
            rbp = rsp;
            // 2c3fb push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2c3fd push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2c3ff push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2c401 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2c403 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2c404 sub rsp,0x2e8
            { let (r, f) = sub::<64>(rsp, 0x2e8u64);
              rsp = r;
              fl = f; }
            // 2c40b mov QWORD PTR [rbp-0x2e0],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd20), (rsi) as u64);
            // 2c412 mov QWORD PTR [rbp-0x2f0],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd10), (rdi) as u64);
            // 2c419 xorps xmm0,xmm0
            x0 = 0;
            // 2c41c movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // 2c420 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 2c428 mov r13,QWORD PTR [rsi]
            r13 = m.r64(rsi) as u64;
            // 2c42b mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 2c42f cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 2c432 je 0x2c8c2
            if fl.e() { pc = 0x2c8c2; continue; }
            pc = 0x2c438;
        }
        0x2c438 => {
            // block 2c438
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c438 lea rbx,[rbp-0x118]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 2c43f mov esi,0xffffffff
            rsi = (0xffffffffu64) as u32 as u64;
            // 2c444 mov rcx,QWORD PTR [rip+0x113f5d] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // 2c44b add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // 2c44f mov QWORD PTR [rbp-0x2c8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd38), (rcx) as u64);
            // 2c456 mov rcx,QWORD PTR [rip+0x113dab] # 0x140208
            rcx = m.r64(m.env.data_va(346581)) as u64;
            // 2c45d add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 2c461 mov QWORD PTR [rbp-0x2e8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd18), (rcx) as u64);
            // 2c468 mov r12d,0xffffffff
            r12 = (0xffffffffu64) as u32 as u64;
            // 2c46e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            pc = 0x2c473;
        }
        0x2c473 => {
            // block 2c473
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c473 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c473 mov r14d,DWORD PTR [r13+0x0]
            r14 = (m.r32(r13) as u64) as u32 as u64;
            // 2c477 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // 2c47a je 0x2c6a7
            if fl.e() { pc = 0x2c6a7; continue; }
            pc = 0x2c480;
        }
        0x2c480 => {
            // block 2c480
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c480 lea edx,[rcx+0x1]
            rdx = (rcx.wrapping_add(0x1)) as u32 as u64;
            // 2c483 cmp r14d,edx
            fl = sub::<32>((r14 as u32 as u64), (rdx as u32 as u64)).1;
            // 2c486 je 0x2c6aa
            if fl.e() { pc = 0x2c6aa; continue; }
            pc = 0x2c48c;
        }
        0x2c48c => {
            // block 2c48c
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c48c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c48c cmp r14d,ecx
            fl = sub::<32>((r14 as u32 as u64), (rcx as u32 as u64)).1;
            // 2c48f jne 0x2c499
            if fl.ne() { pc = 0x2c499; continue; }
            pc = 0x2c491;
        }
        0x2c491 => {
            // block 2c491
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c491 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c491 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 2c494 jmp 0x2c6ad
            pc = 0x2c6ad; continue;
        }
        0x2c499 => {
            // block 2c499
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c499 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2c49c mov rax,QWORD PTR [rip+0x113d2d] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 2c4a3 mov rcx,rax
            rcx = rax;
            // 2c4a6 lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 2c4aa mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 2c4b1 lea rax,[rcx+0x40]
            rax = rcx.wrapping_add(0x40);
            // 2c4b5 mov QWORD PTR [rbp-0x118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rax) as u64);
            // 2c4bc mov rdi,rbx
            rdi = rbx;
            // 2c4bf lea rbx,[rbp-0x180]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 2c4c6 mov rsi,rbx
            rsi = rbx;
            // 2c4c9 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c4ce mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 2c4d9 mov DWORD PTR [rbp-0x88],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0xffffffffu64) as u32);
            // 2c4e3 mov rax,QWORD PTR [rip+0x113ed6] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 2c4ea lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 2c4ee mov QWORD PTR [rbp-0x2d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd30), (rcx) as u64);
            // 2c4f5 mov QWORD PTR [rbp-0x188],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rcx) as u64);
            // 2c4fc lea rax,[rax+0x40]
            rax = rax.wrapping_add(0x40);
            // 2c500 mov QWORD PTR [rbp-0x2d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd28), (rax) as u64);
            // 2c507 mov QWORD PTR [rbp-0x118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rax) as u64);
            // 2c50e mov rdi,rbx
            rdi = rbx;
            // 2c511 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c516 mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c51d mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 2c524 lea rax,[rbp-0x140]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 2c52b mov QWORD PTR [rax+0x18],0x0
            m.w64(rax.wrapping_add(0x18), (0x0u64) as u64);
            // 2c533 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2c53b mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2c543 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 2c54a mov DWORD PTR [rbp-0x120],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x10u64) as u32);
            // 2c554 xorps xmm0,xmm0
            x0 = 0;
            // 2c557 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 2c55b mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 2c563 mov rdi,rbx
            rdi = rbx;
            // 2c566 lea rbx,[rbp-0x60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 2c56a mov rsi,rbx
            rsi = rbx;
            // 2c56d call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c572 mov rdi,rbx
            rdi = rbx;
            // 2c575 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c57a cmp r12d,r15d
            fl = sub::<32>((r12 as u32 as u64), (r15 as u32 as u64)).1;
            // 2c57d jne 0x2c597
            if fl.ne() { pc = 0x2c597; continue; }
            pc = 0x2c57f;
        }
        0x2c57f => {
            // block 2c57f
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c57f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c57f lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 2c586 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2c589 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c58e lea rbx,[rbp-0x1a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 2c595 jmp 0x2c5cc
            pc = 0x2c5cc; continue;
        }
        0x2c597 => {
            // block 2c597
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c597 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c597 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 2c59e mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 2c5a1 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c5a6 lea rbx,[rbp-0x1a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 2c5ad mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c5b2 mov rdi,rax
            rdi = rax;
            // 2c5b5 lea rsi,[rip+0xfe3c8] # 0x12a984
            rsi = m.env.data_va(258385);
            // 2c5bc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c5c1 mov rdi,rax
            rdi = rax;
            // 2c5c4 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2c5c7 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c5cc;
        }
        0x2c5cc => {
            // block 2c5cc
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c5cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c5cc mov rdi,rbx
            rdi = rbx;
            // 2c5cf lea rsi,[rbp-0x180]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 2c5d6 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c5db mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 2c5df cmp rax,QWORD PTR [rbp-0x70]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64).1;
            // 2c5e3 jae 0x2c621
            if fl.ae() { pc = 0x2c621; continue; }
            pc = 0x2c5e5;
        }
        0x2c5e5 => {
            // block 2c5e5
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c5e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c5e5 mov rcx,QWORD PTR [rbp-0x190]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 2c5ec mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 2c5f0 mov rcx,QWORD PTR [rbp-0x1a0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 2c5f7 mov rdx,QWORD PTR [rbp-0x198]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            // 2c5fe mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 2c602 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 2c605 xorps xmm0,xmm0
            x0 = 0;
            // 2c608 movaps XMMWORD PTR [rbp-0x1a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe60), x0);
            // 2c60f mov QWORD PTR [rbp-0x190],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (0x0u64) as u64);
            // 2c61a add QWORD PTR [rbp-0x78],0x18
            { let (r, f) = add::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64, 0x18u64);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (r) as u64);
              fl = f; }
            // 2c61f jmp 0x2c62d
            pc = 0x2c62d; continue;
        }
        0x2c621 => {
            // block 2c621
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c621 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c621 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 2c625 mov rsi,rbx
            rsi = rbx;
            // 2c628 call 0xebe8e
            { let r = { let t = m.env.stub_va(0x65b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c62d;
        }
        0x2c62d => {
            // block 2c62d
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c62d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c62d mov rdi,rbx
            rdi = rbx;
            // 2c630 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c635 mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 2c63c mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 2c643 mov rax,QWORD PTR [rbp-0x2d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64;
            // 2c64a mov QWORD PTR [rbp-0x118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rax) as u64);
            // 2c651 mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c658 mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 2c65f lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 2c666 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c66b lea r15,[rbp-0x180]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 2c672 mov rdi,r15
            rdi = r15;
            // 2c675 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c67a lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 2c681 mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            // 2c688 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c68d lea rbx,[rbp-0x118]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 2c694 mov rdi,rbx
            rdi = rbx;
            // 2c697 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c69c mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 2c6a3 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            pc = 0x2c6a7;
        }
        0x2c6a7 => {
            // block 2c6a7
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c6a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c6a7 mov r12d,r14d
            r12 = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0x2c6aa;
        }
        0x2c6aa => {
            // block 2c6aa
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c6aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c6aa mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0x2c6ad;
        }
        0x2c6ad => {
            // block 2c6ad
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c6ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c6ad add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 2c6b1 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 2c6b4 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 2c6b7 jne 0x2c473
            if fl.ne() { pc = 0x2c473; continue; }
            pc = 0x2c6bd;
        }
        0x2c6bd => {
            // block 2c6bd
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c6bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c6bd cmp r12d,0xffffffff
            fl = sub::<32>((r12 as u32 as u64), 0xffffffffu64).1;
            // 2c6c1 je 0x2c8c2
            if fl.e() { pc = 0x2c8c2; continue; }
            pc = 0x2c6c7;
        }
        0x2c6c7 => {
            // block 2c6c7
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c6c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c6c7 cmp esi,0xffffffff
            fl = sub::<32>((rsi as u32 as u64), 0xffffffffu64).1;
            // 2c6ca je 0x2c8c2
            if fl.e() { pc = 0x2c8c2; continue; }
            pc = 0x2c6d0;
        }
        0x2c6d0 => {
            // block 2c6d0
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c6d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c6d0 mov r15d,r12d
            r15 = ((r12 as u32 as u64)) as u32 as u64;
            // 2c6d3 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 2c6d5 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 2c6dc lea r13,[rbp-0x2a0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // 2c6e3 mov rax,QWORD PTR [rip+0x113ae6] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 2c6ea mov rcx,rax
            rcx = rax;
            // 2c6ed lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 2c6f1 mov QWORD PTR [rbp-0x2a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (rax) as u64);
            // 2c6f8 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 2c6fc mov QWORD PTR [rbp-0x238],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rcx) as u64);
            // 2c703 mov rsi,r13
            rsi = r13;
            // 2c706 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c70b mov QWORD PTR [rbp-0x1b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (0x0u64) as u64);
            // 2c716 mov DWORD PTR [rbp-0x1a8],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (0xffffffffu64) as u32);
            // 2c720 mov rax,QWORD PTR [rip+0x113c99] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 2c727 lea r14,[rax+0x18]
            r14 = rax.wrapping_add(0x18);
            // 2c72b mov QWORD PTR [rbp-0x2a8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (r14) as u64);
            // 2c732 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 2c736 mov QWORD PTR [rbp-0x2d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd30), (rax) as u64);
            // 2c73d mov QWORD PTR [rbp-0x238],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rax) as u64);
            // 2c744 mov rdi,r13
            rdi = r13;
            // 2c747 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c74c mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c753 mov QWORD PTR [rbp-0x2a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (rax) as u64);
            // 2c75a lea r12,[rbp-0x260]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 2c761 mov QWORD PTR [rbp-0x248],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb8), (0x0u64) as u64);
            // 2c76c mov QWORD PTR [rbp-0x250],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0), (0x0u64) as u64);
            // 2c777 mov QWORD PTR [rbp-0x258],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (0x0u64) as u64);
            // 2c782 mov QWORD PTR [rbp-0x260],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (0x0u64) as u64);
            // 2c78d mov DWORD PTR [rbp-0x240],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fdc0), (0x10u64) as u32);
            // 2c797 xorps xmm0,xmm0
            x0 = 0;
            // 2c79a movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 2c79e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 2c7a6 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2c7aa mov rdi,r13
            rdi = r13;
            // 2c7ad call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7b2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2c7b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7bb mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 2c7bd cmp r15d,esi
            fl = sub::<32>((r15 as u32 as u64), (rsi as u32 as u64)).1;
            // 2c7c0 jne 0x2c7d0
            if fl.ne() { pc = 0x2c7d0; continue; }
            pc = 0x2c7c2;
        }
        0x2c7c2 => {
            // block 2c7c2
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c7c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c7c2 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 2c7c9 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7ce jmp 0x2c7ff
            pc = 0x2c7ff; continue;
        }
        0x2c7d0 => {
            // block 2c7d0
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c7d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c7d0 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 2c7d2 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 2c7d9 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2c7dc call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7e1 lea rsi,[rip+0xfe19c] # 0x12a984
            rsi = m.env.data_va(258385);
            // 2c7e8 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c7ed mov rdi,rax
            rdi = rax;
            // 2c7f0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7f5 mov rdi,rax
            rdi = rax;
            // 2c7f8 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 2c7fa call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c7ff;
        }
        0x2c7ff => {
            // block 2c7ff
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c7ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c7ff lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 2c806 mov rsi,r13
            rsi = r13;
            // 2c809 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c80e mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 2c812 cmp rax,QWORD PTR [rbp-0x70]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64).1;
            // 2c816 jae 0x2c854
            if fl.ae() { pc = 0x2c854; continue; }
            pc = 0x2c818;
        }
        0x2c818 => {
            // block 2c818
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c818 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c818 mov rcx,QWORD PTR [rbp-0x2b0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // 2c81f mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 2c823 mov rcx,QWORD PTR [rbp-0x2c0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64;
            // 2c82a mov rdx,QWORD PTR [rbp-0x2b8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64;
            // 2c831 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 2c835 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 2c838 xorps xmm0,xmm0
            x0 = 0;
            // 2c83b movaps XMMWORD PTR [rbp-0x2c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd40), x0);
            // 2c842 mov QWORD PTR [rbp-0x2b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd50), (0x0u64) as u64);
            // 2c84d add QWORD PTR [rbp-0x78],0x18
            { let (r, f) = add::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64, 0x18u64);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (r) as u64);
              fl = f; }
            // 2c852 jmp 0x2c864
            pc = 0x2c864; continue;
        }
        0x2c854 => {
            // block 2c854
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c854 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c854 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 2c858 lea rsi,[rbp-0x2c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 2c85f call 0xebe8e
            { let r = { let t = m.env.stub_va(0x65b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c864;
        }
        0x2c864 => {
            // block 2c864
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c864 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c864 lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 2c86b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c870 mov QWORD PTR [rbp-0x2a8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (r14) as u64);
            // 2c877 mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 2c87e mov QWORD PTR [rbp-0x238],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rax) as u64);
            // 2c885 mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c88c mov QWORD PTR [rbp-0x2a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (rax) as u64);
            // 2c893 mov rdi,r12
            rdi = r12;
            // 2c896 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c89b mov rdi,r13
            rdi = r13;
            // 2c89e call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c8a3 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 2c8aa mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            // 2c8b1 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c8b6 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 2c8bd call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c8c2;
        }
        0x2c8c2 => {
            // block 2c8c2
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c8c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c8c2 mov rbx,QWORD PTR [rbp-0x80]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 2c8c6 lea r14,[rip+0xfde96] # 0x12a763
            r14 = m.env.data_va(257840);
            // 2c8cd mov r15,QWORD PTR [rbp-0x2f0]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd10)) as u64;
            pc = 0x2c8d4;
        }
        0x2c8d4 => {
            // block 2c8d4
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c8d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c8d4 mov rdi,r15
            rdi = r15;
            // 2c8d7 mov rsi,rbx
            rsi = rbx;
            // 2c8da call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c8df add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // 2c8e3 cmp rbx,QWORD PTR [rbp-0x78]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64).1;
            // 2c8e7 je 0x2c903
            if fl.e() { pc = 0x2c903; continue; }
            pc = 0x2c8e9;
        }
        0x2c8e9 => {
            // block 2c8e9
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c8e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c8e9 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c8ee mov rdi,r15
            rdi = r15;
            // 2c8f1 mov rsi,r14
            rsi = r14;
            // 2c8f4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c8f9 jmp 0x2c8d4
            pc = 0x2c8d4; continue;
        }
        0x2c8fb => {
            // block 2c8fb
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c8fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c8fb mov r12,rax
            r12 = rax;
            // 2c8fe jmp 0x2c99d
            pc = 0x2c99d; continue;
        }
        0x2c903 => {
            // block 2c903
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c903 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 2c907 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c90c mov rax,r15
            rax = r15;
            // 2c90f add rsp,0x2e8
            { let (r, f) = add::<64>(rsp, 0x2e8u64);
              rsp = r;
              fl = f; }
            // 2c916 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2c917 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2c919 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2c91b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2c91d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2c91f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2c920 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2c921 => {
            // block 2c921
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c921 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c921 mov r12,rax
            r12 = rax;
            pc = 0x2c924;
        }
        0x2c924 => {
            // block 2c924
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c924 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c924 mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 2c92b mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 2c932 mov rax,QWORD PTR [rbp-0x2d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64;
            // 2c939 mov QWORD PTR [rbp-0x118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rax) as u64);
            // 2c940 mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c947 mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 2c94e jmp 0x2c966
            pc = 0x2c966; continue;
        }
        0x2c950 => {
            // block 2c950
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c950 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c950 mov r12,rax
            r12 = rax;
            // 2c953 jmp 0x2c991
            pc = 0x2c991; continue;
        }
        0x2c955 => {
            // block 2c955
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c955 mov r12,rax
            r12 = rax;
            // 2c958 jmp 0x2c97e
            pc = 0x2c97e; continue;
        }
        0x2c95a => {
            // block 2c95a
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c95a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c95a mov r12,rax
            r12 = rax;
            // 2c95d lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 2c961 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c966;
        }
        0x2c966 => {
            // block 2c966
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c966 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 2c96d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c972 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 2c979 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c97e;
        }
        0x2c97e => {
            // block 2c97e
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c97e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c97e lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 2c985 mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            // 2c98c call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c991;
        }
        0x2c991 => {
            // block 2c991
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c991 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c991 lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            pc = 0x2c998;
        }
        0x2c998 => {
            // block 2c998
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c998 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c998 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c99d;
        }
        0x2c99d => {
            // block 2c99d
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c99d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c99d lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 2c9a1 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c9a6 mov rdi,r12
            rdi = r12;
            // 2c9a9 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c9ae mov r12,rax
            r12 = rax;
            // 2c9b1 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 2c9b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c9bd jmp 0x2c924
            pc = 0x2c924; continue;
        }
        0x2c9c2 => {
            // block 2c9c2
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c9c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c9c2 mov rbx,r12
            rbx = r12;
            // 2c9c5 mov r12,rax
            r12 = rax;
            pc = 0x2c9c8;
        }
        0x2c9c8 => {
            // block 2c9c8
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c9c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c9c8 mov QWORD PTR [rbp-0x2a8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (r14) as u64);
            // 2c9cf mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 2c9d6 mov QWORD PTR [rbp-0x238],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rax) as u64);
            // 2c9dd mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c9e4 mov QWORD PTR [rbp-0x2a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (rax) as u64);
            // 2c9eb jmp 0x2ca06
            pc = 0x2ca06; continue;
        }
        0x2c9ed => {
            // block 2c9ed
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c9ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c9ed mov r12,rax
            r12 = rax;
            // 2c9f0 jmp 0x2ca29
            pc = 0x2ca29; continue;
        }
        0x2c9f2 => {
            // block 2c9f2
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c9f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c9f2 mov r12,rax
            r12 = rax;
            // 2c9f5 jmp 0x2ca16
            pc = 0x2ca16; continue;
        }
        0x2c9f7 => {
            // block 2c9f7
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2c9f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c9f7 mov rbx,r12
            rbx = r12;
            // 2c9fa mov r12,rax
            r12 = rax;
            // 2c9fd lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2ca01 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2ca06;
        }
        0x2ca06 => {
            // block 2ca06
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2ca06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ca06 mov rdi,rbx
            rdi = rbx;
            // 2ca09 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ca0e mov rdi,r13
            rdi = r13;
            // 2ca11 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2ca16;
        }
        0x2ca16 => {
            // block 2ca16
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2ca16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ca16 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 2ca1d mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            // 2ca24 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2ca29;
        }
        0x2ca29 => {
            // block 2ca29
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2ca29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ca29 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 2ca30 jmp 0x2c998
            pc = 0x2c998; continue;
        }
        0x2ca35 => {
            // block 2ca35
            #[cfg(feature = "trace")] eprintln!("f_0002c3f7 pc 2ca35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ca35 mov rbx,r12
            rbx = r12;
            // 2ca38 mov r12,rax
            r12 = rax;
            // 2ca3b lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 2ca42 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ca47 jmp 0x2c9c8
            pc = 0x2c9c8; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x500ae size=1590
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000500ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x500ae;
    loop {
        match pc {
        0x500ae => {
            // block 500ae
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 500ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 500ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 500af mov rbp,rsp
            rbp = rsp;
            // 500b2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 500b4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 500b6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 500b8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 500ba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 500bb sub rsp,0x278
            { let (r, f) = sub::<64>(rsp, 0x278u64);
              rsp = r;
              fl = f; }
            // 500c2 mov rbx,rdx
            rbx = rdx;
            // 500c5 mov r14,rdi
            r14 = rdi;
            // 500c8 mov r15,QWORD PTR [rip+0xf0309] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 500cf mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 500d2 mov QWORD PTR [rbp-0x30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r15) as u64);
            // 500d6 mov QWORD PTR [rbp-0x158],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (0x0u64) as u64);
            // 500e1 lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 500e8 mov edx,0x80
            rdx = (0x80u64) as u32 as u64;
            // 500ed call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 500f2 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 500f9 mov edx,0x80
            rdx = (0x80u64) as u32 as u64;
            // 500fe mov rsi,rbx
            rsi = rbx;
            // 50101 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50106 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 50109 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5010b mov rdi,r14
            rdi = r14;
            // 5010e call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50114 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 5011a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 5011c je 0x5015c
            if fl.e() { pc = 0x5015c; continue; }
            pc = 0x5011e;
        }
        0x5011e => {
            // block 5011e
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 5011e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5011e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 50121 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 50123 mov rdi,r14
            rdi = r14;
            // 50126 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5012c lea rdx,[rbp-0x130]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 50133 lea r9,[rbp-0x158]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 5013a mov esi,0x13
            rsi = (0x13u64) as u32 as u64;
            // 5013f mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // 50144 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 50147 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 50149 call 0xebf6c
            { let r = { let t = m.env.stub_va(0x739); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5014e mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 50151 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 50153 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 50156 je 0x5061e
            if fl.e() { pc = 0x5061e; continue; }
            pc = 0x5015c;
        }
        0x5015c => {
            // block 5015c
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 5015c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5015c test BYTE PTR [rip+0x17812f],0x8 # 0x1c8292
            fl = logic::<8>(m.r8(m.env.data_va(903775)) as u64 & 0x8u64);
            // 50163 je 0x5061c
            if fl.e() { pc = 0x5061c; continue; }
            pc = 0x50169;
        }
        0x50169 => {
            // block 50169
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 50169 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50169 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 50170 lea r15,[rbp-0x258]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 50177 mov rax,QWORD PTR [rip+0xf0052] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 5017e lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 50182 mov QWORD PTR [rbp-0x260],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (rcx) as u64);
            // 50189 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 5018d mov QWORD PTR [rbp-0x1f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (rax) as u64);
            // 50194 mov rsi,r15
            rsi = r15;
            // 50197 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5019c mov QWORD PTR [rbp-0x168],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (0x0u64) as u64);
            // 501a7 mov DWORD PTR [rbp-0x160],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (0xffffffffu64) as u32);
            // 501b1 mov r12,QWORD PTR [rip+0xf0208] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 501b8 lea rbx,[r12+0x18]
            rbx = r12.wrapping_add(0x18);
            // 501bd mov QWORD PTR [rbp-0x260],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (rbx) as u64);
            // 501c4 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 501c8 mov QWORD PTR [rbp-0x1f0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (r12) as u64);
            // 501cf mov rdi,r15
            rdi = r15;
            // 501d2 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 501d7 mov QWORD PTR [rbp-0x298],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd68), (rbx) as u64);
            // 501de mov rax,QWORD PTR [rip+0xf01c3] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 501e5 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 501e9 mov QWORD PTR [rbp-0x2a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (rax) as u64);
            // 501f0 mov QWORD PTR [rbp-0x258],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (rax) as u64);
            // 501f7 mov QWORD PTR [rbp-0x200],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (0x0u64) as u64);
            // 50202 mov QWORD PTR [rbp-0x208],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8), (0x0u64) as u64);
            // 5020d mov QWORD PTR [rbp-0x210],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0), (0x0u64) as u64);
            // 50218 mov QWORD PTR [rbp-0x218],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fde8), (0x0u64) as u64);
            // 50223 mov DWORD PTR [rbp-0x1f8],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe08), (0x10u64) as u32);
            // 5022d xorps xmm0,xmm0
            x0 = 0;
            // 50230 movaps XMMWORD PTR [rbp-0x150],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_feb0), x0);
            // 50237 mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // 50242 lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 50249 mov rdi,r15
            rdi = r15;
            // 5024c call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50251 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 50258 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5025d call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50262 mov r15,rax
            r15 = rax;
            // 50265 lea rsi,[rip+0xdaa55] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 5026c lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 50273 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 50278 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5027d lea rsi,[rip+0xdaa41] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 50284 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 50289 mov rdi,rax
            rdi = rax;
            // 5028c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50291 lea rsi,[rip+0xda6e8] # 0x12a980
            rsi = m.env.data_va(258381);
            // 50298 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 5029d mov rdi,rax
            rdi = rax;
            // 502a0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 502a5 lea rsi,[rip+0xda1c9] # 0x12a475
            rsi = m.env.data_va(257090);
            // 502ac mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 502b1 mov rdi,rax
            rdi = rax;
            // 502b4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 502b9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 502bc mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 502c0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 502c4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 502c7 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 502ca mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 502ce mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 502d1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 502d5 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 502de mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 502e1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 502e5 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 502f0 mov rdi,rax
            rdi = rax;
            // 502f3 mov rsi,r15
            rsi = r15;
            // 502f6 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 502fb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 502fe mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50302 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 50306 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50309 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5030c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 50310 lea rsi,[rip+0xda669] # 0x12a980
            rsi = m.env.data_va(258381);
            // 50317 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 5031c mov rdi,rax
            rdi = rax;
            // 5031f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50324 mov rbx,rax
            rbx = rax;
            // 50327 lea rsi,[rip+0xdac58] # 0x12af86
            rsi = m.env.data_va(259923);
            // 5032e lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // 50335 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 5033a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5033f movzx edx,BYTE PTR [rbp-0x278]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd88)) as u64) as u32 as u64;
            // 50346 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 50349 jne 0x50357
            if fl.ne() { pc = 0x50357; continue; }
            pc = 0x5034b;
        }
        0x5034b => {
            // block 5034b
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 5034b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5034b lea rsi,[rbp-0x277]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd89);
            // 50352 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 50355 jmp 0x50365
            pc = 0x50365; continue;
        }
        0x50357 => {
            // block 50357
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 50357 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50357 mov rdx,QWORD PTR [rbp-0x270]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd90)) as u64;
            // 5035e mov rsi,QWORD PTR [rbp-0x268]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd98)) as u64;
            pc = 0x50365;
        }
        0x50365 => {
            // block 50365
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 50365 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50365 mov rdi,rbx
            rdi = rbx;
            // 50368 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5036d lea rsi,[rip+0xda60c] # 0x12a980
            rsi = m.env.data_va(258381);
            // 50374 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 50379 mov rdi,rax
            rdi = rax;
            // 5037c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50381 lea rsi,[rip+0xda9e8] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 50388 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 5038d mov rdi,rax
            rdi = rax;
            // 50390 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50395 lea rsi,[rip+0xda0d9] # 0x12a475
            rsi = m.env.data_va(257090);
            // 5039c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 503a1 mov rdi,rax
            rdi = rax;
            // 503a4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 503a9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 503ac mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 503b0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 503b4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 503b7 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 503ba mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 503be mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 503c1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 503c5 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 503ce mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 503d1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 503d5 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 503e0 mov rdi,rax
            rdi = rax;
            // 503e3 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 503e6 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 503eb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 503ee mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 503f2 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 503f6 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 503f9 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 503fc mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 50400 lea rsi,[rip+0xda57f] # 0x12a986
            rsi = m.env.data_va(258387);
            // 50407 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 5040c mov rdi,rax
            rdi = rax;
            // 5040f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50414 mov r15,rax
            r15 = rax;
            // 50417 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 5041a call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5041f mov rbx,rax
            rbx = rax;
            // 50422 mov rdi,rbx
            rdi = rbx;
            // 50425 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5042a mov rdi,r15
            rdi = r15;
            // 5042d mov rsi,rbx
            rsi = rbx;
            // 50430 mov rdx,rax
            rdx = rax;
            // 50433 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50438 lea rsi,[rip+0xda243] # 0x12a682
            rsi = m.env.data_va(257615);
            // 5043f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 50444 mov rdi,rax
            rdi = rax;
            // 50447 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5044c lea rsi,[rip+0xda8a7] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 50453 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 50458 mov rdi,rax
            rdi = rax;
            // 5045b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50460 lea rsi,[rip+0xda00e] # 0x12a475
            rsi = m.env.data_va(257090);
            // 50467 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 5046c mov rdi,rax
            rdi = rax;
            // 5046f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50474 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 50477 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 5047b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 5047f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50482 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50485 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 50489 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5048c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50490 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 50499 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5049c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 504a0 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 504ab mov rdi,rax
            rdi = rax;
            // 504ae mov rsi,r14
            rsi = r14;
            // 504b1 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 504b6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 504b9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 504bd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 504c1 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 504c4 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 504c7 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 504cb lea rsi,[rip+0xda83a] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 504d2 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 504d7 mov rdi,rax
            rdi = rax;
            // 504da call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 504df lea rsi,[rip+0xd9f8f] # 0x12a475
            rsi = m.env.data_va(257090);
            // 504e6 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 504eb mov rdi,rax
            rdi = rax;
            // 504ee call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 504f3 mov rbx,rax
            rbx = rax;
            // 504f6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 504f9 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 504fd mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 50501 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 50504 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 50507 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 5050b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5050e mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50512 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 5051b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5051e mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50522 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 5052d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 50530 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 50537 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 50539 mov rdi,r14
            rdi = r14;
            // 5053c call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5053e mov rdi,rbx
            rdi = rbx;
            // 50541 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 50543 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50548 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5054b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 5054f mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 50553 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50556 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50559 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 5055d lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // 50564 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50569 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 50570 lea rsi,[rbp-0x258]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 50577 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5057c test BYTE PTR [rbp-0x290],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd70)) as u64 & 0x1u64);
            // 50583 jne 0x5058e
            if fl.ne() { pc = 0x5058e; continue; }
            pc = 0x50585;
        }
        0x50585 => {
            // block 50585
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 50585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50585 lea rdx,[rbp-0x28f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fd71);
            // 5058c jmp 0x50595
            pc = 0x50595; continue;
        }
        0x5058e => {
            // block 5058e
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 5058e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5058e mov rdx,QWORD PTR [rbp-0x280]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd80)) as u64;
            pc = 0x50595;
        }
        0x50595 => {
            // block 50595
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 50595 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50595 mov r15,QWORD PTR [rip+0xefe3c] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 5059c mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 5059f lea rsi,[rip+0xda773] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 505a6 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 505ab xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 505ad call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 505b2 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 505b9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 505be mov rax,QWORD PTR [rbp-0x298]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd68)) as u64;
            // 505c5 mov QWORD PTR [rbp-0x260],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (rax) as u64);
            // 505cc mov QWORD PTR [rbp-0x1f0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (r12) as u64);
            // 505d3 mov rax,QWORD PTR [rbp-0x2a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd60)) as u64;
            // 505da mov QWORD PTR [rbp-0x258],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (rax) as u64);
            // 505e1 lea rdi,[rbp-0x218]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde8);
            // 505e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 505ed lea rdi,[rbp-0x258]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 505f4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 505f9 mov rsi,QWORD PTR [rip+0xefc08] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 50600 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 50604 lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 5060b call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50610 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 50617 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5061c;
        }
        0x5061c => {
            // block 5061c
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 5061c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5061c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5061e;
        }
        0x5061e => {
            // block 5061e
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 5061e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5061e cmp r15,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 50622 jne 0x506b4
            if fl.ne() { pc = 0x506b4; continue; }
            pc = 0x50628;
        }
        0x50628 => {
            // block 50628
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 50628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50628 add rsp,0x278
            { let (r, f) = add::<64>(rsp, 0x278u64);
              rsp = r;
              fl = f; }
            // 5062f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 50630 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 50632 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 50634 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 50636 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 50638 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 50639 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5063a => {
            // block 5063a
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 5063a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5063a mov rbx,rax
            rbx = rax;
            // 5063d lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            pc = 0x50644;
        }
        0x50644 => {
            // block 50644
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 50644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50644 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50649 jmp 0x5064e
            pc = 0x5064e; continue;
        }
        0x5064b => {
            // block 5064b
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 5064b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5064b mov rbx,rax
            rbx = rax;
            pc = 0x5064e;
        }
        0x5064e => {
            // block 5064e
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 5064e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5064e mov rax,QWORD PTR [rbp-0x298]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd68)) as u64;
            // 50655 mov QWORD PTR [rbp-0x260],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (rax) as u64);
            // 5065c mov QWORD PTR [rbp-0x1f0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (r12) as u64);
            // 50663 mov rax,QWORD PTR [rbp-0x2a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd60)) as u64;
            // 5066a mov QWORD PTR [rbp-0x258],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (rax) as u64);
            pc = 0x50671;
        }
        0x50671 => {
            // block 50671
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 50671 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50671 lea rdi,[rbp-0x218]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde8);
            // 50678 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5067d lea rdi,[rbp-0x258]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 50684 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x50689;
        }
        0x50689 => {
            // block 50689
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 50689 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50689 mov rsi,QWORD PTR [rip+0xefb78] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 50690 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 50694 lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 5069b call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x506a0;
        }
        0x506a0 => {
            // block 506a0
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 506a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 506a0 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 506a7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 506ac mov rdi,rbx
            rdi = rbx;
            // 506af call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x506b4;
        }
        0x506b4 => {
            // block 506b4
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 506b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 506b4 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x506be => {
            // block 506be
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 506be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 506be mov rbx,rax
            rbx = rax;
            // 506c1 jmp 0x50689
            pc = 0x50689; continue;
        }
        0x506c3 => {
            // block 506c3
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 506c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 506c3 mov rbx,rax
            rbx = rax;
            // 506c6 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 506cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 506d2 jmp 0x50671
            pc = 0x50671; continue;
        }
        0x506d4 => {
            // block 506d4
            #[cfg(feature = "trace")] eprintln!("f_000500ae pc 506d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 506d4 mov rbx,rax
            rbx = rax;
            // 506d7 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 506de jmp 0x50644
            pc = 0x50644; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8398 size=1584
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008398<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8398;
    loop {
        match pc {
        0x8398 => {
            // block 8398
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8398 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8398 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8399 mov rbp,rsp
            rbp = rsp;
            // 839c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 839e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 83a0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 83a2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 83a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 83a5 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 83a9 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 83ac mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 83ae mov r15,rdi
            r15 = rdi;
            // 83b1 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 83b8 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 83bf mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 83c2 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 83c8 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 83cb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 83cd jne 0x8966
            if fl.ne() { pc = 0x8966; continue; }
            pc = 0x83d3;
        }
        0x83d3 => {
            // block 83d3
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 83d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 83d3 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 83d5 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 83d8 ja 0x8966
            if fl.a() { pc = 0x8966; continue; }
            pc = 0x83de;
        }
        0x83de => {
            // block 83de
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 83de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 83de lea rcx,[rip+0x5c3] # 0x89a8
            rcx = m.env.code_va(0x89a8);
            // 83e5 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 83e9 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 83ec jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x83ee => {
            // block 83ee
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 83ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 83ee mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 83f1 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 83f5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 83f8 mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // 83fd mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8402 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8405 mov rdi,r15
            rdi = r15;
            // 8408 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 840b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 840d je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x8413;
        }
        0x8413 => {
            // block 8413
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8413 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8413 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 8417 call 0xce3af
            { let r = { let t = m.env.code_va(0xce3af); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 841c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 841e je 0x88a0
            if fl.e() { pc = 0x88a0; continue; }
            pc = 0x8424;
        }
        0x8424 => {
            // block 8424
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8424 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8424 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 8427 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 842b mov esi,0xe8
            rsi = (0xe8u64) as u32 as u64;
            // 8430 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8435 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8438 mov rdi,r15
            rdi = r15;
            // 843b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 843e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8440 je 0x88a0
            if fl.e() { pc = 0x88a0; continue; }
            pc = 0x8446;
        }
        0x8446 => {
            // block 8446
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8446 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8446 mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 8449 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 844b shr eax,0x19
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x19u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 844e and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8451 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // 8454 and r12d,0x7
            { let r = (r12 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 8458 or r12d,eax
            { let r = (r12 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 845b mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 845d shr eax,0x1b
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1bu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8460 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8463 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 8465 shr ebx,0x4
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x4u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 8468 and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 846b or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 846d and esi,0x80
            { let r = (rsi as u32 as u64) & 0x80u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8473 shr esi,0x7
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x7u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 8476 mov r13d,DWORD PTR [rbp-0x30]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 847a and r13d,0x1
            { let r = (r13 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 847e mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 8480 call 0x5a594
            { let r = { let t = m.env.code_va(0x5a594); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8485 movzx r8d,r14b
            r8 = ((r14 as u8 as u64)) as u32 as u64;
            // 8489 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 848c mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 848e mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // 8491 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 8493 call 0x6d8
            { let r = { let t = m.env.code_va(0x6d8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8498 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 849b mov BYTE PTR [rbp-0x31],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x0u64) as u8);
            // 849f mov BYTE PTR [rbp-0x32],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffce), (0x0u64) as u8);
            // 84a3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 84a6 lea rsi,[rbp-0x31]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            // 84aa xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 84ac mov rdi,r15
            rdi = r15;
            // 84af call QWORD PTR [rax+0x10b8]
            { let r = { let t = m.r64(rax.wrapping_add(0x10b8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 84b5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 84b8 lea rsi,[rbp-0x32]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffce);
            // 84bc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 84be mov rdi,r15
            rdi = r15;
            // 84c1 call QWORD PTR [rax+0x10c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x10c0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 84c7 mov al,BYTE PTR [rbp-0x32]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffce)) as u64) as u8 as u64);
            // 84ca or al,BYTE PTR [rbp-0x31]
            { let r = (rax as u8 as u64) | m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf)) as u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 84cd je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x84d3;
        }
        0x84d3 => {
            // block 84d3
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 84d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 84d3 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 84d6 call 0x5a733
            { let r = { let t = m.env.code_va(0x5a733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 84db jmp 0x8963
            pc = 0x8963; continue;
        }
        0x84e0 => {
            // block 84e0
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 84e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 84e0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 84e3 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 84e7 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 84ea mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // 84ef mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 84f4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 84f7 mov rdi,r15
            rdi = r15;
            // 84fa call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 84fd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 84ff je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x8505;
        }
        0x8505 => {
            // block 8505
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8505 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8505 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 8509 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 850e call 0xd12f4
            { let r = { let t = m.env.code_va(0xd12f4); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8513 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8515 je 0x866c
            if fl.e() { pc = 0x866c; continue; }
            pc = 0x851b;
        }
        0x851b => {
            // block 851b
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 851b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 851b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 851e lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 8522 mov esi,0xe8
            rsi = (0xe8u64) as u32 as u64;
            // 8527 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 852c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 852f mov rdi,r15
            rdi = r15;
            // 8532 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8535 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8537 je 0x866c
            if fl.e() { pc = 0x866c; continue; }
            pc = 0x853d;
        }
        0x853d => {
            // block 853d
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 853d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 853d mov r12d,DWORD PTR [rbp-0x30]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 8541 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 8544 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8547 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 854a mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 854c shr ecx,0x1a
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x1au64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 854f and ecx,0x8
            { let r = (rcx as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 8552 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 8554 shr ebx,0x8
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x8u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 8557 and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 855a or ebx,ecx
            { let r = (rbx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 855c mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 855e shr esi,0x1c
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1cu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 8561 and esi,0x8
            { let r = (rsi as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8564 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 8566 shr ecx,0xc
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0xcu64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 8569 and ecx,0x7
            { let r = (rcx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 856c or ecx,esi
            { let r = (rcx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 856e and eax,0x8000
            { let r = (rax as u32 as u64) & 0x8000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8573 cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // 8576 jne 0x8978
            if fl.ne() { pc = 0x8978; continue; }
            pc = 0x857c;
        }
        0x857c => {
            // block 857c
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 857c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 857c mov edx,DWORD PTR [r15+0x10]
            rdx = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 8580 and edx,0xfffffffe
            { let r = (rdx as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8583 cmp edx,0x10266400
            fl = sub::<32>((rdx as u32 as u64), 0x10266400u64).1;
            // 8589 jne 0x8978
            if fl.ne() { pc = 0x8978; continue; }
            pc = 0x858f;
        }
        0x858f => {
            // block 858f
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 858f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 858f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 8591 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 8594 movzx r15d,cl
            r15 = ((rcx as u8 as u64)) as u32 as u64;
            // 8598 movzx esi,al
            rsi = ((rax as u8 as u64)) as u32 as u64;
            // 859b mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // 859e call 0x5a594
            { let r = { let t = m.env.code_va(0x5a594); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 85a3 movzx r8d,r14b
            r8 = ((r14 as u8 as u64)) as u32 as u64;
            // 85a7 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 85ac mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 85ae mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 85b0 jmp 0x89a2
            pc = 0x89a2; continue;
        }
        0x85b5 => {
            // block 85b5
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 85b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 85b5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 85b8 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 85bc xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 85bf mov esi,0x120
            rsi = (0x120u64) as u32 as u64;
            // 85c4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 85c9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 85cc mov rdi,r15
            rdi = r15;
            // 85cf call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 85d2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 85d4 je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x85da;
        }
        0x85da => {
            // block 85da
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 85da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 85da mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 85de mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 85e3 call 0xd12f4
            { let r = { let t = m.env.code_va(0xd12f4); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 85e8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 85ea jne 0x8602
            if fl.ne() { pc = 0x8602; continue; }
            pc = 0x85ec;
        }
        0x85ec => {
            // block 85ec
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 85ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 85ec mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 85f0 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 85f5 call 0xd1370
            { let r = { let t = m.env.code_va(0xd1370); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 85fa test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 85fc je 0x891a
            if fl.e() { pc = 0x891a; continue; }
            pc = 0x8602;
        }
        0x8602 => {
            // block 8602
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8602 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8602 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 8605 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 8609 mov esi,0x11f
            rsi = (0x11fu64) as u32 as u64;
            // 860e jmp 0x88bb
            pc = 0x88bb; continue;
        }
        0x8613 => {
            // block 8613
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8613 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 8616 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 861a xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 861d mov esi,0x120
            rsi = (0x120u64) as u32 as u64;
            // 8622 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8627 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 862a mov rdi,r15
            rdi = r15;
            // 862d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8630 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8632 je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x8638;
        }
        0x8638 => {
            // block 8638
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8638 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8638 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 863c mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 8641 call 0xd12f4
            { let r = { let t = m.env.code_va(0xd12f4); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8646 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8648 je 0x866c
            if fl.e() { pc = 0x866c; continue; }
            pc = 0x864a;
        }
        0x864a => {
            // block 864a
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 864a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 864a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 864d lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 8651 mov esi,0x11f
            rsi = (0x11fu64) as u32 as u64;
            // 8656 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 865b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 865e mov rdi,r15
            rdi = r15;
            // 8661 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8664 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8666 jne 0x873e
            if fl.ne() { pc = 0x873e; continue; }
            pc = 0x866c;
        }
        0x866c => {
            // block 866c
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 866c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 866c mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 866f mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 8671 shr eax,0x1a
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1au64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8674 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8677 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 867a shr r15d,0x8
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x8u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 867e and r15d,0x7
            { let r = (r15 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 8682 or r15d,eax
            { let r = (r15 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 8685 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 8687 shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 868a and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 868d mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 868f shr ebx,0xc
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xcu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 8692 and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8695 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8697 and esi,0x8000
            { let r = (rsi as u32 as u64) & 0x8000u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 869d shr esi,0xf
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xfu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 86a0 jmp 0x894a
            pc = 0x894a; continue;
        }
        0x86a5 => {
            // block 86a5
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 86a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 86a5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 86a8 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 86ac xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 86af mov esi,0x1ca
            rsi = (0x1cau64) as u32 as u64;
            // 86b4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 86b9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 86bc mov rdi,r15
            rdi = r15;
            // 86bf call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 86c2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 86c4 je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x86ca;
        }
        0x86ca => {
            // block 86ca
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 86ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 86ca mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 86cd lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 86d1 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 86d4 mov esi,0x1c9
            rsi = (0x1c9u64) as u32 as u64;
            // 86d9 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 86de xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 86e1 mov rdi,r15
            rdi = r15;
            // 86e4 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 86e7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 86e9 jne 0x88cd
            if fl.ne() { pc = 0x88cd; continue; }
            pc = 0x86ef;
        }
        0x86ef => {
            // block 86ef
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 86ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 86ef jmp 0x8966
            pc = 0x8966; continue;
        }
        0x86f4 => {
            // block 86f4
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 86f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 86f4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 86f7 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 86fb xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 86fe mov esi,0x1ca
            rsi = (0x1cau64) as u32 as u64;
            // 8703 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8708 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 870b mov rdi,r15
            rdi = r15;
            // 870e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8711 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8713 je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x8719;
        }
        0x8719 => {
            // block 8719
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8719 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8719 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 871c lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 8720 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 8723 mov esi,0x1c9
            rsi = (0x1c9u64) as u32 as u64;
            // 8728 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 872d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8730 mov rdi,r15
            rdi = r15;
            // 8733 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8736 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8738 je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x873e;
        }
        0x873e => {
            // block 873e
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 873e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 873e mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 8741 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 8743 shr eax,0x1a
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1au64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8746 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8749 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 874c shr r15d,0x8
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x8u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 8750 and r15d,0x7
            { let r = (r15 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 8754 or r15d,eax
            { let r = (r15 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 8757 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 8759 shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 875c and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 875f mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 8761 shr ebx,0xc
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xcu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 8764 and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8767 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8769 and esi,0x8000
            { let r = (rsi as u32 as u64) & 0x8000u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 876f shr esi,0xf
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xfu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 8772 mov r12d,0x100
            r12 = (0x100u64) as u32 as u64;
            // 8778 and r12d,DWORD PTR [rbp-0x30]
            { let r = (r12 as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 877c shr r12d,0x8
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x8u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // 8780 jmp 0x8905
            pc = 0x8905; continue;
        }
        0x8785 => {
            // block 8785
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8785 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8785 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 8788 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 878c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 878f mov esi,0x1cb
            rsi = (0x1cbu64) as u32 as u64;
            // 8794 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8799 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 879c mov rdi,r15
            rdi = r15;
            // 879f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 87a2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 87a4 je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x87aa;
        }
        0x87aa => {
            // block 87aa
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 87aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 87aa mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 87ad lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 87b1 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 87b4 mov esi,0x1c9
            rsi = (0x1c9u64) as u32 as u64;
            // 87b9 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 87be xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 87c1 mov rdi,r15
            rdi = r15;
            // 87c4 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 87c7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 87c9 je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x87cf;
        }
        0x87cf => {
            // block 87cf
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 87cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 87cf mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 87d2 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 87d4 shr eax,0x19
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x19u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 87d7 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 87da mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 87dd and r15d,0x7
            { let r = (r15 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 87e1 or r15d,eax
            { let r = (r15 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 87e4 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 87e6 shr eax,0x1b
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1bu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 87e9 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 87ec mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 87ee shr ebx,0x4
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x4u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 87f1 and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 87f4 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 87f6 and esi,0x80
            { let r = (rsi as u32 as u64) & 0x80u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 87fc shr esi,0x7
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x7u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 87ff mov r12d,0x10000
            r12 = (0x10000u64) as u32 as u64;
            // 8805 and r12d,DWORD PTR [rbp-0x30]
            { let r = (r12 as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 8809 shr r12d,0x10
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x10u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // 880d jmp 0x8905
            pc = 0x8905; continue;
        }
        0x8812 => {
            // block 8812
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8812 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8812 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 8815 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 8819 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 881c mov esi,0x1cb
            rsi = (0x1cbu64) as u32 as u64;
            // 8821 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8826 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8829 mov rdi,r15
            rdi = r15;
            // 882c call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 882f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8831 je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x8837;
        }
        0x8837 => {
            // block 8837
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8837 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8837 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 883a lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 883e xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 8841 mov esi,0x1c9
            rsi = (0x1c9u64) as u32 as u64;
            // 8846 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 884b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 884e mov rdi,r15
            rdi = r15;
            // 8851 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8854 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8856 je 0x8966
            if fl.e() { pc = 0x8966; continue; }
            pc = 0x885c;
        }
        0x885c => {
            // block 885c
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 885c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 885c mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 885f mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 8861 shr eax,0x1a
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1au64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8864 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8867 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 886a shr r15d,0x8
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x8u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 886e and r15d,0x7
            { let r = (r15 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 8872 or r15d,eax
            { let r = (r15 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 8875 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 8877 shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 887a and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 887d mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 887f shr ebx,0xc
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xcu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 8882 and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8885 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8887 and esi,0x8000
            { let r = (rsi as u32 as u64) & 0x8000u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 888d shr esi,0xf
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xfu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 8890 mov r12d,0x1000000
            r12 = (0x1000000u64) as u32 as u64;
            // 8896 and r12d,DWORD PTR [rbp-0x30]
            { let r = (r12 as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 889a shr r12d,0x18
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x18u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // 889e jmp 0x8905
            pc = 0x8905; continue;
        }
        0x88a0 => {
            // block 88a0
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 88a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 88a0 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 88a4 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 88a6 call 0xd12f4
            { let r = { let t = m.env.code_va(0xd12f4); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 88ab test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 88ad je 0x891a
            if fl.e() { pc = 0x891a; continue; }
            pc = 0x88af;
        }
        0x88af => {
            // block 88af
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 88af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 88af mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 88b2 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 88b6 mov esi,0xe8
            rsi = (0xe8u64) as u32 as u64;
            pc = 0x88bb;
        }
        0x88bb => {
            // block 88bb
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 88bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 88bb mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 88c0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 88c3 mov rdi,r15
            rdi = r15;
            // 88c6 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 88c9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 88cb je 0x891a
            if fl.e() { pc = 0x891a; continue; }
            pc = 0x88cd;
        }
        0x88cd => {
            // block 88cd
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 88cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 88cd mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 88d0 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 88d2 shr eax,0x19
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x19u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 88d5 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 88d8 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 88db and r15d,0x7
            { let r = (r15 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 88df or r15d,eax
            { let r = (r15 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 88e2 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 88e4 shr eax,0x1b
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1bu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 88e7 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 88ea mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 88ec shr ebx,0x4
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x4u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 88ef and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 88f2 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 88f4 and esi,0x80
            { let r = (rsi as u32 as u64) & 0x80u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 88fa shr esi,0x7
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x7u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 88fd mov r12d,DWORD PTR [rbp-0x30]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 8901 and r12d,0x1
            { let r = (r12 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x8905;
        }
        0x8905 => {
            // block 8905
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8905 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8905 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 8907 call 0x5a594
            { let r = { let t = m.env.code_va(0x5a594); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 890c movzx r8d,r14b
            r8 = ((r14 as u8 as u64)) as u32 as u64;
            // 8910 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // 8913 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 8915 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 8918 jmp 0x895c
            pc = 0x895c; continue;
        }
        0x891a => {
            // block 891a
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 891a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 891a mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 891d mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 891f shr eax,0x19
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x19u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8922 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8925 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 8928 and r15d,0x7
            { let r = (r15 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 892c or r15d,eax
            { let r = (r15 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 892f mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 8931 shr eax,0x1b
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1bu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8934 and eax,0x8
            { let r = (rax as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8937 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 8939 shr ebx,0x4
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x4u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 893c and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 893f or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8941 and esi,0x80
            { let r = (rsi as u32 as u64) & 0x80u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8947 shr esi,0x7
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x7u64, fl); rsi = (r) as u32 as u64; fl = f; }
            pc = 0x894a;
        }
        0x894a => {
            // block 894a
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 894a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 894a mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 894c call 0x5a594
            { let r = { let t = m.env.code_va(0x5a594); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8951 movzx r8d,r14b
            r8 = ((r14 as u8 as u64)) as u32 as u64;
            // 8955 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8957 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // 895a mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x895c;
        }
        0x895c => {
            // block 895c
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 895c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 895c mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            pc = 0x895e;
        }
        0x895e => {
            // block 895e
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 895e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 895e call 0x6d8
            { let r = { let t = m.env.code_va(0x6d8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x8963;
        }
        0x8963 => {
            // block 8963
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8963 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x8966;
        }
        0x8966 => {
            // block 8966
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8966 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 8969 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 896d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 896e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 8970 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 8972 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8974 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8976 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8977 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x8978 => {
            // block 8978
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 8978 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8978 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 897a setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 897d and r12d,0x100
            { let r = (r12 as u32 as u64) & 0x100u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 8984 shr r12d,0x8
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x8u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // 8988 movzx r15d,cl
            r15 = ((rcx as u8 as u64)) as u32 as u64;
            // 898c movzx esi,al
            rsi = ((rax as u8 as u64)) as u32 as u64;
            // 898f mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // 8992 call 0x5a594
            { let r = { let t = m.env.code_va(0x5a594); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8997 movzx r8d,r14b
            r8 = ((r14 as u8 as u64)) as u32 as u64;
            // 899b mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 899d mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 899f mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            pc = 0x89a2;
        }
        0x89a2 => {
            // block 89a2
            #[cfg(feature = "trace")] eprintln!("f_00008398 pc 89a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89a2 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 89a5 jmp 0x895e
            pc = 0x895e; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4faa0 size=1550
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004faa0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4faa0;
    loop {
        match pc {
        0x4faa0 => {
            // block 4faa0
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4faa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4faa0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4faa1 mov rbp,rsp
            rbp = rsp;
            // 4faa4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4faa6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4faa8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4faaa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4faac push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4faad sub rsp,0x198
            { let (r, f) = sub::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // 4fab4 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 4fab6 mov r14,rdi
            r14 = rdi;
            // 4fab9 mov r12,QWORD PTR [rip+0xf0918] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // 4fac0 mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // 4fac4 mov QWORD PTR [rbp-0x30],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r12) as u64);
            // 4fac8 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 4facf mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // 4fad3 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 4fad5 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 4fad9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4fadc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4fade call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fae4 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4faea test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4faec je 0x4fb1b
            if fl.e() { pc = 0x4fb1b; continue; }
            pc = 0x4faee;
        }
        0x4faee => {
            // block 4faee
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4faee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4faee mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4faf1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4faf3 mov rdi,r14
            rdi = r14;
            // 4faf6 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fafc lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4fb00 lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 4fb04 mov esi,0x1c
            rsi = (0x1cu64) as u32 as u64;
            // 4fb09 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 4fb0e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4fb11 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4fb13 call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fb18 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x4fb1b;
        }
        0x4fb1b => {
            // block 4fb1b
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4fb1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fb1b test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 4fb1d jne 0x4ffe2
            if fl.ne() { pc = 0x4ffe2; continue; }
            pc = 0x4fb23;
        }
        0x4fb23 => {
            // block 4fb23
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4fb23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fb23 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4fb25 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4fb28 je 0x4ffe4
            if fl.e() { pc = 0x4ffe4; continue; }
            pc = 0x4fb2e;
        }
        0x4fb2e => {
            // block 4fb2e
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4fb2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fb2e test BYTE PTR [rip+0x17875e],0x10 # 0x1c8293
            fl = logic::<8>(m.r8(m.env.data_va(903776)) as u64 & 0x10u64);
            // 4fb35 je 0x4ffe2
            if fl.e() { pc = 0x4ffe2; continue; }
            pc = 0x4fb3b;
        }
        0x4fb3b => {
            // block 4fb3b
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4fb3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fb3b lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4fb42 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4fb49 mov rax,QWORD PTR [rip+0xf0680] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4fb50 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4fb54 mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 4fb5b add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4fb5f mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4fb66 mov rsi,rbx
            rsi = rbx;
            // 4fb69 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fb6e mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 4fb76 mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 4fb7d mov rax,QWORD PTR [rip+0xf083c] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 4fb84 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4fb88 mov QWORD PTR [rbp-0x1b0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rcx) as u64);
            // 4fb8f mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 4fb96 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4fb9a mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 4fba1 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4fba8 mov rdi,rbx
            rdi = rbx;
            // 4fbab call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fbb0 mov rax,QWORD PTR [rip+0xf07f1] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4fbb7 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4fbbb mov QWORD PTR [rbp-0x1b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (rax) as u64);
            // 4fbc2 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4fbc9 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4fbd4 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4fbdf mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 4fbea mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 4fbf5 mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 4fbff xorps xmm0,xmm0
            x0 = 0;
            // 4fc02 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 4fc06 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 4fc0e lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4fc12 mov rdi,rbx
            rdi = rbx;
            // 4fc15 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fc1a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4fc1e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fc23 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fc28 mov r15,rax
            r15 = rax;
            // 4fc2b lea rsi,[rip+0xdb08f] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4fc32 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4fc39 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4fc3e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fc43 lea rsi,[rip+0xdb07b] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4fc4a mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4fc4f mov rdi,rax
            rdi = rax;
            // 4fc52 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fc57 lea rsi,[rip+0xdad22] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4fc5e mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4fc63 mov rdi,rax
            rdi = rax;
            // 4fc66 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fc6b lea rsi,[rip+0xda803] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4fc72 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4fc77 mov rdi,rax
            rdi = rax;
            // 4fc7a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fc7f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fc82 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fc86 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4fc8a and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fc8d or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fc90 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4fc94 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fc97 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fc9b mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4fca4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fca7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fcab mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4fcb6 mov rdi,rax
            rdi = rax;
            // 4fcb9 mov rsi,r15
            rsi = r15;
            // 4fcbc call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fcc1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fcc4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fcc8 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4fccc and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fccf or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fcd2 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4fcd6 lea rsi,[rip+0xdaca3] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4fcdd mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4fce2 mov rdi,rax
            rdi = rax;
            // 4fce5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fcea mov rbx,rax
            rbx = rax;
            // 4fced lea rsi,[rip+0xdb285] # 0x12af79
            rsi = m.env.data_va(259910);
            // 4fcf4 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4fcfb mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 4fd00 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fd05 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 4fd0c test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4fd0f jne 0x4fd1d
            if fl.ne() { pc = 0x4fd1d; continue; }
            pc = 0x4fd11;
        }
        0x4fd11 => {
            // block 4fd11
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4fd11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fd11 lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 4fd18 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4fd1b jmp 0x4fd2b
            pc = 0x4fd2b; continue;
        }
        0x4fd1d => {
            // block 4fd1d
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4fd1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fd1d mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 4fd24 mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x4fd2b;
        }
        0x4fd2b => {
            // block 4fd2b
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4fd2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fd2b mov rdi,rbx
            rdi = rbx;
            // 4fd2e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fd33 lea rsi,[rip+0xdac46] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4fd3a mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4fd3f mov rdi,rax
            rdi = rax;
            // 4fd42 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fd47 lea rsi,[rip+0xdb022] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4fd4e mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4fd53 mov rdi,rax
            rdi = rax;
            // 4fd56 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fd5b lea rsi,[rip+0xda713] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4fd62 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4fd67 mov rdi,rax
            rdi = rax;
            // 4fd6a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fd6f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fd72 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fd76 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4fd7a and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fd7d or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fd80 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4fd84 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fd87 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fd8b mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4fd94 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fd97 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fd9b mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4fda6 mov rdi,rax
            rdi = rax;
            // 4fda9 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4fdac call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fdb1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fdb4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fdb8 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4fdbc and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fdbf or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fdc2 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4fdc6 lea rsi,[rip+0xdabb9] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4fdcd mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4fdd2 mov rdi,rax
            rdi = rax;
            // 4fdd5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fdda mov r15,rax
            r15 = rax;
            // 4fddd mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4fde0 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fde5 mov rbx,rax
            rbx = rax;
            // 4fde8 mov rdi,rbx
            rdi = rbx;
            // 4fdeb call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fdf0 mov rdi,r15
            rdi = r15;
            // 4fdf3 mov rsi,rbx
            rsi = rbx;
            // 4fdf6 mov rdx,rax
            rdx = rax;
            // 4fdf9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fdfe lea rsi,[rip+0xda87d] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4fe05 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4fe0a mov rdi,rax
            rdi = rax;
            // 4fe0d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fe12 lea rsi,[rip+0xdaee1] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4fe19 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4fe1e mov rdi,rax
            rdi = rax;
            // 4fe21 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fe26 lea rsi,[rip+0xda648] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4fe2d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4fe32 mov rdi,rax
            rdi = rax;
            // 4fe35 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fe3a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fe3d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fe41 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4fe45 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fe48 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fe4b mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4fe4f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fe52 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fe56 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4fe5f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fe62 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fe66 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4fe71 mov rdi,rax
            rdi = rax;
            // 4fe74 mov rsi,r14
            rsi = r14;
            // 4fe77 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fe7c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4fe7f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fe83 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4fe87 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fe8a or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4fe8d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4fe91 lea rsi,[rip+0xdae74] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4fe98 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4fe9d mov rdi,rax
            rdi = rax;
            // 4fea0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fea5 lea rsi,[rip+0xda5c9] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4feac mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4feb1 mov rdi,rax
            rdi = rax;
            // 4feb4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4feb9 mov rbx,rax
            rbx = rax;
            // 4febc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4febf mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fec3 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4fec7 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4feca or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4fecd mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4fed1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4fed4 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fed8 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4fee1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4fee4 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4fee8 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4fef3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4fef6 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4fefd xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4feff mov rdi,r14
            rdi = r14;
            // 4ff02 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ff04 mov rdi,rbx
            rdi = rbx;
            // 4ff07 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4ff09 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ff0e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ff11 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ff15 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ff19 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ff1c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ff1f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ff23 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4ff2a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ff2f lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4ff36 lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4ff3d call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ff42 test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 4ff49 jne 0x4ff54
            if fl.ne() { pc = 0x4ff54; continue; }
            pc = 0x4ff4b;
        }
        0x4ff4b => {
            // block 4ff4b
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4ff4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ff4b lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 4ff52 jmp 0x4ff5b
            pc = 0x4ff5b; continue;
        }
        0x4ff54 => {
            // block 4ff54
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4ff54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ff54 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x4ff5b;
        }
        0x4ff5b => {
            // block 4ff5b
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4ff5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ff5b lea rsi,[rip+0xdadb7] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4ff62 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4ff67 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4ff69 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4ff70 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ff75 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4ff7c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ff81 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4ff88 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4ff8f mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4ff96 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4ff9d mov rax,QWORD PTR [rbp-0x1b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 4ffa4 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4ffab lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4ffb2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ffb7 mov rdi,rbx
            rdi = rbx;
            // 4ffba call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ffbf mov rsi,QWORD PTR [rip+0xf0242] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4ffc6 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4ffca lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4ffd1 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ffd6 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4ffdd call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4ffe2;
        }
        0x4ffe2 => {
            // block 4ffe2
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4ffe2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ffe2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4ffe4;
        }
        0x4ffe4 => {
            // block 4ffe4
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4ffe4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ffe4 cmp r12,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4ffe8 jne 0x50081
            if fl.ne() { pc = 0x50081; continue; }
            pc = 0x4ffee;
        }
        0x4ffee => {
            // block 4ffee
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 4ffee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ffee add rsp,0x198
            { let (r, f) = add::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // 4fff5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4fff6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4fff8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4fffa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4fffc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4fffe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4ffff ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x50000 => {
            // block 50000
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 50000 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50000 mov r14,rax
            r14 = rax;
            // 50003 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x5000a;
        }
        0x5000a => {
            // block 5000a
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 5000a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5000a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5000f jmp 0x50014
            pc = 0x50014; continue;
        }
        0x50011 => {
            // block 50011
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 50011 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50011 mov r14,rax
            r14 = rax;
            pc = 0x50014;
        }
        0x50014 => {
            // block 50014
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 50014 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50014 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 5001b mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 50022 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 50029 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 50030 mov rax,QWORD PTR [rbp-0x1b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 50037 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x5003e;
        }
        0x5003e => {
            // block 5003e
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 5003e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5003e lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 50045 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5004a lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 50051 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x50056;
        }
        0x50056 => {
            // block 50056
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 50056 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50056 mov rsi,QWORD PTR [rip+0xf01ab] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 5005d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 50061 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 50068 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5006d;
        }
        0x5006d => {
            // block 5006d
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 5006d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5006d lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 50074 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50079 mov rdi,r14
            rdi = r14;
            // 5007c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x50081;
        }
        0x50081 => {
            // block 50081
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 50081 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50081 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x5008b => {
            // block 5008b
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 5008b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5008b mov r14,rax
            r14 = rax;
            // 5008e jmp 0x50056
            pc = 0x50056; continue;
        }
        0x50090 => {
            // block 50090
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 50090 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50090 mov r14,rax
            r14 = rax;
            // 50093 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 50097 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5009c jmp 0x5003e
            pc = 0x5003e; continue;
        }
        0x5009e => {
            // block 5009e
            #[cfg(feature = "trace")] eprintln!("f_0004faa0 pc 5009e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5009e mov r14,rax
            r14 = rax;
            // 500a1 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 500a8 jmp 0x5000a
            pc = 0x5000a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x48d1e size=1528
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00048d1e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x48d1e;
    loop {
        match pc {
        0x48d1e => {
            // block 48d1e
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 48d1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48d1e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 48d1f mov rbp,rsp
            rbp = rsp;
            // 48d22 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 48d24 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 48d26 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 48d28 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 48d2a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 48d2b sub rsp,0x188
            { let (r, f) = sub::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 48d32 mov r14,rdi
            r14 = rdi;
            // 48d35 mov rbx,QWORD PTR [rip+0xf769c] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 48d3c mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 48d3f mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 48d43 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 48d4a mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 48d4c mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 48d50 movsxd rax,edx
            rax = sx::<32>((rdx as u32 as u64)) as u64;
            // 48d53 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 48d57 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 48d5a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 48d5c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48d62 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 48d68 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 48d6a je 0x48da4
            if fl.e() { pc = 0x48da4; continue; }
            pc = 0x48d6c;
        }
        0x48d6c => {
            // block 48d6c
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 48d6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48d6c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 48d6f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 48d71 mov rdi,r14
            rdi = r14;
            // 48d74 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48d7a lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 48d7e lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 48d82 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 48d87 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 48d8c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 48d8f mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 48d91 call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48d96 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 48d99 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 48d9b test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 48d9e je 0x49254
            if fl.e() { pc = 0x49254; continue; }
            pc = 0x48da4;
        }
        0x48da4 => {
            // block 48da4
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 48da4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48da4 test BYTE PTR [rip+0x17f4e5],0x10 # 0x1c8290
            fl = logic::<8>(m.r8(m.env.data_va(903773)) as u64 & 0x10u64);
            // 48dab je 0x49252
            if fl.e() { pc = 0x49252; continue; }
            pc = 0x48db1;
        }
        0x48db1 => {
            // block 48db1
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 48db1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48db1 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 48db8 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 48dbf mov rax,QWORD PTR [rip+0xf740a] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 48dc6 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 48dca mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 48dd1 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 48dd5 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 48ddc mov rsi,rbx
            rsi = rbx;
            // 48ddf call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48de4 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 48dec mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 48df3 mov r12,QWORD PTR [rip+0xf75c6] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 48dfa lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 48dff mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 48e06 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 48e0d add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 48e11 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 48e18 mov rdi,rbx
            rdi = rbx;
            // 48e1b call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48e20 mov rax,QWORD PTR [rip+0xf7581] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 48e27 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 48e2b mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 48e32 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 48e39 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 48e44 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 48e4f mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 48e5a mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 48e65 mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 48e6f xorps xmm0,xmm0
            x0 = 0;
            // 48e72 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 48e76 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 48e7e lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 48e82 mov rdi,rbx
            rdi = rbx;
            // 48e85 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48e8a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 48e8e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48e93 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48e98 mov r15,rax
            r15 = rax;
            // 48e9b lea rsi,[rip+0xe1e1f] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 48ea2 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 48ea9 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 48eae call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48eb3 lea rsi,[rip+0xe1e0b] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 48eba mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 48ebf mov rdi,rax
            rdi = rax;
            // 48ec2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48ec7 lea rsi,[rip+0xe1ab2] # 0x12a980
            rsi = m.env.data_va(258381);
            // 48ece mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 48ed3 mov rdi,rax
            rdi = rax;
            // 48ed6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48edb lea rsi,[rip+0xe1593] # 0x12a475
            rsi = m.env.data_va(257090);
            // 48ee2 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 48ee7 mov rdi,rax
            rdi = rax;
            // 48eea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48eef mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 48ef2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 48ef6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 48efa and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 48efd or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 48f00 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 48f04 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 48f07 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 48f0b mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 48f14 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 48f17 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 48f1b mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 48f26 mov rdi,rax
            rdi = rax;
            // 48f29 mov rsi,r15
            rsi = r15;
            // 48f2c call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48f31 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 48f34 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 48f38 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 48f3c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 48f3f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 48f42 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 48f46 lea rsi,[rip+0xe1a33] # 0x12a980
            rsi = m.env.data_va(258381);
            // 48f4d mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 48f52 mov rdi,rax
            rdi = rax;
            // 48f55 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48f5a mov rbx,rax
            rbx = rax;
            // 48f5d lea rsi,[rip+0xe1e7c] # 0x12ade0
            rsi = m.env.data_va(259501);
            // 48f64 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 48f6b mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 48f70 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48f75 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 48f7c test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 48f7f jne 0x48f8d
            if fl.ne() { pc = 0x48f8d; continue; }
            pc = 0x48f81;
        }
        0x48f81 => {
            // block 48f81
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 48f81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48f81 lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 48f88 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 48f8b jmp 0x48f9b
            pc = 0x48f9b; continue;
        }
        0x48f8d => {
            // block 48f8d
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 48f8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48f8d mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 48f94 mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x48f9b;
        }
        0x48f9b => {
            // block 48f9b
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 48f9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48f9b mov rdi,rbx
            rdi = rbx;
            // 48f9e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48fa3 lea rsi,[rip+0xe19d6] # 0x12a980
            rsi = m.env.data_va(258381);
            // 48faa mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 48faf mov rdi,rax
            rdi = rax;
            // 48fb2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48fb7 lea rsi,[rip+0xe1db2] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 48fbe mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 48fc3 mov rdi,rax
            rdi = rax;
            // 48fc6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48fcb lea rsi,[rip+0xe14a3] # 0x12a475
            rsi = m.env.data_va(257090);
            // 48fd2 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 48fd7 mov rdi,rax
            rdi = rax;
            // 48fda call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48fdf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 48fe2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 48fe6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 48fea and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 48fed or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 48ff0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 48ff4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 48ff7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 48ffb mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 49004 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49007 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4900b mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 49016 mov rdi,rax
            rdi = rax;
            // 49019 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4901c call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49021 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49024 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49028 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4902c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4902f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49032 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49036 lea rsi,[rip+0xe1949] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4903d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 49042 mov rdi,rax
            rdi = rax;
            // 49045 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4904a mov r15,rax
            r15 = rax;
            // 4904d mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 49050 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49055 mov rbx,rax
            rbx = rax;
            // 49058 mov rdi,rbx
            rdi = rbx;
            // 4905b call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49060 mov rdi,r15
            rdi = r15;
            // 49063 mov rsi,rbx
            rsi = rbx;
            // 49066 mov rdx,rax
            rdx = rax;
            // 49069 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4906e lea rsi,[rip+0xe160d] # 0x12a682
            rsi = m.env.data_va(257615);
            // 49075 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4907a mov rdi,rax
            rdi = rax;
            // 4907d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49082 lea rsi,[rip+0xe1c71] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 49089 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4908e mov rdi,rax
            rdi = rax;
            // 49091 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49096 lea rsi,[rip+0xe13d8] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4909d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 490a2 mov rdi,rax
            rdi = rax;
            // 490a5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 490aa mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 490ad mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 490b1 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 490b5 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 490b8 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 490bb mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 490bf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 490c2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 490c6 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 490cf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 490d2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 490d6 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 490e1 mov rdi,rax
            rdi = rax;
            // 490e4 mov rsi,r14
            rsi = r14;
            // 490e7 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 490ec mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 490ef mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 490f3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 490f7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 490fa or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 490fd mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49101 lea rsi,[rip+0xe1c04] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 49108 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4910d mov rdi,rax
            rdi = rax;
            // 49110 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49115 lea rsi,[rip+0xe1359] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4911c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 49121 mov rdi,rax
            rdi = rax;
            // 49124 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49129 mov rbx,rax
            rbx = rax;
            // 4912c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4912f mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49133 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49137 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4913a or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4913d mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 49141 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 49144 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49148 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 49151 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 49154 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49158 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 49163 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49166 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4916d xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4916f mov rdi,r14
            rdi = r14;
            // 49172 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49174 mov rdi,rbx
            rdi = rbx;
            // 49177 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 49179 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4917e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49181 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49185 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49189 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4918c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4918f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49193 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4919a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4919f lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 491a6 lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 491ad call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 491b2 test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 491b9 jne 0x491c4
            if fl.ne() { pc = 0x491c4; continue; }
            pc = 0x491bb;
        }
        0x491bb => {
            // block 491bb
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 491bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 491bb lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 491c2 jmp 0x491cb
            pc = 0x491cb; continue;
        }
        0x491c4 => {
            // block 491c4
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 491c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 491c4 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x491cb;
        }
        0x491cb => {
            // block 491cb
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 491cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 491cb mov rbx,QWORD PTR [rip+0xf7206] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 491d2 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 491d5 lea rsi,[rip+0xe1b3d] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 491dc mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 491e1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 491e3 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 491e8 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 491ef call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 491f4 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 491fb mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 49202 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 49209 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 49210 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 49217 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4921e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49223 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4922a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4922f mov rsi,QWORD PTR [rip+0xf6fd2] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 49236 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4923a lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 49241 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49246 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4924d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x49252;
        }
        0x49252 => {
            // block 49252
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 49252 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49252 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x49254;
        }
        0x49254 => {
            // block 49254
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 49254 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49254 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 49258 jne 0x492ea
            if fl.ne() { pc = 0x492ea; continue; }
            pc = 0x4925e;
        }
        0x4925e => {
            // block 4925e
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 4925e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4925e add rsp,0x188
            { let (r, f) = add::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 49265 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 49266 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 49268 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4926a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4926c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4926e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4926f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x49270 => {
            // block 49270
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 49270 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49270 mov rbx,rax
            rbx = rax;
            // 49273 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x4927a;
        }
        0x4927a => {
            // block 4927a
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 4927a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4927a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4927f jmp 0x49284
            pc = 0x49284; continue;
        }
        0x49281 => {
            // block 49281
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 49281 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49281 mov rbx,rax
            rbx = rax;
            pc = 0x49284;
        }
        0x49284 => {
            // block 49284
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 49284 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49284 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4928b mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 49292 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 49299 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 492a0 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x492a7;
        }
        0x492a7 => {
            // block 492a7
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 492a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 492a7 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 492ae call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 492b3 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 492ba call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x492bf;
        }
        0x492bf => {
            // block 492bf
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 492bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 492bf mov rsi,QWORD PTR [rip+0xf6f42] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 492c6 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 492ca lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 492d1 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x492d6;
        }
        0x492d6 => {
            // block 492d6
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 492d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 492d6 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 492dd call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 492e2 mov rdi,rbx
            rdi = rbx;
            // 492e5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x492ea;
        }
        0x492ea => {
            // block 492ea
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 492ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 492ea call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x492f4 => {
            // block 492f4
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 492f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 492f4 mov rbx,rax
            rbx = rax;
            // 492f7 jmp 0x492bf
            pc = 0x492bf; continue;
        }
        0x492f9 => {
            // block 492f9
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 492f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 492f9 mov rbx,rax
            rbx = rax;
            // 492fc lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 49300 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49305 jmp 0x492a7
            pc = 0x492a7; continue;
        }
        0x49307 => {
            // block 49307
            #[cfg(feature = "trace")] eprintln!("f_00048d1e pc 49307 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49307 mov rbx,rax
            rbx = rax;
            // 4930a lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 49311 jmp 0x4927a
            pc = 0x4927a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x49f06 size=1528
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00049f06<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x49f06;
    loop {
        match pc {
        0x49f06 => {
            // block 49f06
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 49f06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49f06 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 49f07 mov rbp,rsp
            rbp = rsp;
            // 49f0a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 49f0c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 49f0e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 49f10 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 49f12 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 49f13 sub rsp,0x188
            { let (r, f) = sub::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 49f1a mov r14,rdi
            r14 = rdi;
            // 49f1d mov rbx,QWORD PTR [rip+0xf64b4] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 49f24 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 49f27 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 49f2b mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 49f32 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 49f34 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 49f38 movsxd rax,edx
            rax = sx::<32>((rdx as u32 as u64)) as u64;
            // 49f3b mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 49f3f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49f42 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 49f44 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49f4a mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 49f50 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 49f52 je 0x49f8c
            if fl.e() { pc = 0x49f8c; continue; }
            pc = 0x49f54;
        }
        0x49f54 => {
            // block 49f54
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 49f54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49f54 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49f57 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 49f59 mov rdi,r14
            rdi = r14;
            // 49f5c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49f62 lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 49f66 lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 49f6a mov esi,0x1f
            rsi = (0x1fu64) as u32 as u64;
            // 49f6f mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 49f74 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 49f77 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 49f79 call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49f7e mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 49f81 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 49f83 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 49f86 je 0x4a43c
            if fl.e() { pc = 0x4a43c; continue; }
            pc = 0x49f8c;
        }
        0x49f8c => {
            // block 49f8c
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 49f8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49f8c test BYTE PTR [rip+0x17e300],0x80 # 0x1c8293
            fl = logic::<8>(m.r8(m.env.data_va(903776)) as u64 & 0x80u64);
            // 49f93 je 0x4a43a
            if fl.e() { pc = 0x4a43a; continue; }
            pc = 0x49f99;
        }
        0x49f99 => {
            // block 49f99
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 49f99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49f99 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 49fa0 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 49fa7 mov rax,QWORD PTR [rip+0xf6222] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 49fae lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 49fb2 mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 49fb9 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 49fbd mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 49fc4 mov rsi,rbx
            rsi = rbx;
            // 49fc7 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49fcc mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 49fd4 mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 49fdb mov r12,QWORD PTR [rip+0xf63de] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 49fe2 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 49fe7 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 49fee mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 49ff5 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 49ff9 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4a000 mov rdi,rbx
            rdi = rbx;
            // 4a003 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a008 mov rax,QWORD PTR [rip+0xf6399] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4a00f add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4a013 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 4a01a mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4a021 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4a02c mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4a037 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 4a042 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 4a04d mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 4a057 xorps xmm0,xmm0
            x0 = 0;
            // 4a05a movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 4a05e mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 4a066 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4a06a mov rdi,rbx
            rdi = rbx;
            // 4a06d call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a072 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4a076 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a07b call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a080 mov r15,rax
            r15 = rax;
            // 4a083 lea rsi,[rip+0xe0c37] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4a08a lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4a091 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4a096 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a09b lea rsi,[rip+0xe0c23] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4a0a2 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4a0a7 mov rdi,rax
            rdi = rax;
            // 4a0aa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a0af lea rsi,[rip+0xe08ca] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4a0b6 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4a0bb mov rdi,rax
            rdi = rax;
            // 4a0be call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a0c3 lea rsi,[rip+0xe03ab] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4a0ca mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4a0cf mov rdi,rax
            rdi = rax;
            // 4a0d2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a0d7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a0da mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a0de mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a0e2 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a0e5 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a0e8 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a0ec mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a0ef mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a0f3 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4a0fc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a0ff mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a103 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4a10e mov rdi,rax
            rdi = rax;
            // 4a111 mov rsi,r15
            rsi = r15;
            // 4a114 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a119 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a11c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a120 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a124 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a127 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a12a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a12e lea rsi,[rip+0xe084b] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4a135 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4a13a mov rdi,rax
            rdi = rax;
            // 4a13d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a142 mov rbx,rax
            rbx = rax;
            // 4a145 lea rsi,[rip+0xe0cf5] # 0x12ae41
            rsi = m.env.data_va(259598);
            // 4a14c lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4a153 mov edx,0x28
            rdx = (0x28u64) as u32 as u64;
            // 4a158 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a15d movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 4a164 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4a167 jne 0x4a175
            if fl.ne() { pc = 0x4a175; continue; }
            pc = 0x4a169;
        }
        0x4a169 => {
            // block 4a169
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a169 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a169 lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 4a170 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4a173 jmp 0x4a183
            pc = 0x4a183; continue;
        }
        0x4a175 => {
            // block 4a175
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a175 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a175 mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 4a17c mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x4a183;
        }
        0x4a183 => {
            // block 4a183
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a183 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a183 mov rdi,rbx
            rdi = rbx;
            // 4a186 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a18b lea rsi,[rip+0xe07ee] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4a192 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4a197 mov rdi,rax
            rdi = rax;
            // 4a19a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a19f lea rsi,[rip+0xe0bca] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4a1a6 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4a1ab mov rdi,rax
            rdi = rax;
            // 4a1ae call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a1b3 lea rsi,[rip+0xe02bb] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4a1ba mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4a1bf mov rdi,rax
            rdi = rax;
            // 4a1c2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a1c7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a1ca mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a1ce mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a1d2 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a1d5 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a1d8 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a1dc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a1df mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a1e3 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4a1ec mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a1ef mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a1f3 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4a1fe mov rdi,rax
            rdi = rax;
            // 4a201 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4a204 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a209 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a20c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a210 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a214 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a217 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a21a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a21e lea rsi,[rip+0xe0761] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4a225 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4a22a mov rdi,rax
            rdi = rax;
            // 4a22d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a232 mov r15,rax
            r15 = rax;
            // 4a235 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4a238 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a23d mov rbx,rax
            rbx = rax;
            // 4a240 mov rdi,rbx
            rdi = rbx;
            // 4a243 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a248 mov rdi,r15
            rdi = r15;
            // 4a24b mov rsi,rbx
            rsi = rbx;
            // 4a24e mov rdx,rax
            rdx = rax;
            // 4a251 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a256 lea rsi,[rip+0xe0425] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4a25d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4a262 mov rdi,rax
            rdi = rax;
            // 4a265 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a26a lea rsi,[rip+0xe0a89] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4a271 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4a276 mov rdi,rax
            rdi = rax;
            // 4a279 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a27e lea rsi,[rip+0xe01f0] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4a285 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4a28a mov rdi,rax
            rdi = rax;
            // 4a28d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a292 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a295 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a299 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a29d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a2a0 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a2a3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a2a7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a2aa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a2ae mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4a2b7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a2ba mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a2be mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4a2c9 mov rdi,rax
            rdi = rax;
            // 4a2cc mov rsi,r14
            rsi = r14;
            // 4a2cf call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a2d4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a2d7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a2db mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a2df and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a2e2 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a2e5 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a2e9 lea rsi,[rip+0xe0a1c] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4a2f0 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4a2f5 mov rdi,rax
            rdi = rax;
            // 4a2f8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a2fd lea rsi,[rip+0xe0171] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4a304 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4a309 mov rdi,rax
            rdi = rax;
            // 4a30c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a311 mov rbx,rax
            rbx = rax;
            // 4a314 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4a317 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a31b mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a31f and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4a322 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4a325 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4a329 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4a32c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a330 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4a339 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4a33c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a340 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4a34b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a34e mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4a355 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4a357 mov rdi,r14
            rdi = r14;
            // 4a35a call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a35c mov rdi,rbx
            rdi = rbx;
            // 4a35f mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4a361 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a366 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a369 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a36d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a371 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a374 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a377 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a37b lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4a382 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a387 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4a38e lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4a395 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a39a test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 4a3a1 jne 0x4a3ac
            if fl.ne() { pc = 0x4a3ac; continue; }
            pc = 0x4a3a3;
        }
        0x4a3a3 => {
            // block 4a3a3
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a3a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a3a3 lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 4a3aa jmp 0x4a3b3
            pc = 0x4a3b3; continue;
        }
        0x4a3ac => {
            // block 4a3ac
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a3ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a3ac mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x4a3b3;
        }
        0x4a3b3 => {
            // block 4a3b3
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a3b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a3b3 mov rbx,QWORD PTR [rip+0xf601e] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4a3ba mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4a3bd lea rsi,[rip+0xe0955] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4a3c4 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4a3c9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4a3cb call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a3d0 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4a3d7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a3dc mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4a3e3 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4a3ea mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4a3f1 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4a3f8 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4a3ff lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4a406 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a40b lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4a412 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a417 mov rsi,QWORD PTR [rip+0xf5dea] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4a41e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4a422 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4a429 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a42e lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4a435 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4a43a;
        }
        0x4a43a => {
            // block 4a43a
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a43a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a43a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4a43c;
        }
        0x4a43c => {
            // block 4a43c
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a43c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a43c cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4a440 jne 0x4a4d2
            if fl.ne() { pc = 0x4a4d2; continue; }
            pc = 0x4a446;
        }
        0x4a446 => {
            // block 4a446
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a446 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a446 add rsp,0x188
            { let (r, f) = add::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4a44d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4a44e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4a450 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4a452 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4a454 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4a456 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4a457 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4a458 => {
            // block 4a458
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a458 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a458 mov rbx,rax
            rbx = rax;
            // 4a45b lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x4a462;
        }
        0x4a462 => {
            // block 4a462
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a462 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a462 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a467 jmp 0x4a46c
            pc = 0x4a46c; continue;
        }
        0x4a469 => {
            // block 4a469
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a469 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a469 mov rbx,rax
            rbx = rax;
            pc = 0x4a46c;
        }
        0x4a46c => {
            // block 4a46c
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a46c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a46c mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4a473 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4a47a mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4a481 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4a488 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x4a48f;
        }
        0x4a48f => {
            // block 4a48f
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a48f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a48f lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4a496 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a49b lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4a4a2 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4a4a7;
        }
        0x4a4a7 => {
            // block 4a4a7
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a4a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a4a7 mov rsi,QWORD PTR [rip+0xf5d5a] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4a4ae add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4a4b2 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4a4b9 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4a4be;
        }
        0x4a4be => {
            // block 4a4be
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a4be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a4be lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4a4c5 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a4ca mov rdi,rbx
            rdi = rbx;
            // 4a4cd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4a4d2;
        }
        0x4a4d2 => {
            // block 4a4d2
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a4d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a4d2 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x4a4dc => {
            // block 4a4dc
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a4dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a4dc mov rbx,rax
            rbx = rax;
            // 4a4df jmp 0x4a4a7
            pc = 0x4a4a7; continue;
        }
        0x4a4e1 => {
            // block 4a4e1
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a4e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a4e1 mov rbx,rax
            rbx = rax;
            // 4a4e4 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4a4e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a4ed jmp 0x4a48f
            pc = 0x4a48f; continue;
        }
        0x4a4ef => {
            // block 4a4ef
            #[cfg(feature = "trace")] eprintln!("f_00049f06 pc 4a4ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a4ef mov rbx,rax
            rbx = rax;
            // 4a4f2 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4a4f9 jmp 0x4a462
            pc = 0x4a462; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4a4fe size=1526
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004a4fe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4a4fe;
    loop {
        match pc {
        0x4a4fe => {
            // block 4a4fe
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a4fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a4fe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4a4ff mov rbp,rsp
            rbp = rsp;
            // 4a502 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4a504 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4a506 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4a508 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4a50a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4a50b sub rsp,0x188
            { let (r, f) = sub::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4a512 mov r14,rdi
            r14 = rdi;
            // 4a515 mov rbx,QWORD PTR [rip+0xf5ebc] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4a51c mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4a51f mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 4a523 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 4a52a mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // 4a52e mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 4a530 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 4a534 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a537 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4a539 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a53f mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4a545 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4a547 je 0x4a581
            if fl.e() { pc = 0x4a581; continue; }
            pc = 0x4a549;
        }
        0x4a549 => {
            // block 4a549
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a549 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a549 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a54c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4a54e mov rdi,r14
            rdi = r14;
            // 4a551 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a557 lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4a55b lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 4a55f mov esi,0x20
            rsi = (0x20u64) as u32 as u64;
            // 4a564 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 4a569 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a56c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4a56e call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a573 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4a576 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4a578 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4a57b je 0x4aa31
            if fl.e() { pc = 0x4aa31; continue; }
            pc = 0x4a581;
        }
        0x4a581 => {
            // block 4a581
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a581 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a581 test BYTE PTR [rip+0x17dd0c],0x1 # 0x1c8294
            fl = logic::<8>(m.r8(m.env.data_va(903777)) as u64 & 0x1u64);
            // 4a588 je 0x4aa2f
            if fl.e() { pc = 0x4aa2f; continue; }
            pc = 0x4a58e;
        }
        0x4a58e => {
            // block 4a58e
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a58e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a58e lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4a595 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4a59c mov rax,QWORD PTR [rip+0xf5c2d] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4a5a3 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4a5a7 mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 4a5ae add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4a5b2 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4a5b9 mov rsi,rbx
            rsi = rbx;
            // 4a5bc call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a5c1 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 4a5c9 mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 4a5d0 mov r12,QWORD PTR [rip+0xf5de9] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 4a5d7 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 4a5dc mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 4a5e3 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4a5ea add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4a5ee mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4a5f5 mov rdi,rbx
            rdi = rbx;
            // 4a5f8 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a5fd mov rax,QWORD PTR [rip+0xf5da4] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4a604 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4a608 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 4a60f mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4a616 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4a621 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4a62c mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 4a637 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 4a642 mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 4a64c xorps xmm0,xmm0
            x0 = 0;
            // 4a64f movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 4a653 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 4a65b lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4a65f mov rdi,rbx
            rdi = rbx;
            // 4a662 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a667 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4a66b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a670 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a675 mov r15,rax
            r15 = rax;
            // 4a678 lea rsi,[rip+0xe0642] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4a67f lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4a686 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4a68b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a690 lea rsi,[rip+0xe062e] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4a697 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4a69c mov rdi,rax
            rdi = rax;
            // 4a69f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a6a4 lea rsi,[rip+0xe02d5] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4a6ab mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4a6b0 mov rdi,rax
            rdi = rax;
            // 4a6b3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a6b8 lea rsi,[rip+0xdfdb6] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4a6bf mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4a6c4 mov rdi,rax
            rdi = rax;
            // 4a6c7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a6cc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a6cf mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a6d3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a6d7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a6da or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a6dd mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a6e1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a6e4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a6e8 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4a6f1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a6f4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a6f8 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4a703 mov rdi,rax
            rdi = rax;
            // 4a706 mov rsi,r15
            rsi = r15;
            // 4a709 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a70e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a711 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a715 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a719 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a71c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a71f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a723 lea rsi,[rip+0xe0256] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4a72a mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4a72f mov rdi,rax
            rdi = rax;
            // 4a732 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a737 mov rbx,rax
            rbx = rax;
            // 4a73a lea rsi,[rip+0xe0729] # 0x12ae6a
            rsi = m.env.data_va(259639);
            // 4a741 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4a748 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 4a74d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a752 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 4a759 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4a75c jne 0x4a76a
            if fl.ne() { pc = 0x4a76a; continue; }
            pc = 0x4a75e;
        }
        0x4a75e => {
            // block 4a75e
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a75e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a75e lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 4a765 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4a768 jmp 0x4a778
            pc = 0x4a778; continue;
        }
        0x4a76a => {
            // block 4a76a
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a76a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a76a mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 4a771 mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x4a778;
        }
        0x4a778 => {
            // block 4a778
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a778 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a778 mov rdi,rbx
            rdi = rbx;
            // 4a77b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a780 lea rsi,[rip+0xe01f9] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4a787 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4a78c mov rdi,rax
            rdi = rax;
            // 4a78f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a794 lea rsi,[rip+0xe05d5] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4a79b mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4a7a0 mov rdi,rax
            rdi = rax;
            // 4a7a3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a7a8 lea rsi,[rip+0xdfcc6] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4a7af mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4a7b4 mov rdi,rax
            rdi = rax;
            // 4a7b7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a7bc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a7bf mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a7c3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a7c7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a7ca or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a7cd mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a7d1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a7d4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a7d8 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4a7e1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a7e4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a7e8 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4a7f3 mov rdi,rax
            rdi = rax;
            // 4a7f6 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4a7f9 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a7fe mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a801 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a805 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a809 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a80c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a80f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a813 lea rsi,[rip+0xe016c] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4a81a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4a81f mov rdi,rax
            rdi = rax;
            // 4a822 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a827 mov r15,rax
            r15 = rax;
            // 4a82a mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4a82d call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a832 mov rbx,rax
            rbx = rax;
            // 4a835 mov rdi,rbx
            rdi = rbx;
            // 4a838 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a83d mov rdi,r15
            rdi = r15;
            // 4a840 mov rsi,rbx
            rsi = rbx;
            // 4a843 mov rdx,rax
            rdx = rax;
            // 4a846 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a84b lea rsi,[rip+0xdfe30] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4a852 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4a857 mov rdi,rax
            rdi = rax;
            // 4a85a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a85f lea rsi,[rip+0xe0494] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4a866 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4a86b mov rdi,rax
            rdi = rax;
            // 4a86e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a873 lea rsi,[rip+0xdfbfb] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4a87a mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4a87f mov rdi,rax
            rdi = rax;
            // 4a882 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a887 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a88a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a88e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a892 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a895 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a898 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a89c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a89f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a8a3 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4a8ac mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a8af mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a8b3 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4a8be mov rdi,rax
            rdi = rax;
            // 4a8c1 mov rsi,r14
            rsi = r14;
            // 4a8c4 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a8c9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a8cc mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a8d0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a8d4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a8d7 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a8da mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a8de lea rsi,[rip+0xe0427] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4a8e5 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4a8ea mov rdi,rax
            rdi = rax;
            // 4a8ed call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a8f2 lea rsi,[rip+0xdfb7c] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4a8f9 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4a8fe mov rdi,rax
            rdi = rax;
            // 4a901 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a906 mov rbx,rax
            rbx = rax;
            // 4a909 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4a90c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a910 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a914 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4a917 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4a91a mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4a91e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4a921 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a925 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4a92e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4a931 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a935 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4a940 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a943 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4a94a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4a94c mov rdi,r14
            rdi = r14;
            // 4a94f call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a951 mov rdi,rbx
            rdi = rbx;
            // 4a954 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4a956 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a95b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4a95e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4a962 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4a966 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a969 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4a96c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4a970 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4a977 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a97c lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4a983 lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4a98a call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a98f test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 4a996 jne 0x4a9a1
            if fl.ne() { pc = 0x4a9a1; continue; }
            pc = 0x4a998;
        }
        0x4a998 => {
            // block 4a998
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a998 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a998 lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 4a99f jmp 0x4a9a8
            pc = 0x4a9a8; continue;
        }
        0x4a9a1 => {
            // block 4a9a1
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a9a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a9a1 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x4a9a8;
        }
        0x4a9a8 => {
            // block 4a9a8
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4a9a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a9a8 mov rbx,QWORD PTR [rip+0xf5a29] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4a9af mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4a9b2 lea rsi,[rip+0xe0360] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4a9b9 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4a9be xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4a9c0 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a9c5 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4a9cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a9d1 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4a9d8 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4a9df mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4a9e6 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4a9ed mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4a9f4 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4a9fb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aa00 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4aa07 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aa0c mov rsi,QWORD PTR [rip+0xf57f5] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4aa13 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4aa17 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4aa1e call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aa23 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4aa2a call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4aa2f;
        }
        0x4aa2f => {
            // block 4aa2f
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aa2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aa2f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4aa31;
        }
        0x4aa31 => {
            // block 4aa31
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aa31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aa31 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4aa35 jne 0x4aac7
            if fl.ne() { pc = 0x4aac7; continue; }
            pc = 0x4aa3b;
        }
        0x4aa3b => {
            // block 4aa3b
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aa3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aa3b add rsp,0x188
            { let (r, f) = add::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4aa42 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4aa43 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4aa45 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4aa47 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4aa49 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4aa4b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4aa4c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4aa4d => {
            // block 4aa4d
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aa4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aa4d mov rbx,rax
            rbx = rax;
            // 4aa50 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x4aa57;
        }
        0x4aa57 => {
            // block 4aa57
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aa57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aa57 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aa5c jmp 0x4aa61
            pc = 0x4aa61; continue;
        }
        0x4aa5e => {
            // block 4aa5e
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aa5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aa5e mov rbx,rax
            rbx = rax;
            pc = 0x4aa61;
        }
        0x4aa61 => {
            // block 4aa61
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aa61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aa61 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4aa68 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4aa6f mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4aa76 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4aa7d mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x4aa84;
        }
        0x4aa84 => {
            // block 4aa84
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aa84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aa84 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4aa8b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aa90 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4aa97 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4aa9c;
        }
        0x4aa9c => {
            // block 4aa9c
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aa9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aa9c mov rsi,QWORD PTR [rip+0xf5765] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4aaa3 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4aaa7 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4aaae call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4aab3;
        }
        0x4aab3 => {
            // block 4aab3
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aab3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aab3 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4aaba call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aabf mov rdi,rbx
            rdi = rbx;
            // 4aac2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4aac7;
        }
        0x4aac7 => {
            // block 4aac7
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aac7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aac7 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x4aad1 => {
            // block 4aad1
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aad1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aad1 mov rbx,rax
            rbx = rax;
            // 4aad4 jmp 0x4aa9c
            pc = 0x4aa9c; continue;
        }
        0x4aad6 => {
            // block 4aad6
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aad6 mov rbx,rax
            rbx = rax;
            // 4aad9 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4aadd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aae2 jmp 0x4aa84
            pc = 0x4aa84; continue;
        }
        0x4aae4 => {
            // block 4aae4
            #[cfg(feature = "trace")] eprintln!("f_0004a4fe pc 4aae4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aae4 mov rbx,rax
            rbx = rax;
            // 4aae7 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4aaee jmp 0x4aa57
            pc = 0x4aa57; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4b0ec size=1522
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004b0ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4b0ec;
    loop {
        match pc {
        0x4b0ec => {
            // block 4b0ec
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b0ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b0ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4b0ed mov rbp,rsp
            rbp = rsp;
            // 4b0f0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4b0f2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4b0f4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4b0f6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4b0f8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4b0f9 sub rsp,0x178
            { let (r, f) = sub::<64>(rsp, 0x178u64);
              rsp = r;
              fl = f; }
            // 4b100 mov r14,rdi
            r14 = rdi;
            // 4b103 mov rbx,QWORD PTR [rip+0xf52ce] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4b10a mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4b10d mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 4b111 mov DWORD PTR [rbp-0x54],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), (0x0u64) as u32);
            // 4b118 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // 4b11b mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 4b11f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4b122 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b124 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b12a mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4b130 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4b132 je 0x4b16c
            if fl.e() { pc = 0x4b16c; continue; }
            pc = 0x4b134;
        }
        0x4b134 => {
            // block 4b134
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b134 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b134 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4b137 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b139 mov rdi,r14
            rdi = r14;
            // 4b13c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b142 lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 4b146 lea r9,[rbp-0x54]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffac);
            // 4b14a mov esi,0x19
            rsi = (0x19u64) as u32 as u64;
            // 4b14f mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 4b154 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4b157 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4b159 call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b15e mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4b161 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4b163 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4b166 je 0x4b61c
            if fl.e() { pc = 0x4b61c; continue; }
            pc = 0x4b16c;
        }
        0x4b16c => {
            // block 4b16c
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b16c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b16c test BYTE PTR [rip+0x17d120],0x2 # 0x1c8293
            fl = logic::<8>(m.r8(m.env.data_va(903776)) as u64 & 0x2u64);
            // 4b173 je 0x4b61a
            if fl.e() { pc = 0x4b61a; continue; }
            pc = 0x4b179;
        }
        0x4b179 => {
            // block 4b179
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b179 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b179 lea rdi,[rbp-0xf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 4b180 lea rbx,[rbp-0x158]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 4b187 mov rax,QWORD PTR [rip+0xf5042] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4b18e lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4b192 mov QWORD PTR [rbp-0x160],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rcx) as u64);
            // 4b199 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4b19d mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 4b1a4 mov rsi,rbx
            rsi = rbx;
            // 4b1a7 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b1ac mov QWORD PTR [rbp-0x68],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0x0u64) as u64);
            // 4b1b4 mov DWORD PTR [rbp-0x60],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0xffffffffu64) as u32);
            // 4b1bb mov r12,QWORD PTR [rip+0xf51fe] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 4b1c2 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 4b1c7 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // 4b1ce mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // 4b1d5 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4b1d9 mov QWORD PTR [rbp-0xf0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (r12) as u64);
            // 4b1e0 mov rdi,rbx
            rdi = rbx;
            // 4b1e3 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b1e8 mov rax,QWORD PTR [rip+0xf51b9] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4b1ef add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4b1f3 mov QWORD PTR [rbp-0x1a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rax) as u64);
            // 4b1fa mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 4b201 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 4b20c mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 4b217 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4b222 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4b22d mov DWORD PTR [rbp-0xf8],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x10u64) as u32);
            // 4b237 xorps xmm0,xmm0
            x0 = 0;
            // 4b23a movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 4b23e mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 4b246 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 4b24a mov rdi,rbx
            rdi = rbx;
            // 4b24d call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b252 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 4b256 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b25b call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b260 mov r15,rax
            r15 = rax;
            // 4b263 lea rsi,[rip+0xdfa57] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4b26a lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 4b271 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4b276 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b27b lea rsi,[rip+0xdfa43] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4b282 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4b287 mov rdi,rax
            rdi = rax;
            // 4b28a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b28f lea rsi,[rip+0xdf6ea] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4b296 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4b29b mov rdi,rax
            rdi = rax;
            // 4b29e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b2a3 lea rsi,[rip+0xdf1cb] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4b2aa mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4b2af mov rdi,rax
            rdi = rax;
            // 4b2b2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b2b7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b2ba mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b2be mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b2c2 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b2c5 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b2c8 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4b2cc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b2cf mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b2d3 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4b2dc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b2df mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b2e3 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4b2ee mov rdi,rax
            rdi = rax;
            // 4b2f1 mov rsi,r15
            rsi = r15;
            // 4b2f4 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b2f9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b2fc mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b300 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b304 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b307 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b30a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4b30e lea rsi,[rip+0xdf66b] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4b315 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4b31a mov rdi,rax
            rdi = rax;
            // 4b31d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b322 mov rbx,rax
            rbx = rax;
            // 4b325 lea rsi,[rip+0xdfb78] # 0x12aea4
            rsi = m.env.data_va(259697);
            // 4b32c lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 4b333 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 4b338 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b33d movzx edx,BYTE PTR [rbp-0x178]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64) as u32 as u64;
            // 4b344 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4b347 jne 0x4b355
            if fl.ne() { pc = 0x4b355; continue; }
            pc = 0x4b349;
        }
        0x4b349 => {
            // block 4b349
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b349 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b349 lea rsi,[rbp-0x177]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe89);
            // 4b350 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4b353 jmp 0x4b363
            pc = 0x4b363; continue;
        }
        0x4b355 => {
            // block 4b355
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b355 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b355 mov rdx,QWORD PTR [rbp-0x170]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 4b35c mov rsi,QWORD PTR [rbp-0x168]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            pc = 0x4b363;
        }
        0x4b363 => {
            // block 4b363
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b363 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b363 mov rdi,rbx
            rdi = rbx;
            // 4b366 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b36b lea rsi,[rip+0xdf60e] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4b372 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4b377 mov rdi,rax
            rdi = rax;
            // 4b37a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b37f lea rsi,[rip+0xdf9ea] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4b386 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4b38b mov rdi,rax
            rdi = rax;
            // 4b38e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b393 lea rsi,[rip+0xdf0db] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4b39a mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4b39f mov rdi,rax
            rdi = rax;
            // 4b3a2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b3a7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b3aa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b3ae mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b3b2 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b3b5 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b3b8 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4b3bc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b3bf mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b3c3 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4b3cc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b3cf mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b3d3 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4b3de mov rdi,rax
            rdi = rax;
            // 4b3e1 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4b3e4 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b3e9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b3ec mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b3f0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b3f4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b3f7 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b3fa mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4b3fe lea rsi,[rip+0xdf581] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4b405 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4b40a mov rdi,rax
            rdi = rax;
            // 4b40d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b412 mov r15,rax
            r15 = rax;
            // 4b415 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4b418 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b41d mov rbx,rax
            rbx = rax;
            // 4b420 mov rdi,rbx
            rdi = rbx;
            // 4b423 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b428 mov rdi,r15
            rdi = r15;
            // 4b42b mov rsi,rbx
            rsi = rbx;
            // 4b42e mov rdx,rax
            rdx = rax;
            // 4b431 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b436 lea rsi,[rip+0xdf245] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4b43d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4b442 mov rdi,rax
            rdi = rax;
            // 4b445 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b44a lea rsi,[rip+0xdf8a9] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4b451 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4b456 mov rdi,rax
            rdi = rax;
            // 4b459 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b45e lea rsi,[rip+0xdf010] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4b465 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4b46a mov rdi,rax
            rdi = rax;
            // 4b46d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b472 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b475 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b479 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b47d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b480 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b483 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4b487 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b48a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b48e mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4b497 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b49a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b49e mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4b4a9 mov rdi,rax
            rdi = rax;
            // 4b4ac mov rsi,r14
            rsi = r14;
            // 4b4af call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b4b4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b4b7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b4bb mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b4bf and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b4c2 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b4c5 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4b4c9 lea rsi,[rip+0xdf83c] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4b4d0 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4b4d5 mov rdi,rax
            rdi = rax;
            // 4b4d8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b4dd lea rsi,[rip+0xdef91] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4b4e4 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4b4e9 mov rdi,rax
            rdi = rax;
            // 4b4ec call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b4f1 mov rbx,rax
            rbx = rax;
            // 4b4f4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b4f7 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b4fb mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b4ff and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4b502 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4b505 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4b509 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b50c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b510 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4b519 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b51c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b520 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4b52b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4b52e mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4b535 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b537 mov rdi,r14
            rdi = r14;
            // 4b53a call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b53c mov rdi,rbx
            rdi = rbx;
            // 4b53f mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4b541 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b546 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b549 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b54d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b551 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b554 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b557 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4b55b lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 4b562 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b567 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 4b56e lea rsi,[rbp-0x158]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 4b575 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b57a test BYTE PTR [rbp-0x190],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64 & 0x1u64);
            // 4b581 jne 0x4b58c
            if fl.ne() { pc = 0x4b58c; continue; }
            pc = 0x4b583;
        }
        0x4b583 => {
            // block 4b583
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b583 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b583 lea rdx,[rbp-0x18f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe71);
            // 4b58a jmp 0x4b593
            pc = 0x4b593; continue;
        }
        0x4b58c => {
            // block 4b58c
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b58c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b58c mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            pc = 0x4b593;
        }
        0x4b593 => {
            // block 4b593
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b593 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b593 mov rbx,QWORD PTR [rip+0xf4e3e] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4b59a mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4b59d lea rsi,[rip+0xdf775] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4b5a4 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4b5a9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4b5ab call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b5b0 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 4b5b7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b5bc mov rax,QWORD PTR [rbp-0x198]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            // 4b5c3 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // 4b5ca mov QWORD PTR [rbp-0xf0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (r12) as u64);
            // 4b5d1 mov rax,QWORD PTR [rbp-0x1a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 4b5d8 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 4b5df lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 4b5e6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b5eb lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 4b5f2 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b5f7 mov rsi,QWORD PTR [rip+0xf4c0a] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4b5fe add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4b602 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 4b609 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b60e lea rdi,[rbp-0xf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 4b615 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4b61a;
        }
        0x4b61a => {
            // block 4b61a
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b61a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b61a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4b61c;
        }
        0x4b61c => {
            // block 4b61c
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b61c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b61c cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4b620 jne 0x4b6b2
            if fl.ne() { pc = 0x4b6b2; continue; }
            pc = 0x4b626;
        }
        0x4b626 => {
            // block 4b626
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b626 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b626 add rsp,0x178
            { let (r, f) = add::<64>(rsp, 0x178u64);
              rsp = r;
              fl = f; }
            // 4b62d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4b62e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4b630 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4b632 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4b634 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4b636 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4b637 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4b638 => {
            // block 4b638
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b638 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b638 mov rbx,rax
            rbx = rax;
            // 4b63b lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            pc = 0x4b642;
        }
        0x4b642 => {
            // block 4b642
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b642 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b647 jmp 0x4b64c
            pc = 0x4b64c; continue;
        }
        0x4b649 => {
            // block 4b649
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b649 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b649 mov rbx,rax
            rbx = rax;
            pc = 0x4b64c;
        }
        0x4b64c => {
            // block 4b64c
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b64c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b64c mov rax,QWORD PTR [rbp-0x198]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            // 4b653 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // 4b65a mov QWORD PTR [rbp-0xf0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (r12) as u64);
            // 4b661 mov rax,QWORD PTR [rbp-0x1a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 4b668 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            pc = 0x4b66f;
        }
        0x4b66f => {
            // block 4b66f
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b66f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b66f lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 4b676 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b67b lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 4b682 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4b687;
        }
        0x4b687 => {
            // block 4b687
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b687 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b687 mov rsi,QWORD PTR [rip+0xf4b7a] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4b68e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4b692 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 4b699 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4b69e;
        }
        0x4b69e => {
            // block 4b69e
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b69e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b69e lea rdi,[rbp-0xf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 4b6a5 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b6aa mov rdi,rbx
            rdi = rbx;
            // 4b6ad call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4b6b2;
        }
        0x4b6b2 => {
            // block 4b6b2
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b6b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b6b2 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x4b6bc => {
            // block 4b6bc
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b6bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b6bc mov rbx,rax
            rbx = rax;
            // 4b6bf jmp 0x4b687
            pc = 0x4b687; continue;
        }
        0x4b6c1 => {
            // block 4b6c1
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b6c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b6c1 mov rbx,rax
            rbx = rax;
            // 4b6c4 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 4b6c8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b6cd jmp 0x4b66f
            pc = 0x4b66f; continue;
        }
        0x4b6cf => {
            // block 4b6cf
            #[cfg(feature = "trace")] eprintln!("f_0004b0ec pc 4b6cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b6cf mov rbx,rax
            rbx = rax;
            // 4b6d2 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 4b6d9 jmp 0x4b642
            pc = 0x4b642; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb3e3e size=1420
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b3e3e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb3e3e;
    loop {
        match pc {
        0xb3e3e => {
            // block b3e3e
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b3e3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b3e3e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b3e3f mov rbp,rsp
            rbp = rsp;
            // b3e42 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b3e44 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b3e46 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b3e48 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b3e4a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b3e4b sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b3e52 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // b3e55 mov QWORD PTR [rbp-0x160],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rdi) as u64);
            // b3e5c lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // b3e63 lea r12,[rbp-0x150]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b3e6a mov rax,QWORD PTR [rip+0x8c35f] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b3e71 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b3e75 mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // b3e7c add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b3e80 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // b3e87 mov rsi,r12
            rsi = r12;
            // b3e8a call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3e8f mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // b3e97 mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // b3e9e mov r15,QWORD PTR [rip+0x8c51b] # 0x1403c0
            r15 = m.r64(m.env.data_va(347021)) as u64;
            // b3ea5 lea rbx,[r15+0x18]
            rbx = r15.wrapping_add(0x18);
            // b3ea9 mov QWORD PTR [rbp-0x158],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rbx) as u64);
            // b3eb0 add r15,0x40
            { let (r, f) = add::<64>(r15, 0x40u64);
              r15 = r;
              fl = f; }
            // b3eb4 mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // b3ebb mov rdi,r12
            rdi = r12;
            // b3ebe call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3ec3 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // b3eca mov r13,QWORD PTR [rip+0x8c4d7] # 0x1403a8
            r13 = m.r64(m.env.data_va(346997)) as u64;
            // b3ed1 add r13,0x10
            { let (r, f) = add::<64>(r13, 0x10u64);
              r13 = r;
              fl = f; }
            // b3ed5 mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // b3edc mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // b3ee7 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // b3ef2 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // b3efd mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // b3f08 mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // b3f12 xorps xmm0,xmm0
            x0 = 0;
            // b3f15 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // b3f19 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // b3f21 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b3f25 mov rdi,r12
            rdi = r12;
            // b3f28 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3f2d lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b3f31 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3f36 lea rsi,[rip+0x81355] # 0x135292
            rsi = m.env.data_va(301663);
            // b3f3d lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // b3f44 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // b3f49 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3f4e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b3f51 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b3f55 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b3f59 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b3f5c or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b3f5f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b3f63 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b3f66 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b3f6a or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b3f72 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b3f75 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b3f79 mov QWORD PTR [rcx+rax*1+0x18],0x4
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x4u64) as u64);
            // b3f82 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b3f85 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b3f89 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b3f94 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // b3f97 and esi,0xfff8
            { let r = (rsi as u32 as u64) & 0xfff8u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b3f9d mov rdi,rax
            rdi = rax;
            // b3fa0 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3fa5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b3fa8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b3fac mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b3fb0 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b3fb3 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b3fb6 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b3fba mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b3fbd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b3fc1 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b3fcc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b3fcf mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b3fd3 and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b3fdb lea rsi,[rip+0x762b2] # 0x12a294
            rsi = m.env.data_va(256609);
            // b3fe2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b3fe7 mov rdi,rax
            rdi = rax;
            // b3fea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3fef mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b3ff2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b3ff6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b3ffa and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b3ffd or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b4000 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b4004 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4007 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b400b or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b4013 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4016 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b401a mov QWORD PTR [rcx+rax*1+0x18],0x4
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x4u64) as u64);
            // b4023 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4026 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b402a mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b4035 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // b4038 and esi,0x7
            { let r = (rsi as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b403b mov rdi,rax
            rdi = rax;
            // b403e call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4043 mov r12,rax
            r12 = rax;
            // b4046 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b404a mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b404e mov ecx,DWORD PTR [rax+r12*1+0x8]
            rcx = (m.r32(rax.wrapping_add(r12).wrapping_add(0x8)) as u64) as u32 as u64;
            // b4053 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b4056 or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b4059 mov DWORD PTR [rax+r12*1+0x8],ecx
            m.w32(rax.wrapping_add(r12).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b405e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b4062 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4066 mov DWORD PTR [rax+r12*1+0x90],0x20
            m.w32(rax.wrapping_add(r12).wrapping_add(0x90), (0x20u64) as u32);
            // b4072 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b4076 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b407a and DWORD PTR [rax+r12*1+0x8],0xffffbfff
            { let r = m.r32(rax.wrapping_add(r12).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rax.wrapping_add(r12).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b4083 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b4087 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b408b add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // b408e lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b4092 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4097 mov rsi,QWORD PTR [rip+0x8c0f2] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b409e lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b40a2 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b40a7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b40aa mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b40ae mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b40b3 mov rdi,rax
            rdi = rax;
            // b40b6 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b40b8 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b40ba lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b40be call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b40c3 movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // b40c6 mov rdi,r12
            rdi = r12;
            // b40c9 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b40ce mov rdi,r12
            rdi = r12;
            // b40d1 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b40d6 lea rsi,[rip+0x811c3] # 0x1352a0
            rsi = m.env.data_va(301677);
            // b40dd mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // b40e2 mov rdi,r12
            rdi = r12;
            // b40e5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b40ea mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b40ed mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b40f1 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b40f5 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b40f8 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b40fb mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b40ff mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4102 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4106 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b410e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4111 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4115 mov QWORD PTR [rcx+rax*1+0x18],0x4
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x4u64) as u64);
            // b411e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4121 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4125 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b4130 mov ebx,r14d
            rbx = ((r14 as u32 as u64)) as u32 as u64;
            // b4133 shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // b4136 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b4138 and esi,0x3ff8
            { let r = (rsi as u32 as u64) & 0x3ff8u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b413e mov rdi,rax
            rdi = rax;
            // b4141 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4146 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4149 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b414d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b4151 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b4154 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b4157 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b415b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b415e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4162 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b416d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4170 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4174 and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b417c lea rsi,[rip+0x76111] # 0x12a294
            rsi = m.env.data_va(256609);
            // b4183 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b4188 mov rdi,rax
            rdi = rax;
            // b418b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4190 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4193 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4197 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b419b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b419e or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b41a1 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b41a5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b41a8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b41ac or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b41b4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b41b7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b41bb mov QWORD PTR [rcx+rax*1+0x18],0x4
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x4u64) as u64);
            // b41c4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b41c7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b41cb mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b41d6 and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b41d9 mov rdi,rax
            rdi = rax;
            // b41dc mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b41de call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b41e3 mov rbx,rax
            rbx = rax;
            // b41e6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b41e9 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b41ed mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // b41f1 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b41f4 or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b41f7 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b41fb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b41fe mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4202 mov DWORD PTR [rax+rbx*1+0x90],0x20
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x20u64) as u32);
            // b420d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b4210 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4214 and DWORD PTR [rax+rbx*1+0x8],0xffffbfff
            { let r = m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b421c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b421f mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4223 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b4226 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b422a call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b422f mov rsi,QWORD PTR [rip+0x8bf5a] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b4236 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b423a call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b423f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4242 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b4246 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b424b mov rdi,rax
            rdi = rax;
            // b424e call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4250 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b4253 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4257 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b425c movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // b4260 mov rdi,rbx
            rdi = rbx;
            // b4263 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4268 mov rdi,rbx
            rdi = rbx;
            // b426b call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4270 lea rsi,[rip+0x81037] # 0x1352ae
            rsi = m.env.data_va(301691);
            // b4277 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // b427c mov rdi,rbx
            rdi = rbx;
            // b427f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4284 and r14d,0x40000000
            { let r = (r14 as u32 as u64) & 0x40000000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b428b lea rcx,[rip+0x81029] # 0x1352bb
            rcx = m.env.data_va(301704);
            // b4292 lea rsi,[rip+0x8102b] # 0x1352c4
            rsi = m.env.data_va(301713);
            // b4299 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b429d shr r14,0x1d
            { let (r, f) = shr::<64>(r14, 0x1du64, fl); r14 = r; fl = f; }
            // b42a1 xor r14,0xa
            { let r = r14 ^ 0xau64; let f = logic::<64>(r);
              r14 = r;
              fl = f; }
            // b42a5 mov rdi,rax
            rdi = rax;
            // b42a8 mov rdx,r14
            rdx = r14;
            // b42ab call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b42b0 mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // b42b7 lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b42be call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b42c3 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b42ca mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // b42d1 mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // b42d8 mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // b42df lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // b42e6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b42eb lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b42f2 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b42f7 mov rsi,QWORD PTR [rip+0x8bf0a] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b42fe add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b4302 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // b4309 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b430e lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // b4315 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b431a mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // b4321 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b4328 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b4329 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b432b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b432d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b432f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b4331 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b4332 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb4333 => {
            // block b4333
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b4333 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4333 mov rbx,rax
            rbx = rax;
            pc = 0xb4336;
        }
        0xb4336 => {
            // block b4336
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b4336 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4336 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b433d mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // b4344 mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // b434b mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            pc = 0xb4352;
        }
        0xb4352 => {
            // block b4352
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b4352 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4352 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // b4359 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b435e lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b4365 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb436a;
        }
        0xb436a => {
            // block b436a
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b436a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b436a mov rsi,QWORD PTR [rip+0x8be97] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b4371 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b4375 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // b437c call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb4381;
        }
        0xb4381 => {
            // block b4381
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b4381 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4381 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // b4388 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b438d mov rdi,rbx
            rdi = rbx;
            // b4390 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4395 mov rbx,rax
            rbx = rax;
            // b4398 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b439c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b43a1 jmp 0xb4336
            pc = 0xb4336; continue;
        }
        0xb43a3 => {
            // block b43a3
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b43a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b43a3 mov rbx,rax
            rbx = rax;
            // b43a6 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b43aa call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b43af jmp 0xb4336
            pc = 0xb4336; continue;
        }
        0xb43b1 => {
            // block b43b1
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b43b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b43b1 mov rbx,rax
            rbx = rax;
            // b43b4 jmp 0xb4381
            pc = 0xb4381; continue;
        }
        0xb43b6 => {
            // block b43b6
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b43b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b43b6 mov rbx,rax
            rbx = rax;
            // b43b9 jmp 0xb436a
            pc = 0xb436a; continue;
        }
        0xb43bb => {
            // block b43bb
            #[cfg(feature = "trace")] eprintln!("f_000b3e3e pc b43bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b43bb mov rbx,rax
            rbx = rax;
            // b43be lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b43c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b43c7 jmp 0xb4352
            pc = 0xb4352; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xde832 size=1352
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000de832<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xde832;
    loop {
        match pc {
        0xde832 => {
            // block de832
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de832 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de832 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // de833 mov rbp,rsp
            rbp = rsp;
            // de836 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // de838 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // de83a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // de83c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // de83e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // de83f sub rsp,0xa0b8
            { let (r, f) = sub::<64>(rsp, 0xa0b8u64);
              rsp = r;
              fl = f; }
            // de846 mov rbx,rdi
            rbx = rdi;
            // de849 mov rax,QWORD PTR [rip+0x61b88] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // de850 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // de853 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // de857 mov QWORD PTR [rbp-0xa038],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5fc8), (0x0u64) as u64);
            // de862 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de865 lea rsi,[rbp-0xa03c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fc4);
            // de86c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // de86e call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de874 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de877 lea rsi,[rbp-0xa040]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fc0);
            // de87e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // de880 mov rdi,rbx
            rdi = rbx;
            // de883 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de889 mov r14d,DWORD PTR [rbp-0xa040]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5fc0)) as u64) as u32 as u64;
            // de890 cmp r14d,0xa
            fl = sub::<32>((r14 as u32 as u64), 0xau64).1;
            // de894 sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // de898 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de89b lea rsi,[rbp-0xa041]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fbf);
            // de8a2 lea rdx,[rbp-0xa042]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5fbe);
            // de8a9 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // de8ab mov rdi,rbx
            rdi = rbx;
            // de8ae call QWORD PTR [rax+0x5e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x5e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de8b4 mov esi,DWORD PTR [rbp-0xa03c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5fc4)) as u64) as u32 as u64;
            // de8ba mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // de8c0 movzx r9d,BYTE PTR [rbp-0xa042]
            r9 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_5fbe)) as u64) as u32 as u64;
            // de8c8 movzx ecx,BYTE PTR [rbp-0xa041]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_5fbf)) as u64) as u32 as u64;
            // de8cf movzx r8d,r15b
            r8 = ((r15 as u8 as u64)) as u32 as u64;
            // de8d3 lea rdi,[rbp-0xa080]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5f80);
            // de8da call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de8df cmp r14d,0xa
            fl = sub::<32>((r14 as u32 as u64), 0xau64).1;
            // de8e3 mov eax,DWORD PTR [rbp-0xa07c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f84)) as u64) as u32 as u64;
            // de8e9 mov DWORD PTR [rbp-0xa084],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c), ((rax as u32 as u64)) as u32);
            // de8ef mov eax,DWORD PTR [rbp-0xa078]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f88)) as u64) as u32 as u64;
            // de8f5 mov DWORD PTR [rbp-0xa088],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f78), ((rax as u32 as u64)) as u32);
            // de8fb mov eax,DWORD PTR [rbp-0xa080]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f80)) as u64) as u32 as u64;
            // de901 mov DWORD PTR [rbp-0xa0d4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f2c), ((rax as u32 as u64)) as u32);
            // de907 mov r12d,DWORD PTR [rbp-0xa074]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f8c)) as u64) as u32 as u64;
            // de90e jne 0xde92a
            if fl.ne() { pc = 0xde92a; continue; }
            pc = 0xde910;
        }
        0xde910 => {
            // block de910
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de910 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de910 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de913 lea rsi,[rbp-0xa084]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5f7c);
            // de91a lea rdx,[rbp-0xa088]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5f78);
            // de921 mov rdi,rbx
            rdi = rbx;
            // de924 call QWORD PTR [rax+0x1a00]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0xde92a;
        }
        0xde92a => {
            // block de92a
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de92a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de92a mov rcx,QWORD PTR [rbx+0x280]
            rcx = m.r64(rbx.wrapping_add(0x280)) as u64;
            // de931 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // de933 mov QWORD PTR [rbp-0xa0d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f30), (rax) as u64);
            // de93a test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // de93d jne 0xde98c
            if fl.ne() { pc = 0xde98c; continue; }
            pc = 0xde93f;
        }
        0xde93f => {
            // block de93f
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de93f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de93f cmp BYTE PTR [rbx+0x279],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x279)) as u64, 0x0u64).1;
            // de946 je 0xde963
            if fl.e() { pc = 0xde963; continue; }
            pc = 0xde948;
        }
        0xde948 => {
            // block de948
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de948 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de948 mov edi,DWORD PTR [rbp-0xa088]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de94e imul edi,DWORD PTR [rbp-0xa0d4]
            { let (r, f) = imul::<32>((rdi as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f2c)) as u64); rdi = (r) as u32 as u64; fl = f; }
            // de955 shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // de959 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de95e mov rcx,rax
            rcx = rax;
            // de961 jmp 0xde985
            pc = 0xde985; continue;
        }
        0xde963 => {
            // block de963
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de963 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de966 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // de96c lea rdx,[rbp-0xa038]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5fc8);
            // de973 mov rdi,rbx
            rdi = rbx;
            // de976 call QWORD PTR [rax+0xa10]
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de97c mov rcx,QWORD PTR [rbp-0xa038]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5fc8)) as u64;
            // de983 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xde985;
        }
        0xde985 => {
            // block de985
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de985 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de985 mov QWORD PTR [rbp-0xa0d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f30), (rax) as u64);
            pc = 0xde98c;
        }
        0xde98c => {
            // block de98c
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de98c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de98c mov QWORD PTR [rbp-0xa098],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f68), (rbx) as u64);
            // de993 mov rbx,rcx
            rbx = rcx;
            // de996 lea rdi,[rbp-0xa030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // de99d mov esi,0x800
            rsi = (0x800u64) as u32 as u64;
            // de9a2 call 0x5932c
            { let r = { let t = m.env.code_va(0x5932c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de9a7 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // de9aa je 0xdea0b
            if fl.e() { pc = 0xdea0b; continue; }
            pc = 0xde9ac;
        }
        0xde9ac => {
            // block de9ac
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de9ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de9ac lea r14,[rbp-0xa030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // de9b3 lea r15,[rbp-0x8030]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // de9ba mov DWORD PTR [rbp-0xa0a0],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f60), ((r12 as u32 as u64)) as u32);
            // de9c1 mov r13d,r12d
            r13 = ((r12 as u32 as u64)) as u32 as u64;
            // de9c4 mov r12,rbx
            r12 = rbx;
            // de9c7 mov rbx,QWORD PTR [rbp-0xa098]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u64;
            pc = 0xde9ce;
        }
        0xde9ce => {
            // block de9ce
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc de9ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de9ce mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de9d1 mov ecx,DWORD PTR [rbp-0xa084]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            // de9d7 mov rdi,rbx
            rdi = rbx;
            // de9da mov rsi,r14
            rsi = r14;
            // de9dd mov rdx,r15
            rdx = r15;
            // de9e0 call QWORD PTR [rax+0x19e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de9e6 mov edx,DWORD PTR [rbp-0xa088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de9ec shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // de9ef mov rdi,r12
            rdi = r12;
            // de9f2 mov rsi,r15
            rsi = r15;
            // de9f5 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de9fa mov eax,DWORD PTR [rbp-0xa088]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // dea00 lea r12,[r12+rax*4]
            r12 = r12.wrapping_add(rax.wrapping_mul(4));
            // dea04 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // dea07 jne 0xde9ce
            if fl.ne() { pc = 0xde9ce; continue; }
            pc = 0xdea09;
        }
        0xdea09 => {
            // block dea09
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dea09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dea09 jmp 0xdea1c
            pc = 0xdea1c; continue;
        }
        0xdea0b => {
            // block dea0b
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dea0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dea0b mov DWORD PTR [rbp-0xa0a0],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f60), ((r12 as u32 as u64)) as u32);
            // dea12 mov r12,rbx
            r12 = rbx;
            // dea15 mov rbx,QWORD PTR [rbp-0xa098]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u64;
            pc = 0xdea1c;
        }
        0xdea1c => {
            // block dea1c
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dea1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dea1c mov eax,DWORD PTR [rbp-0xa084]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            // dea22 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // dea25 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // dea2a movsd xmm1,QWORD PTR [rip+0x2455e] # 0x102f90
            x1 = m.r64(m.env.data_va(96093)) as u128;
            // dea32 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // dea36 movsd QWORD PTR [rbp-0xa0a8],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f58), x1 as u64);
            // dea3e movzx ecx,WORD PTR [rbx+0x270]
            rcx = (m.r16(rbx.wrapping_add(0x270)) as u64) as u32 as u64;
            // dea45 test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // dea48 mov r14,rbx
            r14 = rbx;
            // dea4b jne 0xdea72
            if fl.ne() { pc = 0xdea72; continue; }
            pc = 0xdea4d;
        }
        0xdea4d => {
            // block dea4d
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dea4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dea4d and ecx,0x6
            { let r = (rcx as u32 as u64) & 0x6u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dea50 mov edx,DWORD PTR [rbp-0xa0a0]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f60)) as u64) as u32 as u64;
            // dea56 je 0xdea78
            if fl.e() { pc = 0xdea78; continue; }
            pc = 0xdea58;
        }
        0xdea58 => {
            // block dea58
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dea58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dea58 movsd xmm0,QWORD PTR [rbp-0xa0a8]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f58)) as u128;
            // dea60 mulsd xmm0,QWORD PTR [rip+0x1fe70] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // dea68 movsd QWORD PTR [rbp-0xa0a8],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f58), x0 as u64);
            // dea70 jmp 0xdea78
            pc = 0xdea78; continue;
        }
        0xdea72 => {
            // block dea72
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dea72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dea72 mov edx,DWORD PTR [rbp-0xa0a0]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f60)) as u64) as u32 as u64;
            pc = 0xdea78;
        }
        0xdea78 => {
            // block dea78
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dea78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dea78 mov ecx,DWORD PTR [rbp-0xa0d4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f2c)) as u64) as u32 as u64;
            // dea7e sub ecx,edx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dea80 mov DWORD PTR [rbp-0xa0bc],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f44), ((rcx as u32 as u64)) as u32);
            // dea86 je 0xdec64
            if fl.e() { pc = 0xdec64; continue; }
            pc = 0xdea8c;
        }
        0xdea8c => {
            // block dea8c
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dea8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dea8c mov QWORD PTR [rbp-0xa0b8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f48), (r12) as u64);
            // dea93 mov ecx,DWORD PTR [rbp-0xa0bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f44)) as u64) as u32 as u64;
            // dea99 xorps xmm0,xmm0
            x0 = 0;
            // dea9c cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // deaa1 mulsd xmm0,QWORD PTR [rip+0x1fe2f] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // deaa9 movsd QWORD PTR [rbp-0xa0c8],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f38), x0 as u64);
            // deab1 mov DWORD PTR [rbp-0xa0ac],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f54), (0x0u64) as u32);
            // deabb jmp 0xdead4
            pc = 0xdead4; continue;
        }
        0xdeabd => {
            // block deabd
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc deabd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // deabd mov eax,DWORD PTR [rbp-0xa088]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // deac3 lea rbx,[rbx+rax*4]
            rbx = rbx.wrapping_add(rax.wrapping_mul(4));
            // deac7 mov QWORD PTR [rbp-0xa0b8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f48), (rbx) as u64);
            // deace mov eax,DWORD PTR [rbp-0xa084]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            pc = 0xdead4;
        }
        0xdead4 => {
            // block dead4
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dead4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dead4 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // dead6 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // deadb je 0xdec12
            if fl.e() { pc = 0xdec12; continue; }
            pc = 0xdeae1;
        }
        0xdeae1 => {
            // block deae1
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc deae1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // deae1 mov ecx,DWORD PTR [rbp-0xa0ac]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f54)) as u64) as u32 as u64;
            // deae7 xorps xmm0,xmm0
            x0 = 0;
            // deaea cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // deaef subsd xmm0,QWORD PTR [rbp-0xa0c8]
            x0 = set_lo64(x0, (f64_of(x0) - f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f38)) as u128))).to_bits());
            // deaf7 mulsd xmm0,xmm0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x0)).to_bits());
            // deafb movsd QWORD PTR [rbp-0xa0a0],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f60), x0 as u64);
            // deb03 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // deb06 mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            // deb0c mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xdeb0e;
        }
        0xdeb0e => {
            // block deb0e
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc deb0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // deb0e mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // deb11 cvtsi2sd xmm1,rax
            x1 = set_lo64(x1, (sx::<64>(rax) as f64).to_bits());
            // deb16 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // deb18 xorps xmm0,xmm0
            x0 = 0;
            // deb1b cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // deb20 mulsd xmm0,QWORD PTR [rip+0x24470] # 0x102f98
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(96101)) as u128))).to_bits());
            // deb28 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // deb2c mulsd xmm0,xmm0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x0)).to_bits());
            // deb30 addsd xmm0,QWORD PTR [rbp-0xa0a0]
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f60)) as u128))).to_bits());
            // deb38 mulsd xmm0,QWORD PTR [rbp-0xa0a8]
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f58)) as u128))).to_bits());
            // deb40 mov eax,0x1f6
            rax = (0x1f6u64) as u32 as u64;
            // deb45 test BYTE PTR [r14+0x270],0x1
            fl = logic::<8>(m.r8(r14.wrapping_add(0x270)) as u64 & 0x1u64);
            // deb4d je 0xdeb6d
            if fl.e() { pc = 0xdeb6d; continue; }
            pc = 0xdeb4f;
        }
        0xdeb4f => {
            // block deb4f
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc deb4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // deb4f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // deb52 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // deb54 mov rdi,r14
            rdi = r14;
            // deb57 movsd QWORD PTR [rbp-0xa098],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f68), x0 as u64);
            // deb5f call QWORD PTR [rax+0x19d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19d8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // deb65 movsd xmm0,QWORD PTR [rbp-0xa098]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u128;
            pc = 0xdeb6d;
        }
        0xdeb6d => {
            // block deb6d
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc deb6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // deb6d mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // deb70 mov WORD PTR [rbp+rcx*2-0xa030],ax
            m.w16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((rax as u16 as u64)) as u16);
            // deb78 test r13b,0x1
            fl = logic::<8>((r13 as u8 as u64) & 0x1u64);
            // deb7c jne 0xdebfc
            if fl.ne() { pc = 0xdebfc; continue; }
            pc = 0xdeb7e;
        }
        0xdeb7e => {
            // block deb7e
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc deb7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // deb7e lea ebx,[r15-0x1]
            rbx = (r15.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // deb82 movzx ecx,WORD PTR [r14+0x270]
            rcx = (m.r16(r14.wrapping_add(0x270)) as u64) as u32 as u64;
            // deb8a mov r12,r14
            r12 = r14;
            // deb8d mov r14d,0x200
            r14 = (0x200u64) as u32 as u64;
            // deb93 test cl,0x2
            fl = logic::<8>((rcx as u8 as u64) & 0x2u64);
            // deb96 mov eax,0x200
            rax = (0x200u64) as u32 as u64;
            // deb9b je 0xdebc8
            if fl.e() { pc = 0xdebc8; continue; }
            pc = 0xdeb9d;
        }
        0xdeb9d => {
            // block deb9d
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc deb9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // deb9d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // deba1 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // deba6 mov rdi,r12
            rdi = r12;
            // deba9 movsd QWORD PTR [rbp-0xa098],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f68), x0 as u64);
            // debb1 call QWORD PTR [rax+0x19d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19d8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // debb7 movsd xmm0,QWORD PTR [rbp-0xa098]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u128;
            // debbf movzx ecx,WORD PTR [r12+0x270]
            rcx = (m.r16(r12.wrapping_add(0x270)) as u64) as u32 as u64;
            pc = 0xdebc8;
        }
        0xdebc8 => {
            // block debc8
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc debc8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // debc8 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // debca mov WORD PTR [rbp+rdx*2-0xa030],ax
            m.w16(rbp.wrapping_add(rdx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((rax as u16 as u64)) as u16);
            // debd2 test cl,0x4
            fl = logic::<8>((rcx as u8 as u64) & 0x4u64);
            // debd5 je 0xdebec
            if fl.e() { pc = 0xdebec; continue; }
            pc = 0xdebd7;
        }
        0xdebd7 => {
            // block debd7
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc debd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // debd7 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // debdb mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // debe0 mov rdi,r12
            rdi = r12;
            // debe3 call QWORD PTR [rax+0x19d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19d8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // debe9 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xdebec;
        }
        0xdebec => {
            // block debec
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc debec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // debec lea eax,[r15+0x1]
            rax = (r15.wrapping_add(0x1)) as u32 as u64;
            // debf0 mov WORD PTR [rbp+rax*2-0xa030],r14w
            m.w16(rbp.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((r14 as u16 as u64)) as u16);
            // debf9 mov r14,r12
            r14 = r12;
            pc = 0xdebfc;
        }
        0xdebfc => {
            // block debfc
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc debfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // debfc inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // debff mov ecx,DWORD PTR [rbp-0xa084]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            // dec05 add r15d,0x2
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x2u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // dec09 cmp r13d,ecx
            fl = sub::<32>((r13 as u32 as u64), (rcx as u32 as u64)).1;
            // dec0c jb 0xdeb0e
            if fl.b() { pc = 0xdeb0e; continue; }
            pc = 0xdec12;
        }
        0xdec12 => {
            // block dec12
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dec12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dec12 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dec15 mov rdi,r14
            rdi = r14;
            // dec18 lea rsi,[rbp-0xa030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dec1f lea r15,[rbp-0x8030]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // dec26 mov rdx,r15
            rdx = r15;
            // dec29 call QWORD PTR [rax+0x19e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // dec2f mov edx,DWORD PTR [rbp-0xa088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // dec35 shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // dec38 mov rbx,QWORD PTR [rbp-0xa0b8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f48)) as u64;
            // dec3f mov rdi,rbx
            rdi = rbx;
            // dec42 mov rsi,r15
            rsi = r15;
            // dec45 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // dec4a mov eax,DWORD PTR [rbp-0xa0ac]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f54)) as u64) as u32 as u64;
            // dec50 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // dec52 mov DWORD PTR [rbp-0xa0ac],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f54), ((rax as u32 as u64)) as u32);
            // dec58 cmp eax,DWORD PTR [rbp-0xa0bc]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f44)) as u64).1;
            // dec5e jne 0xdeabd
            if fl.ne() { pc = 0xdeabd; continue; }
            pc = 0xdec64;
        }
        0xdec64 => {
            // block dec64
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dec64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dec64 cmp QWORD PTR [rbp-0xa0d0],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f30)) as u64, 0x0u64).1;
            // dec6c je 0xdecf7
            if fl.e() { pc = 0xdecf7; continue; }
            pc = 0xdec72;
        }
        0xdec72 => {
            // block dec72
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dec72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dec72 mov al,BYTE PTR [r14+0x279]
            rax = (rax & !0xff) | ((m.r8(r14.wrapping_add(0x279)) as u64) as u8 as u64);
            // dec79 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dec7b je 0xdecf7
            if fl.e() { pc = 0xdecf7; continue; }
            pc = 0xdec7d;
        }
        0xdec7d => {
            // block dec7d
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc dec7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dec7d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dec80 mov esi,DWORD PTR [r14+0x250]
            rsi = (m.r32(r14.wrapping_add(0x250)) as u64) as u32 as u64;
            // dec87 lea rdx,[rbp-0xa08c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5f74);
            // dec8e mov rdi,r14
            rdi = r14;
            // dec91 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // dec97 mov rdi,r14
            rdi = r14;
            // dec9a mov r14d,DWORD PTR [rbp-0xa08c]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f74)) as u64) as u32 as u64;
            // deca1 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // deca4 mov rbx,rdi
            rbx = rdi;
            // deca7 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // decad cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // decb0 jbe 0xdecc0
            if fl.be() { pc = 0xdecc0; continue; }
            pc = 0xdecb2;
        }
        0xdecb2 => {
            // block decb2
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc decb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // decb2 mov DWORD PTR [rbp-0xa08c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f74), (0x0u64) as u32);
            // decbc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // decbe jmp 0xdecc6
            pc = 0xdecc6; continue;
        }
        0xdecc0 => {
            // block decc0
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc decc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // decc0 mov esi,DWORD PTR [rbp-0xa08c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f74)) as u64) as u32 as u64;
            pc = 0xdecc6;
        }
        0xdecc6 => {
            // block decc6
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc decc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // decc6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // decc9 mov ecx,DWORD PTR [rbp-0xa0d4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f2c)) as u64) as u32 as u64;
            // deccf imul ecx,DWORD PTR [rbp-0xa088]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // decd6 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // decd9 mov rdi,rbx
            rdi = rbx;
            // decdc mov r14,QWORD PTR [rbp-0xa0d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f30)) as u64;
            // dece3 mov rdx,r14
            rdx = r14;
            // dece6 call QWORD PTR [rax+0x2c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // decec mov rdi,r14
            rdi = r14;
            // decef mov r14,rbx
            r14 = rbx;
            // decf2 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0xdecf7;
        }
        0xdecf7 => {
            // block decf7
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc decf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // decf7 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // decfa movzx esi,BYTE PTR [r14+0x278]
            rsi = (m.r8(r14.wrapping_add(0x278)) as u64) as u32 as u64;
            // ded02 mov rdi,r14
            rdi = r14;
            // ded05 call QWORD PTR [rax+0x570]
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // ded0b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ded0e mov esi,DWORD PTR [r14+0x250]
            rsi = (m.r32(r14.wrapping_add(0x250)) as u64) as u32 as u64;
            // ded15 mov edx,DWORD PTR [r14+0x274]
            rdx = (m.r32(r14.wrapping_add(0x274)) as u64) as u32 as u64;
            // ded1c mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // ded21 mov rdi,r14
            rdi = r14;
            // ded24 call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // ded2a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ded2d mov esi,DWORD PTR [r14+0x250]
            rsi = (m.r32(r14.wrapping_add(0x250)) as u64) as u32 as u64;
            // ded34 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ded36 mov rdi,r14
            rdi = r14;
            // ded39 call QWORD PTR [rax+0x4f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // ded3f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ded42 mov esi,DWORD PTR [r14+0x250]
            rsi = (m.r32(r14.wrapping_add(0x250)) as u64) as u32 as u64;
            // ded49 mov rdi,r14
            rdi = r14;
            // ded4c call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // ded52 mov rax,QWORD PTR [rip+0x6167f] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // ded59 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // ded5c cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // ded60 jne 0xded74
            if fl.ne() { pc = 0xded74; continue; }
            pc = 0xded62;
        }
        0xded62 => {
            // block ded62
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc ded62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ded62 add rsp,0xa0b8
            { let (r, f) = add::<64>(rsp, 0xa0b8u64);
              rsp = r;
              fl = f; }
            // ded69 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ded6a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ded6c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ded6e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ded70 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ded72 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ded73 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xded74 => {
            // block ded74
            #[cfg(feature = "trace")] eprintln!("f_000de832 pc ded74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ded74 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
