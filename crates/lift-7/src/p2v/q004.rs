//! Mechanically lifted functions (scripts/lift_run.py), module q004. Each one is checked against the original
//! in the world harness (tests/world_q004.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe13d0 size=846
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e13d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe13d0;
    loop {
        match pc {
        0xe13d0 => {
            // block e13d0
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e13d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e13d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e13d1 mov rbp,rsp
            rbp = rsp;
            // e13d4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e13d6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e13d8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e13da push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e13dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e13dd sub rsp,0xb8
            { let (r, f) = sub::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // e13e4 mov rbx,r8
            rbx = r8;
            // e13e7 mov QWORD PTR [rbp-0xd8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rcx) as u64);
            // e13ee mov r13,rdx
            r13 = rdx;
            // e13f1 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e13f4 mov r12,rdi
            r12 = rdi;
            // e13f7 call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e13fc mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // e13ff test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e1401 je 0xe1638
            if fl.e() { pc = 0xe1638; continue; }
            pc = 0xe1407;
        }
        0xe1407 => {
            // block e1407
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1407 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1407 mov rdi,r12
            rdi = r12;
            // e140a call 0xe494e
            { let r = { let t = m.env.code_va(0xe494e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e140f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e1411 je 0xe1601
            if fl.e() { pc = 0xe1601; continue; }
            pc = 0xe1417;
        }
        0xe1417 => {
            // block e1417
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1417 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1417 test BYTE PTR [r13+0x0],0x1
            fl = logic::<8>(m.r8(r13) as u64 & 0x1u64);
            // e141c jne 0xe1423
            if fl.ne() { pc = 0xe1423; continue; }
            pc = 0xe141e;
        }
        0xe141e => {
            // block e141e
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e141e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e141e inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // e1421 jmp 0xe1427
            pc = 0xe1427; continue;
        }
        0xe1423 => {
            // block e1423
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1423 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1423 mov r13,QWORD PTR [r13+0x10]
            r13 = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0xe1427;
        }
        0xe1427 => {
            // block e1427
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1427 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1427 mov r14d,r15d
            r14 = ((r15 as u32 as u64)) as u32 as u64;
            // e142a mov r9,QWORD PTR [rbp-0xd8]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff28)) as u64;
            // e1431 test BYTE PTR [r9],0x1
            fl = logic::<8>(m.r8(r9) as u64 & 0x1u64);
            // e1435 jne 0xe143c
            if fl.ne() { pc = 0xe143c; continue; }
            pc = 0xe1437;
        }
        0xe1437 => {
            // block e1437
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1437 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1437 inc r9
            { let (r, f) = inc::<64>(r9, fl); r9 = r; fl = f; }
            // e143a jmp 0xe1440
            pc = 0xe1440; continue;
        }
        0xe143c => {
            // block e143c
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e143c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e143c mov r9,QWORD PTR [r9+0x10]
            r9 = m.r64(r9.wrapping_add(0x10)) as u64;
            pc = 0xe1440;
        }
        0xe1440 => {
            // block e1440
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1440 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1440 lea rdi,[r12+0x34]
            rdi = r12.wrapping_add(0x34);
            // e1445 test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // e1448 jne 0xe144f
            if fl.ne() { pc = 0xe144f; continue; }
            pc = 0xe144a;
        }
        0xe144a => {
            // block e144a
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e144a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e144a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e144d jmp 0xe1453
            pc = 0xe1453; continue;
        }
        0xe144f => {
            // block e144f
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e144f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e144f mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xe1453;
        }
        0xe1453 => {
            // block e1453
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1453 mov QWORD PTR [rsp],rbx
            m.w64(rsp, (rbx) as u64);
            // e1457 lea rsi,[rip+0x57f87] # 0x1393e5
            rsi = m.env.data_va(318386);
            // e145e mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // e1463 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e1465 mov r8,r13
            r8 = r13;
            // e1468 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e146b call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1470 mov esi,0x3e8
            rsi = (0x3e8u64) as u32 as u64;
            // e1475 mov rdi,r12
            rdi = r12;
            // e1478 call 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e147d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e147f je 0xe15f0
            if fl.e() { pc = 0xe15f0; continue; }
            pc = 0xe1485;
        }
        0xe1485 => {
            // block e1485
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1485 xorps xmm0,xmm0
            x0 = 0;
            // e1488 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // e148c mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // e1494 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e1498 mov rdi,r12
            rdi = r12;
            // e149b call 0xe4a2e
            { let r = { let t = m.env.code_va(0xe4a2e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e14a0 xorps xmm0,xmm0
            x0 = 0;
            // e14a3 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // e14a7 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // e14af lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // e14b6 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e14ba call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e14bf lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // e14c6 lea rdx,[rbp-0x70]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e14ca call 0xe171e
            { let r = { let t = m.env.code_va(0xe171e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e14cf lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // e14d6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e14db mov rbx,QWORD PTR [rbp-0x70]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // e14df cmp QWORD PTR [rbp-0x68],rbx
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rbx).1;
            // e14e3 je 0xe1612
            if fl.e() { pc = 0xe1612; continue; }
            pc = 0xe14e9;
        }
        0xe14e9 => {
            // block e14e9
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e14e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e14e9 xorps xmm0,xmm0
            x0 = 0;
            // e14ec movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // e14f3 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // e14fe lea rsi,[rip+0x57f0e] # 0x139413
            rsi = m.env.data_va(318432);
            // e1505 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // e150c mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e1511 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1516 lea rdx,[rbp-0xb8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // e151d lea rcx,[rbp-0xa0]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // e1524 mov rsi,rbx
            rsi = rbx;
            // e1527 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e152c mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e152e lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // e1535 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e153a test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e153c je 0xe1606
            if fl.e() { pc = 0xe1606; continue; }
            pc = 0xe1542;
        }
        0xe1542 => {
            // block e1542
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1542 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1542 lea rsi,[rip+0x57ed1] # 0x13941a
            rsi = m.env.data_va(318439);
            // e1549 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // e1550 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1555 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e1557 je 0xe164d
            if fl.e() { pc = 0xe164d; continue; }
            pc = 0xe155d;
        }
        0xe155d => {
            // block e155d
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e155d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e155d lea rsi,[rip+0x57eb9] # 0x13941d
            rsi = m.env.data_va(318442);
            // e1564 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // e156b call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1570 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e1572 jne 0xe1606
            if fl.ne() { pc = 0xe1606; continue; }
            pc = 0xe1578;
        }
        0xe1578 => {
            // block e1578
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1578 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1578 mov rbx,QWORD PTR [rbp-0x70]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // e157c mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e1580 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e1583 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e1587 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // e1591 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e1595 cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // e1599 jb 0xe1606
            if fl.b() { pc = 0xe1606; continue; }
            pc = 0xe159b;
        }
        0xe159b => {
            // block e159b
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e159b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e159b lea rsi,[rip+0x57e80] # 0x139422
            rsi = m.env.data_va(318447);
            // e15a2 lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // e15a9 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e15ae call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e15b3 add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e15b7 lea rcx,[r12+0x10]
            rcx = r12.wrapping_add(0x10);
            // e15bc lea rdx,[rbp-0xd0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // e15c3 mov rsi,rbx
            rsi = rbx;
            // e15c6 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e15cb lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // e15d2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e15d7 mov DWORD PTR [r12+0x8],0x1a
            m.w32(r12.wrapping_add(0x8), (0x1au64) as u32);
            // e15e0 mov rdi,r12
            rdi = r12;
            // e15e3 call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e15e8 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e15eb jmp 0xe16a7
            pc = 0xe16a7; continue;
        }
        0xe15f0 => {
            // block e15f0
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e15f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e15f0 mov rdi,r12
            rdi = r12;
            // e15f3 call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e15f8 mov DWORD PTR [r12+0x8],0x15
            m.w32(r12.wrapping_add(0x8), (0x15u64) as u32);
            pc = 0xe1601;
        }
        0xe1601 => {
            // block e1601
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1601 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e1604 jmp 0xe1638
            pc = 0xe1638; continue;
        }
        0xe1606 => {
            // block e1606
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1606 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // e160d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1612;
        }
        0xe1612 => {
            // block e1612
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1612 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1612 mov rdi,r12
            rdi = r12;
            // e1615 call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e161a mov DWORD PTR [r12+0x8],0x17
            m.w32(r12.wrapping_add(0x8), (0x17u64) as u32);
            // e1623 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe1626;
        }
        0xe1626 => {
            // block e1626
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1626 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1626 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e162a call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e162f lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e1633 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1638;
        }
        0xe1638 => {
            // block e1638
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e1638 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1638 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // e163b add rsp,0xb8
            { let (r, f) = add::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // e1642 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1643 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e1645 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e1647 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e1649 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e164b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e164c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe164d => {
            // block e164d
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e164d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e164d mov rdi,r12
            rdi = r12;
            // e1650 call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1655 mov rdi,QWORD PTR [r12]
            rdi = m.r64(r12) as u64;
            // e1659 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e165c mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // e1660 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e1664 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e1669 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e166e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1671 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1673 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // e1677 mov eax,0x40000000
            rax = (0x40000000u64) as u32 as u64;
            // e167c mov edx,0x80000000
            rdx = (0x80000000u64) as u32 as u64;
            // e1681 cmove edx,eax
            if fl.e() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e1684 or edx,DWORD PTR [rbp-0x2c]
            { let r = (rdx as u32 as u64) | m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e1687 mov DWORD PTR [rbp-0x2c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rdx as u32 as u64)) as u32);
            // e168a mov rdi,QWORD PTR [r12]
            rdi = m.r64(r12) as u64;
            // e168e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1691 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e1695 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e169a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e169f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e16a2 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e16a4 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe16a7;
        }
        0xe16a7 => {
            // block e16a7
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e16a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e16a7 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // e16ae call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e16b3 jmp 0xe1626
            pc = 0xe1626; continue;
        }
        0xe16b8 => {
            // block e16b8
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e16b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e16b8 mov rbx,rax
            rbx = rax;
            // e16bb jmp 0xe16f3
            pc = 0xe16f3; continue;
        }
        0xe16bd => {
            // block e16bd
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e16bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e16bd mov rbx,rax
            rbx = rax;
            // e16c0 jmp 0xe16e7
            pc = 0xe16e7; continue;
        }
        0xe16c2 => {
            // block e16c2
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e16c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e16c2 mov rbx,rax
            rbx = rax;
            // e16c5 jmp 0xe16fc
            pc = 0xe16fc; continue;
        }
        0xe16c7 => {
            // block e16c7
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e16c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e16c7 mov rbx,rax
            rbx = rax;
            // e16ca lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // e16d1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e16d6 jmp 0xe16f3
            pc = 0xe16f3; continue;
        }
        0xe16d8 => {
            // block e16d8
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e16d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e16d8 mov rbx,rax
            rbx = rax;
            // e16db lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // e16e2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe16e7;
        }
        0xe16e7 => {
            // block e16e7
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e16e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e16e7 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // e16ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe16f3;
        }
        0xe16f3 => {
            // block e16f3
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e16f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e16f3 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e16f7 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe16fc;
        }
        0xe16fc => {
            // block e16fc
            #[cfg(feature = "trace")] eprintln!("f_000e13d0 pc e16fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e16fc lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e1700 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1705 mov rdi,rbx
            rdi = rbx;
            // e1708 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e170d mov rbx,rax
            rbx = rax;
            // e1710 lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // e1717 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e171c jmp 0xe16e7
            pc = 0xe16e7; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x60623 size=818
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00060623<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x60623;
    loop {
        match pc {
        0x60623 => {
            // block 60623
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 60623 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60623 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 60624 mov rbp,rsp
            rbp = rsp;
            // 60627 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 60629 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6062b push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 6062d push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 6062f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 60630 sub rsp,0x168
            { let (r, f) = sub::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // 60637 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 6063a mov r14,rdi
            r14 = rdi;
            // 6063d mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // 60640 call 0xe7cde
            { let r = { let t = m.env.code_va(0xe7cde); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60645 mov rbx,rax
            rbx = rax;
            // 60648 mov rdi,rbx
            rdi = rbx;
            // 6064b call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60650 mov rdi,r14
            rdi = r14;
            // 60653 mov rsi,rbx
            rsi = rbx;
            // 60656 mov rdx,rax
            rdx = rax;
            // 60659 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6065e movzx eax,BYTE PTR [r14]
            rax = (m.r8(r14) as u64) as u32 as u64;
            // 60662 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 60664 jne 0x6066b
            if fl.ne() { pc = 0x6066b; continue; }
            pc = 0x60666;
        }
        0x60666 => {
            // block 60666
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 60666 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60666 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 60669 jmp 0x6066f
            pc = 0x6066f; continue;
        }
        0x6066b => {
            // block 6066b
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 6066b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6066b mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            pc = 0x6066f;
        }
        0x6066f => {
            // block 6066f
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 6066f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6066f test rax,rax
            fl = logic::<64>(rax & rax);
            // 60672 jne 0x608ad
            if fl.ne() { pc = 0x608ad; continue; }
            pc = 0x60678;
        }
        0x60678 => {
            // block 60678
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 60678 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60678 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 6067c mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 6067f call 0x683dc
            { let r = { let t = m.env.code_va(0x683dc); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60684 test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // 60688 jne 0x60695
            if fl.ne() { pc = 0x60695; continue; }
            pc = 0x6068a;
        }
        0x6068a => {
            // block 6068a
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 6068a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6068a mov BYTE PTR [r14+0x1],0x0
            m.w8(r14.wrapping_add(0x1), (0x0u64) as u8);
            // 6068f mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // 60693 jmp 0x606a4
            pc = 0x606a4; continue;
        }
        0x60695 => {
            // block 60695
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 60695 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60695 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 60699 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 6069c mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0x606a4;
        }
        0x606a4 => {
            // block 606a4
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 606a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 606a4 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 606a6 mov rdi,r14
            rdi = r14;
            // 606a9 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 606ae mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 606b2 mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 606b6 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 606ba mov rcx,QWORD PTR [rbp-0x58]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 606be mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // 606c2 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 606c5 xorps xmm0,xmm0
            x0 = 0;
            // 606c8 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 606cc mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 606d4 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 606d8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 606dd movzx eax,BYTE PTR [r14]
            rax = (m.r8(r14) as u64) as u32 as u64;
            // 606e1 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 606e3 jne 0x606ea
            if fl.ne() { pc = 0x606ea; continue; }
            pc = 0x606e5;
        }
        0x606e5 => {
            // block 606e5
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 606e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 606e5 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 606e8 jmp 0x606ee
            pc = 0x606ee; continue;
        }
        0x606ea => {
            // block 606ea
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 606ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 606ea mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            pc = 0x606ee;
        }
        0x606ee => {
            // block 606ee
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 606ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 606ee test rax,rax
            fl = logic::<64>(rax & rax);
            // 606f1 jne 0x608ad
            if fl.ne() { pc = 0x608ad; continue; }
            pc = 0x606f7;
        }
        0x606f7 => {
            // block 606f7
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 606f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 606f7 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 606fe lea r13,[rbp-0x160]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 60705 mov rax,QWORD PTR [rip+0xdfac4] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 6070c lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 60710 mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // 60717 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 6071b mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 60722 mov rsi,r13
            rsi = r13;
            // 60725 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6072a mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 60732 mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // 60739 mov rbx,QWORD PTR [rip+0xdfc80] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // 60740 lea rax,[rbx+0x18]
            rax = rbx.wrapping_add(0x18);
            // 60744 mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 6074b mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 60752 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // 60756 mov QWORD PTR [rbp-0xf8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rbx) as u64);
            // 6075d mov rdi,r13
            rdi = r13;
            // 60760 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60765 mov r12,QWORD PTR [rip+0xdfc3c] # 0x1403a8
            r12 = m.r64(m.env.data_va(346997)) as u64;
            // 6076c add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 60770 mov QWORD PTR [rbp-0x160],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r12) as u64);
            // 60777 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 60782 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 6078d mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 60798 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 607a3 mov DWORD PTR [rbp-0x100],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x10u64) as u32);
            // 607ad xorps xmm0,xmm0
            x0 = 0;
            // 607b0 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 607b4 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 607bc lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 607c0 mov rdi,r13
            rdi = r13;
            // 607c3 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 607c8 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 607cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 607d1 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 607d8 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 607db call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 607e0 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 607e7 mov rsi,r13
            rsi = r13;
            // 607ea call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 607ef test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // 607f3 jne 0x60800
            if fl.ne() { pc = 0x60800; continue; }
            pc = 0x607f5;
        }
        0x607f5 => {
            // block 607f5
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 607f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 607f5 mov BYTE PTR [r14+0x1],0x0
            m.w8(r14.wrapping_add(0x1), (0x0u64) as u8);
            // 607fa mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // 607fe jmp 0x6080f
            pc = 0x6080f; continue;
        }
        0x60800 => {
            // block 60800
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 60800 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60800 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 60804 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 60807 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0x6080f;
        }
        0x6080f => {
            // block 6080f
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 6080f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6080f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 60811 mov rdi,r14
            rdi = r14;
            // 60814 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60819 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 60820 mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 60824 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 6082b mov rcx,QWORD PTR [rbp-0x178]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 60832 mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // 60836 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 60839 xorps xmm0,xmm0
            x0 = 0;
            // 6083c movaps XMMWORD PTR [rbp-0x180],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe80), x0);
            // 60843 mov QWORD PTR [rbp-0x170],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (0x0u64) as u64);
            // 6084e lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 60855 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6085a mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 60861 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 60868 mov QWORD PTR [rbp-0xf8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rbx) as u64);
            // 6086f mov QWORD PTR [rbp-0x160],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r12) as u64);
            // 60876 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 6087d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60882 mov rdi,r13
            rdi = r13;
            // 60885 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6088a mov rsi,QWORD PTR [rip+0xdf977] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 60891 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 60895 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 6089c call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 608a1 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 608a8 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x608ad;
        }
        0x608ad => {
            // block 608ad
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 608ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 608ad mov rax,r14
            rax = r14;
            // 608b0 add rsp,0x168
            { let (r, f) = add::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // 608b7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 608b8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 608ba pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 608bc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 608be pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 608c0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 608c1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x608c2 => {
            // block 608c2
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 608c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 608c2 mov r15,rax
            r15 = rax;
            // 608c5 jmp 0x6093d
            pc = 0x6093d; continue;
        }
        0x608c7 => {
            // block 608c7
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 608c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 608c7 mov rdi,rax
            rdi = rax;
            // 608ca call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 608cf mov r15,rax
            r15 = rax;
            // 608d2 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 608d9 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 608e0 mov QWORD PTR [rbp-0xf8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rbx) as u64);
            // 608e7 mov QWORD PTR [rbp-0x160],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r12) as u64);
            // 608ee jmp 0x60906
            pc = 0x60906; continue;
        }
        0x608f0 => {
            // block 608f0
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 608f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 608f0 mov r15,rax
            r15 = rax;
            // 608f3 jmp 0x60931
            pc = 0x60931; continue;
        }
        0x608f5 => {
            // block 608f5
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 608f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 608f5 mov r15,rax
            r15 = rax;
            // 608f8 jmp 0x6091a
            pc = 0x6091a; continue;
        }
        0x608fa => {
            // block 608fa
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 608fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 608fa mov r15,rax
            r15 = rax;
            // 608fd lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 60901 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x60906;
        }
        0x60906 => {
            // block 60906
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 60906 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60906 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 6090d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60912 mov rdi,r13
            rdi = r13;
            // 60915 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6091a;
        }
        0x6091a => {
            // block 6091a
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 6091a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6091a mov rsi,QWORD PTR [rip+0xdf8e7] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 60921 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 60925 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 6092c call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x60931;
        }
        0x60931 => {
            // block 60931
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 60931 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60931 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 60938 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6093d;
        }
        0x6093d => {
            // block 6093d
            #[cfg(feature = "trace")] eprintln!("f_00060623 pc 6093d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6093d mov rdi,r14
            rdi = r14;
            // 60940 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60945 mov rdi,r15
            rdi = r15;
            // 60948 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6094d mov rdi,rax
            rdi = rax;
            // 60950 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1378a size=806
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001378a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1378a;
    loop {
        match pc {
        0x1378a => {
            // block 1378a
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 1378a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1378a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1378b mov rbp,rsp
            rbp = rsp;
            // 1378e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 13790 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 13792 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 13794 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 13796 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 13797 sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 1379e mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 137a1 mov QWORD PTR [rbp-0x168],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rsi) as u64);
            // 137a8 mov r15,rdi
            r15 = rdi;
            // 137ab mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 137ae call QWORD PTR [rax+0x60]
            { let r = { let t = m.r64(rax.wrapping_add(0x60)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 137b1 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 137b4 cmp BYTE PTR [r15+0x2e8],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x2e8)) as u64, 0x0u64).1;
            // 137bc mov r12d,r14d
            r12 = ((r14 as u32 as u64)) as u32 as u64;
            // 137bf mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 137c4 je 0x13a1a
            if fl.e() { pc = 0x13a1a; continue; }
            pc = 0x137ca;
        }
        0x137ca => {
            // block 137ca
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 137ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 137ca lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // 137ce xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 137d1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 137d3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 137d5 mov rdi,rbx
            rdi = rbx;
            // 137d8 call 0xec044
            { let r = { let t = m.env.stub_va(0x811); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x137dd;
        }
        0x137dd => {
            // block 137dd
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 137dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 137dd cmp DWORD PTR [r15+0x2f0],r13d
            fl = sub::<32>(m.r32(r15.wrapping_add(0x2f0)) as u64, (r13 as u32 as u64)).1;
            // 137e4 jae 0x1381e
            if fl.ae() { pc = 0x1381e; continue; }
            pc = 0x137e6;
        }
        0x137e6 => {
            // block 137e6
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 137e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 137e6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 137e9 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 137ed test BYTE PTR [rax+rbx*1+0x20],0x2
            fl = logic::<8>(m.r8(rax.wrapping_add(rbx).wrapping_add(0x20)) as u64 & 0x2u64);
            // 137f2 jne 0x13826
            if fl.ne() { pc = 0x13826; continue; }
            pc = 0x137f4;
        }
        0x137f4 => {
            // block 137f4
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 137f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 137f4 mov rdi,rbx
            rdi = rbx;
            // 137f7 call 0xec03e
            { let r = { let t = m.env.stub_va(0x80b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 137fc mov rcx,QWORD PTR [rbp-0x168]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 13803 mov BYTE PTR [rcx+r14*1],al
            m.w8(rcx.wrapping_add(r14), ((rax as u8 as u64)) as u8);
            // 13807 inc DWORD PTR [r15+0x2ec]
            { let (r, f) = inc::<32>(m.r32(r15.wrapping_add(0x2ec)) as u64, fl); m.w32(r15.wrapping_add(0x2ec), (r) as u32); fl = f; }
            // 1380e inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 13811 cmp r12d,r14d
            fl = sub::<32>((r12 as u32 as u64), (r14 as u32 as u64)).1;
            // 13814 jne 0x137dd
            if fl.ne() { pc = 0x137dd; continue; }
            pc = 0x13816;
        }
        0x13816 => {
            // block 13816
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13816 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13816 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 13819 jmp 0x13a1a
            pc = 0x13a1a; continue;
        }
        0x1381e => {
            // block 1381e
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 1381e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1381e mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 13821 jmp 0x13a1a
            pc = 0x13a1a; continue;
        }
        0x13826 => {
            // block 13826
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13826 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13826 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 1382d lea r14,[rbp-0x140]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 13834 mov rax,QWORD PTR [rip+0x12c995] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 1383b lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 1383f mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // 13846 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 1384a mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 13851 mov rsi,r14
            rsi = r14;
            // 13854 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13859 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 13861 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // 13868 mov r13,QWORD PTR [rip+0x12cb51] # 0x1403c0
            r13 = m.r64(m.env.data_va(347021)) as u64;
            // 1386f lea rbx,[r13+0x18]
            rbx = r13.wrapping_add(0x18);
            // 13873 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 1387a add r13,0x40
            { let (r, f) = add::<64>(r13, 0x40u64);
              r13 = r;
              fl = f; }
            // 1387e mov QWORD PTR [rbp-0xd8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r13) as u64);
            // 13885 mov rdi,r14
            rdi = r14;
            // 13888 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1388d mov r12,QWORD PTR [rip+0x12cb14] # 0x1403a8
            r12 = m.r64(m.env.data_va(346997)) as u64;
            // 13894 add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 13898 mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // 1389f mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // 138aa mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 138b5 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 138c0 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 138cb mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // 138d5 xorps xmm0,xmm0
            x0 = 0;
            // 138d8 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 138dc mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 138e4 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 138e8 mov rdi,r14
            rdi = r14;
            // 138eb call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 138f0 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // 138f7 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 138fb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13900 lea rsi,[rip+0x116818] # 0x12a11f
            rsi = m.env.data_va(256236);
            // 13907 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 1390e mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 13913 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13918 mov esi,DWORD PTR [r15+0x2ec]
            rsi = (m.r32(r15.wrapping_add(0x2ec)) as u64) as u32 as u64;
            // 1391f mov rdi,rax
            rdi = rax;
            // 13922 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13927 lea rsi,[rip+0x116804] # 0x12a132
            rsi = m.env.data_va(256255);
            // 1392e mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 13933 mov rdi,rax
            rdi = rax;
            // 13936 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1393b lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 13942 mov rsi,r14
            rsi = r14;
            // 13945 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1394a lea rbx,[r15+0x2c8]
            rbx = r15.wrapping_add(0x2c8);
            // 13951 test BYTE PTR [r15+0x2c8],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x2c8)) as u64 & 0x1u64);
            // 13959 jne 0x13962
            if fl.ne() { pc = 0x13962; continue; }
            pc = 0x1395b;
        }
        0x1395b => {
            // block 1395b
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 1395b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1395b mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 13960 jmp 0x13977
            pc = 0x13977; continue;
        }
        0x13962 => {
            // block 13962
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13962 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13962 mov rax,QWORD PTR [r15+0x2d8]
            rax = m.r64(r15.wrapping_add(0x2d8)) as u64;
            // 13969 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 1396c mov QWORD PTR [r15+0x2d0],0x0
            m.w64(r15.wrapping_add(0x2d0), (0x0u64) as u64);
            pc = 0x13977;
        }
        0x13977 => {
            // block 13977
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13977 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13977 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 13979 mov rdi,rbx
            rdi = rbx;
            // 1397c call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13981 mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 13988 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 1398c mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 13993 mov rcx,QWORD PTR [rbp-0x158]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 1399a mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 1399e mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 139a1 xorps xmm0,xmm0
            x0 = 0;
            // 139a4 movaps XMMWORD PTR [rbp-0x160],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fea0), x0);
            // 139ab mov QWORD PTR [rbp-0x150],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0x0u64) as u64);
            // 139b6 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 139bd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 139c2 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 139c9 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 139d0 mov QWORD PTR [rbp-0xd8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r13) as u64);
            // 139d7 mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // 139de lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 139e5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 139ea mov rdi,r14
            rdi = r14;
            // 139ed call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 139f2 mov rsi,QWORD PTR [rip+0x12c80f] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 139f9 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 139fd lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 13a04 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13a09 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 13a10 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13a15 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            pc = 0x13a1a;
        }
        0x13a1a => {
            // block 13a1a
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13a1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13a1a add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 13a21 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 13a22 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 13a24 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 13a26 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 13a28 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 13a2a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 13a2b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x13a2c => {
            // block 13a2c
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13a2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13a2c mov rbx,r14
            rbx = r14;
            // 13a2f mov r14,rax
            r14 = rax;
            // 13a32 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 13a39 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 13a40 mov QWORD PTR [rbp-0xd8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r13) as u64);
            // 13a47 mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // 13a4e jmp 0x13a69
            pc = 0x13a69; continue;
        }
        0x13a50 => {
            // block 13a50
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13a50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13a50 mov r14,rax
            r14 = rax;
            // 13a53 jmp 0x13a94
            pc = 0x13a94; continue;
        }
        0x13a55 => {
            // block 13a55
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13a55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13a55 mov r14,rax
            r14 = rax;
            // 13a58 jmp 0x13a7d
            pc = 0x13a7d; continue;
        }
        0x13a5a => {
            // block 13a5a
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13a5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13a5a mov rbx,r14
            rbx = r14;
            // 13a5d mov r14,rax
            r14 = rax;
            // 13a60 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 13a64 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x13a69;
        }
        0x13a69 => {
            // block 13a69
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13a69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13a69 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 13a70 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13a75 mov rdi,rbx
            rdi = rbx;
            // 13a78 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x13a7d;
        }
        0x13a7d => {
            // block 13a7d
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13a7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13a7d mov rsi,QWORD PTR [rip+0x12c784] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 13a84 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 13a88 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 13a8f call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x13a94;
        }
        0x13a94 => {
            // block 13a94
            #[cfg(feature = "trace")] eprintln!("f_0001378a pc 13a94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13a94 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 13a9b call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13aa0 mov rdi,r14
            rdi = r14;
            // 13aa3 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13aa8 mov rdi,rax
            rdi = rax;
            // 13aab call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63f31 size=793
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00063f31<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x63f31;
    loop {
        match pc {
        0x63f31 => {
            // block 63f31
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 63f31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63f31 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63f32 mov rbp,rsp
            rbp = rsp;
            // 63f35 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 63f37 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 63f39 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 63f3b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 63f3d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 63f3e sub rsp,0x158
            { let (r, f) = sub::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // 63f45 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 63f47 mov r15,rdi
            r15 = rdi;
            // 63f4a lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 63f51 lea r13,[rbp-0x140]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 63f58 mov rax,QWORD PTR [rip+0xdc271] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 63f5f lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 63f63 mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // 63f6a add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 63f6e mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 63f75 mov rsi,r13
            rsi = r13;
            // 63f78 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63f7d mov DWORD PTR [rbp-0x174],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe8c), ((rbx as u32 as u64)) as u32);
            // 63f83 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 63f8b mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // 63f92 mov r12,QWORD PTR [rip+0xdc427] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 63f99 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 63f9e mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 63fa5 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 63fac add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 63fb0 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // 63fb7 mov rdi,r13
            rdi = r13;
            // 63fba call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63fbf mov rbx,QWORD PTR [rip+0xdc3e2] # 0x1403a8
            rbx = m.r64(m.env.data_va(346997)) as u64;
            // 63fc6 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 63fca mov QWORD PTR [rbp-0x140],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rbx) as u64);
            // 63fd1 lea r14,[rbp-0x100]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 63fd8 mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // 63fe3 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 63fee mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 63ff9 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 64004 mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // 6400e xorps xmm0,xmm0
            x0 = 0;
            // 64011 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 64015 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 6401d lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 64021 mov rdi,r13
            rdi = r13;
            // 64024 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64029 mov QWORD PTR [rbp-0x170],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rbx) as u64);
            // 64030 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 64034 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64039 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 64040 mov esi,0xe
            rsi = (0xeu64) as u32 as u64;
            // 64045 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6404a lea rsi,[rip+0xcd94b] # 0x13199c
            rsi = m.env.data_va(287081);
            // 64051 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 64056 mov rdi,rax
            rdi = rax;
            // 64059 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6405e xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 64060 mov rdi,rax
            rdi = rax;
            // 64063 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64068 lea rsi,[rip+0xcd92d] # 0x13199c
            rsi = m.env.data_va(287081);
            // 6406f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 64074 mov rdi,rax
            rdi = rax;
            // 64077 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6407c mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 64081 mov rdi,rax
            rdi = rax;
            // 64084 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64089 lea rsi,[rip+0xd4c38] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 64090 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 64097 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 64099 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6409e mov r14,r15
            r14 = r15;
            // 640a1 movzx r15d,BYTE PTR [rbp-0x160]
            r15 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64) as u32 as u64;
            // 640a9 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // 640ad jne 0x640b4
            if fl.ne() { pc = 0x640b4; continue; }
            pc = 0x640af;
        }
        0x640af => {
            // block 640af
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 640af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 640af shr r15,1
            { let (r, f) = shr::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 640b2 jmp 0x640bb
            pc = 0x640bb; continue;
        }
        0x640b4 => {
            // block 640b4
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 640b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 640b4 mov r15,QWORD PTR [rbp-0x158]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            pc = 0x640bb;
        }
        0x640bb => {
            // block 640bb
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 640bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 640bb lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 640c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 640c7 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 640ca je 0x64102
            if fl.e() { pc = 0x64102; continue; }
            pc = 0x640cc;
        }
        0x640cc => {
            // block 640cc
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 640cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 640cc lea rsi,[rip+0xc61c1] # 0x12a294
            rsi = m.env.data_va(256609);
            // 640d3 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 640da mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 640df call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 640e4 lea rsi,[rip+0xd4bdd] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 640eb xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 640ed mov rdi,rax
            rdi = rax;
            // 640f0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 640f5 mov esi,0x28
            rsi = (0x28u64) as u32 as u64;
            // 640fa mov rdi,rax
            rdi = rax;
            // 640fd call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x64102;
        }
        0x64102 => {
            // block 64102
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 64102 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64102 mov eax,DWORD PTR [rbp-0x174]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe8c)) as u64) as u32 as u64;
            // 64108 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6410a je 0x64150
            if fl.e() { pc = 0x64150; continue; }
            pc = 0x6410c;
        }
        0x6410c => {
            // block 6410c
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 6410c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6410c lea rsi,[rip+0xcd88b] # 0x13199e
            rsi = m.env.data_va(287083);
            // 64113 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 6411a mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 6411f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64124 lea rsi,[rip+0xcd885] # 0x1319b0
            rsi = m.env.data_va(287101);
            // 6412b lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 64132 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 64137 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6413c lea rsi,[rip+0xcd878] # 0x1319bb
            rsi = m.env.data_va(287112);
            // 64143 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 64148 mov rdi,rax
            rdi = rax;
            // 6414b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x64150;
        }
        0x64150 => {
            // block 64150
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 64150 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64150 mov rdi,r14
            rdi = r14;
            // 64153 mov rsi,r13
            rsi = r13;
            // 64156 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6415b mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 64162 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 64169 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // 64170 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 64177 mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            // 6417e lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 64185 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6418a mov rdi,r13
            rdi = r13;
            // 6418d call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64192 mov rsi,QWORD PTR [rip+0xdc06f] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 64199 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 6419d lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 641a4 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 641a9 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 641b0 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 641b5 mov rax,r14
            rax = r14;
            // 641b8 add rsp,0x158
            { let (r, f) = add::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // 641bf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 641c0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 641c2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 641c4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 641c6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 641c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 641c9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x641ca => {
            // block 641ca
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 641ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 641ca mov r15,rax
            r15 = rax;
            // 641cd mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 641d4 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 641db mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // 641e2 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 641e9 mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            // 641f0 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            pc = 0x641f7;
        }
        0x641f7 => {
            // block 641f7
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 641f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 641f7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 641fc mov rdi,r13
            rdi = r13;
            // 641ff call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x64204;
        }
        0x64204 => {
            // block 64204
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 64204 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64204 mov rsi,QWORD PTR [rip+0xdbffd] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 6420b add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 6420f lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 64216 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6421b;
        }
        0x6421b => {
            // block 6421b
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 6421b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6421b lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 64222 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64227 mov rdi,r15
            rdi = r15;
            // 6422a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6422f mov r15,rax
            r15 = rax;
            // 64232 jmp 0x6421b
            pc = 0x6421b; continue;
        }
        0x64234 => {
            // block 64234
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 64234 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64234 mov r15,rax
            r15 = rax;
            // 64237 jmp 0x64204
            pc = 0x64204; continue;
        }
        0x64239 => {
            // block 64239
            #[cfg(feature = "trace")] eprintln!("f_00063f31 pc 64239 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64239 mov r15,rax
            r15 = rax;
            // 6423c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 64240 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64245 mov rdi,r14
            rdi = r14;
            // 64248 jmp 0x641f7
            pc = 0x641f7; continue;
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
            if fl.e() { pc = 0x123d6; continue; }
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
            if fl.le() { pc = 0x12384; continue; }
            pc = 0x12140;
        }
        0x12140 => {
            // block 12140
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12140 cmp r13,QWORD PTR [rbp-0x68]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 12144 jle 0x12384
            if fl.le() { pc = 0x12384; continue; }
            pc = 0x1214a;
        }
        0x1214a => {
            // block 1214a
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1214a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1214a test r13,r13
            fl = logic::<64>(r13 & r13);
            // 1214d je 0x123d6
            if fl.e() { pc = 0x123d6; continue; }
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
            if fl.l() { pc = 0x12170; continue; }
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
            if fl.ne() { pc = 0x1215d; continue; }
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
            if fl.ge() { pc = 0x121a8; continue; }
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
            if fl.e() { pc = 0x123aa; continue; }
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
            if fl.e() { pc = 0x1220e; continue; }
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
            if fl.l() { pc = 0x121ec; continue; }
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
            if fl.e() { pc = 0x1224e; continue; }
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
            if fl.ge() { pc = 0x1222c; continue; }
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
            if fl.e() { pc = 0x1229f; continue; }
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
            if fl.e() { pc = 0x1229f; continue; }
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
            { let r = { let t = m.env.stub_va(0x8b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ge() { pc = 0x12323; continue; }
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
            { rsp = rsp.wrapping_sub(8); m.w64(rsp, 0x100122fd); let r = f_000120e8_rec(m, [rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15], [x0, 0, 0, 0, 0, 0, 0, 0]); rsp = rsp.wrapping_add(8); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0x12136; continue; }
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
            { rsp = rsp.wrapping_sub(8); m.w64(rsp, 0x10012362); let r = f_000120e8_rec(m, [rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15], [x0, 0, 0, 0, 0, 0, 0, 0]); rsp = rsp.wrapping_add(8); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0x1212c; continue; }
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
            { let r = { let t = m.env.stub_va(0xb5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
        0x123aa => {
            // block 123aa
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 123aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 123aa lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 123ae mov rsi,rbx
            rsi = rbx;
            // 123b1 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 123b6 mov rdi,rbx
            rdi = rbx;
            // 123b9 mov rsi,r14
            rsi = r14;
            // 123bc call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 123c1 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 123c5 mov rdi,r14
            rdi = r14;
            // 123c8 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 123cd lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 123d1 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xe05fa); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 123f4 mov rdi,rbx
            rdi = rbx;
            // 123f7 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa5a20 size=754
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a5a20<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xa5a20;
    loop {
        match pc {
        0xa5a20 => {
            // block a5a20
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5a20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5a20 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a5a21 mov rbp,rsp
            rbp = rsp;
            // a5a24 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a5a26 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a5a28 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // a5a2a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // a5a2c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a5a2d sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // a5a34 mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // a5a37 mov r14,rdi
            r14 = rdi;
            // a5a3a lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a5a41 lea r15,[rbp-0x150]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a5a48 mov rax,QWORD PTR [rip+0x9a781] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // a5a4f lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // a5a53 mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // a5a5a add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // a5a5e mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a5a65 mov rsi,r15
            rsi = r15;
            // a5a68 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5a6d mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // a5a75 mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // a5a7c mov rbx,QWORD PTR [rip+0x9a93d] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // a5a83 lea r12,[rbx+0x18]
            r12 = rbx.wrapping_add(0x18);
            // a5a87 mov QWORD PTR [rbp-0x158],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r12) as u64);
            // a5a8e add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // a5a92 mov QWORD PTR [rbp-0xe8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rbx) as u64);
            // a5a99 mov rdi,r15
            rdi = r15;
            // a5a9c call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5aa1 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // a5aa8 mov QWORD PTR [rbp-0x160],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r12) as u64);
            // a5aaf mov rbx,QWORD PTR [rip+0x9a8f2] # 0x1403a8
            rbx = m.r64(m.env.data_va(346997)) as u64;
            // a5ab6 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // a5aba mov QWORD PTR [rbp-0x150],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rbx) as u64);
            // a5ac1 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // a5acc mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // a5ad7 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // a5ae2 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // a5aed mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // a5af7 xorps xmm0,xmm0
            x0 = 0;
            // a5afa movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // a5afe mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // a5b06 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a5b0a mov rdi,r15
            rdi = r15;
            // a5b0d call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b12 mov QWORD PTR [rbp-0x170],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rbx) as u64);
            // a5b19 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a5b1d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b22 lea rsi,[rip+0x8e506] # 0x13402f
            rsi = m.env.data_va(296956);
            // a5b29 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a5b30 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // a5b35 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b3a test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // a5b3d sets cl
            rcx = (rcx & !0xff) | (((fl.s() as u64)) as u8 as u64);
            // a5b40 lea rdx,[rip+0x8e4f7] # 0x13403e
            rdx = m.env.data_va(296971);
            // a5b47 lea rsi,[rip+0x8e4f5] # 0x134043
            rsi = m.env.data_va(296976);
            // a5b4e cmovs rsi,rdx
            if fl.s() { rsi = rdx; }
            // a5b52 movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // a5b55 add rdx,0x3
            { let (r, f) = add::<64>(rdx, 0x3u64);
              rdx = r;
              fl = f; }
            // a5b59 mov rdi,rax
            rdi = rax;
            // a5b5c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b61 mov r12,rax
            r12 = rax;
            // a5b64 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // a5b68 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5b6c add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // a5b6f lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a5b73 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b78 mov ebx,r13d
            rbx = ((r13 as u32 as u64)) as u32 as u64;
            // a5b7b mov r13,r14
            r13 = r14;
            // a5b7e mov rsi,QWORD PTR [rip+0x9a60b] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a5b85 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a5b89 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b8e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5b91 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a5b95 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a5b9a mov rdi,rax
            rdi = rax;
            // a5b9d call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b9f mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a5ba2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a5ba6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bab movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a5baf mov rdi,r12
            rdi = r12;
            // a5bb2 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bb7 mov rdi,r12
            rdi = r12;
            // a5bba call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bbf lea rsi,[rip+0x8e481] # 0x134047
            rsi = m.env.data_va(296980);
            // a5bc6 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // a5bcb mov rdi,r12
            rdi = r12;
            // a5bce call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bd3 and ebx,0x40000000
            { let r = (rbx as u32 as u64) & 0x40000000u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // a5bd9 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // a5bdc lea rdx,[rip+0x8e46c] # 0x13404f
            rdx = m.env.data_va(296988);
            // a5be3 lea rsi,[rip+0x8e470] # 0x13405a
            rsi = m.env.data_va(296999);
            // a5bea cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // a5bee movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // a5bf1 lea rdx,[rcx+rcx*4+0x5]
            rdx = rcx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x5);
            // a5bf6 mov rdi,rax
            rdi = rax;
            // a5bf9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bfe mov rdi,r13
            rdi = r13;
            // a5c01 mov rsi,r15
            rsi = r15;
            // a5c04 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c09 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // a5c10 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a5c17 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a5c1e mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a5c25 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a5c2c mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a5c33 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a5c3a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c3f mov rdi,r15
            rdi = r15;
            // a5c42 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c47 mov rsi,QWORD PTR [rip+0x9a5ba] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a5c4e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a5c52 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a5c59 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c5e lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a5c65 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c6a mov rax,r13
            rax = r13;
            // a5c6d add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // a5c74 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // a5c75 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // a5c77 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // a5c79 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // a5c7b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // a5c7d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a5c7e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xa5c7f => {
            // block a5c7f
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5c7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5c7f mov r14,rax
            r14 = rax;
            // a5c82 jmp 0xa5c90
            pc = 0xa5c90; continue;
        }
        0xa5c84 => {
            // block a5c84
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5c84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5c84 mov r14,rax
            r14 = rax;
            // a5c87 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a5c8b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa5c90;
        }
        0xa5c90 => {
            // block a5c90
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5c90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5c90 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // a5c97 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a5c9e mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a5ca5 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a5cac mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a5cb3 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            pc = 0xa5cba;
        }
        0xa5cba => {
            // block a5cba
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5cba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5cba lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a5cc1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5cc6 mov rdi,r15
            rdi = r15;
            // a5cc9 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa5cce;
        }
        0xa5cce => {
            // block a5cce
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5cce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5cce mov rsi,QWORD PTR [rip+0x9a533] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a5cd5 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a5cd9 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a5ce0 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa5ce5;
        }
        0xa5ce5 => {
            // block a5ce5
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5ce5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5ce5 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a5cec call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5cf1 mov rdi,r14
            rdi = r14;
            // a5cf4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5cf9 mov r14,rax
            r14 = rax;
            // a5cfc jmp 0xa5ce5
            pc = 0xa5ce5; continue;
        }
        0xa5cfe => {
            // block a5cfe
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5cfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5cfe mov r14,rax
            r14 = rax;
            // a5d01 jmp 0xa5cce
            pc = 0xa5cce; continue;
        }
        0xa5d03 => {
            // block a5d03
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5d03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5d03 mov r14,rax
            r14 = rax;
            // a5d06 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a5d0a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5d0f jmp 0xa5cba
            pc = 0xa5cba; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1aea size=736
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e1aea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe1aea;
    loop {
        match pc {
        0xe1aea => {
            // block e1aea
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1aea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1aea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1aeb mov rbp,rsp
            rbp = rsp;
            // e1aee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e1af0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1af2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1af3 sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // e1afa mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e1afd mov r15,rdi
            r15 = rdi;
            // e1b00 call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1b05 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // e1b07 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e1b09 je 0xe1cee
            if fl.e() { pc = 0xe1cee; continue; }
            pc = 0xe1b0f;
        }
        0xe1b0f => {
            // block e1b0f
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1b0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1b0f mov rdi,r15
            rdi = r15;
            // e1b12 call 0xe494e
            { let r = { let t = m.env.code_va(0xe494e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1b17 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e1b19 je 0xe1cba
            if fl.e() { pc = 0xe1cba; continue; }
            pc = 0xe1b1f;
        }
        0xe1b1f => {
            // block e1b1f
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1b1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1b1f lea rdi,[r15+0x34]
            rdi = r15.wrapping_add(0x34);
            // e1b23 lea rsi,[rip+0x578fe] # 0x139428
            rsi = m.env.data_va(318453);
            // e1b2a mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // e1b2f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e1b31 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e1b34 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1b39 mov esi,0x3e8
            rsi = (0x3e8u64) as u32 as u64;
            // e1b3e mov rdi,r15
            rdi = r15;
            // e1b41 call 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1b46 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e1b48 je 0xe1caa
            if fl.e() { pc = 0xe1caa; continue; }
            pc = 0xe1b4e;
        }
        0xe1b4e => {
            // block e1b4e
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1b4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1b4e xorps xmm0,xmm0
            x0 = 0;
            // e1b51 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // e1b55 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // e1b5d lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e1b61 mov rdi,r15
            rdi = r15;
            // e1b64 call 0xe4a2e
            { let r = { let t = m.env.code_va(0xe4a2e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1b69 xorps xmm0,xmm0
            x0 = 0;
            // e1b6c movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // e1b70 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // e1b78 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e1b7c lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e1b80 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1b85 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e1b89 lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e1b8d call 0xe171e
            { let r = { let t = m.env.code_va(0xe171e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1b92 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e1b96 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1b9b mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e1b9f cmp QWORD PTR [rbp-0x58],rbx
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64, rbx).1;
            // e1ba3 je 0xe1cca
            if fl.e() { pc = 0xe1cca; continue; }
            pc = 0xe1ba9;
        }
        0xe1ba9 => {
            // block e1ba9
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1ba9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1ba9 xorps xmm0,xmm0
            x0 = 0;
            // e1bac movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // e1bb3 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // e1bbb lea rsi,[rip+0x57851] # 0x139413
            rsi = m.env.data_va(318432);
            // e1bc2 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e1bc9 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e1bce call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1bd3 lea rdx,[rbp-0xa8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e1bda lea rcx,[rbp-0x90]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e1be1 mov rsi,rbx
            rsi = rbx;
            // e1be4 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1be9 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e1beb lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e1bf2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1bf7 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e1bf9 je 0xe1cbe
            if fl.e() { pc = 0xe1cbe; continue; }
            pc = 0xe1bff;
        }
        0xe1bff => {
            // block e1bff
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1bff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1bff lea rsi,[rip+0x57814] # 0x13941a
            rsi = m.env.data_va(318439);
            // e1c06 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e1c0d call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1c12 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e1c14 je 0xe1cfe
            if fl.e() { pc = 0xe1cfe; continue; }
            pc = 0xe1c1a;
        }
        0xe1c1a => {
            // block e1c1a
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1c1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1c1a lea rsi,[rip+0x577fc] # 0x13941d
            rsi = m.env.data_va(318442);
            // e1c21 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e1c28 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1c2d test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e1c2f jne 0xe1cbe
            if fl.ne() { pc = 0xe1cbe; continue; }
            pc = 0xe1c35;
        }
        0xe1c35 => {
            // block e1c35
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1c35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1c35 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e1c39 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e1c3d sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e1c40 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e1c44 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // e1c4e imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e1c52 cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // e1c56 jb 0xe1cbe
            if fl.b() { pc = 0xe1cbe; continue; }
            pc = 0xe1c58;
        }
        0xe1c58 => {
            // block e1c58
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1c58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1c58 lea rsi,[rip+0x577c3] # 0x139422
            rsi = m.env.data_va(318447);
            // e1c5f lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e1c66 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e1c6b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1c70 add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e1c74 lea rcx,[r15+0x10]
            rcx = r15.wrapping_add(0x10);
            // e1c78 lea rdx,[rbp-0xc0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e1c7f mov rsi,rbx
            rsi = rbx;
            // e1c82 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1c87 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e1c8e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1c93 mov DWORD PTR [r15+0x8],0x1a
            m.w32(r15.wrapping_add(0x8), (0x1au64) as u32);
            // e1c9b mov rdi,r15
            rdi = r15;
            // e1c9e call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1ca3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1ca5 jmp 0xe1d55
            pc = 0xe1d55; continue;
        }
        0xe1caa => {
            // block e1caa
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1caa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1caa mov rdi,r15
            rdi = r15;
            // e1cad call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1cb2 mov DWORD PTR [r15+0x8],0x15
            m.w32(r15.wrapping_add(0x8), (0x15u64) as u32);
            pc = 0xe1cba;
        }
        0xe1cba => {
            // block e1cba
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1cba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1cba xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1cbc jmp 0xe1cee
            pc = 0xe1cee; continue;
        }
        0xe1cbe => {
            // block e1cbe
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1cbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1cbe lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e1cc5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1cca;
        }
        0xe1cca => {
            // block e1cca
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1cca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1cca mov rdi,r15
            rdi = r15;
            // e1ccd call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1cd2 mov DWORD PTR [r15+0x8],0x17
            m.w32(r15.wrapping_add(0x8), (0x17u64) as u32);
            // e1cda xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe1cdc;
        }
        0xe1cdc => {
            // block e1cdc
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1cdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1cdc lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e1ce0 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1ce5 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e1ce9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1cee;
        }
        0xe1cee => {
            // block e1cee
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1cee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1cee mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e1cf0 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // e1cf7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1cf8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e1cfa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e1cfc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1cfd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe1cfe => {
            // block e1cfe
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1cfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1cfe mov rdi,r15
            rdi = r15;
            // e1d01 call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1d06 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e1d09 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1d0c mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // e1d10 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e1d14 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e1d19 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e1d1e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1d21 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1d23 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // e1d27 mov eax,0xbfffffff
            rax = (0xbfffffffu64) as u32 as u64;
            // e1d2c mov edx,0x7fffffff
            rdx = (0x7fffffffu64) as u32 as u64;
            // e1d31 cmove edx,eax
            if fl.e() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e1d34 and edx,DWORD PTR [rbp-0x1c]
            { let r = (rdx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e1d37 mov DWORD PTR [rbp-0x1c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rdx as u32 as u64)) as u32);
            // e1d3a mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e1d3d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1d40 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e1d44 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e1d49 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e1d4e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1d51 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1d53 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe1d55;
        }
        0xe1d55 => {
            // block e1d55
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1d55 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e1d5c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1d61 jmp 0xe1cdc
            pc = 0xe1cdc; continue;
        }
        0xe1d66 => {
            // block e1d66
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1d66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1d66 mov rbx,rax
            rbx = rax;
            // e1d69 jmp 0xe1d9e
            pc = 0xe1d9e; continue;
        }
        0xe1d6b => {
            // block e1d6b
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1d6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1d6b mov rbx,rax
            rbx = rax;
            // e1d6e jmp 0xe1d92
            pc = 0xe1d92; continue;
        }
        0xe1d70 => {
            // block e1d70
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1d70 mov rbx,rax
            rbx = rax;
            // e1d73 jmp 0xe1da7
            pc = 0xe1da7; continue;
        }
        0xe1d75 => {
            // block e1d75
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1d75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1d75 mov rbx,rax
            rbx = rax;
            // e1d78 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e1d7c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1d81 jmp 0xe1d9e
            pc = 0xe1d9e; continue;
        }
        0xe1d83 => {
            // block e1d83
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1d83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1d83 mov rbx,rax
            rbx = rax;
            // e1d86 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e1d8d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1d92;
        }
        0xe1d92 => {
            // block e1d92
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1d92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1d92 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e1d99 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1d9e;
        }
        0xe1d9e => {
            // block e1d9e
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1d9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1d9e lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e1da2 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1da7;
        }
        0xe1da7 => {
            // block e1da7
            #[cfg(feature = "trace")] eprintln!("f_000e1aea pc e1da7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1da7 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e1dab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1db0 mov rdi,rbx
            rdi = rbx;
            // e1db3 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1db8 mov rbx,rax
            rbx = rax;
            // e1dbb lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e1dc2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1dc7 jmp 0xe1d92
            pc = 0xe1d92; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x32996 size=710
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032996<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x32996;
    loop {
        match pc {
        0x32996 => {
            // block 32996
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32996 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32997 mov rbp,rsp
            rbp = rsp;
            // 3299a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3299c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3299d mov QWORD PTR [rdi+0x8],0x0
            m.w64(rdi.wrapping_add(0x8), (0x0u64) as u64);
            // 329a5 mov QWORD PTR [rdi],0x0
            m.w64(rdi, (0x0u64) as u64);
            // 329ac mov DWORD PTR [rdi+0x10],0xc
            m.w32(rdi.wrapping_add(0x10), (0xcu64) as u32);
            // 329b3 mov DWORD PTR [rdi+0x14],0x0
            m.w32(rdi.wrapping_add(0x14), (0x0u64) as u32);
            // 329ba mov DWORD PTR [rdi+0x18],0x20
            m.w32(rdi.wrapping_add(0x18), (0x20u64) as u32);
            // 329c1 mov DWORD PTR [rdi+0x1c],0x3
            m.w32(rdi.wrapping_add(0x1c), (0x3u64) as u32);
            // 329c8 mov BYTE PTR [rdi+0x20],0x0
            m.w8(rdi.wrapping_add(0x20), (0x0u64) as u8);
            // 329cc mov WORD PTR [rdi+0x34],0x0
            m.w16(rdi.wrapping_add(0x34), (0x0u64) as u16);
            // 329d2 mov QWORD PTR [rdi+0x2c],0x0
            m.w64(rdi.wrapping_add(0x2c), (0x0u64) as u64);
            // 329da mov QWORD PTR [rdi+0x24],0x0
            m.w64(rdi.wrapping_add(0x24), (0x0u64) as u64);
            // 329e2 mov DWORD PTR [rdi+0x38],0x12
            m.w32(rdi.wrapping_add(0x38), (0x12u64) as u32);
            // 329e9 cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // 329ec ja 0x32c27
            if fl.a() { pc = 0x32c27; continue; }
            pc = 0x329f2;
        }
        0x329f2 => {
            // block 329f2
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 329f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 329f2 cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 329f5 ja 0x32c27
            if fl.a() { pc = 0x32c27; continue; }
            pc = 0x329fb;
        }
        0x329fb => {
            // block 329fb
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 329fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 329fb mov al,r9b
            rax = (rax & !0xff) | (((r9 as u8 as u64)) as u8 as u64);
            // 329fe xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32a00 or al,cl
            { let r = (rax as u8 as u64) | (rcx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32a02 je 0x32c27
            if fl.e() { pc = 0x32c27; continue; }
            pc = 0x32a08;
        }
        0x32a08 => {
            // block 32a08
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32a08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32a08 mov r10d,edx
            r10 = ((rdx as u32 as u64)) as u32 as u64;
            // 32a0b mov r11d,esi
            r11 = ((rsi as u32 as u64)) as u32 as u64;
            // 32a0e imul rax,r11,0x780
            { let (r, f) = imul::<64>(r11, 0x780u64); rax = r; fl = f; }
            // 32a15 lea rbx,[rip+0x18fc54] # 0x1c2670
            rbx = m.env.data_va(880189);
            // 32a1c add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 32a1f imul r14,r10,0x3c
            { let (r, f) = imul::<64>(r10, 0x3cu64); r14 = r; fl = f; }
            // 32a23 mov r10d,DWORD PTR [r14+rbx*1]
            r10 = (m.r32(r14.wrapping_add(rbx)) as u64) as u32 as u64;
            // 32a27 mov eax,DWORD PTR [r14+rbx*1+0x38]
            rax = (m.r32(r14.wrapping_add(rbx).wrapping_add(0x38)) as u64) as u32 as u64;
            // 32a2c mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32a2f mov rax,QWORD PTR [r14+rbx*1+0x30]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x30)) as u64;
            // 32a34 mov QWORD PTR [rdi+0x30],rax
            m.w64(rdi.wrapping_add(0x30), (rax) as u64);
            // 32a38 mov rax,QWORD PTR [r14+rbx*1+0x28]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x28)) as u64;
            // 32a3d mov QWORD PTR [rdi+0x28],rax
            m.w64(rdi.wrapping_add(0x28), (rax) as u64);
            // 32a41 mov rax,QWORD PTR [r14+rbx*1+0x20]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x20)) as u64;
            // 32a46 mov QWORD PTR [rdi+0x20],rax
            m.w64(rdi.wrapping_add(0x20), (rax) as u64);
            // 32a4a mov rax,QWORD PTR [r14+rbx*1+0x18]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x18)) as u64;
            // 32a4f mov QWORD PTR [rdi+0x18],rax
            m.w64(rdi.wrapping_add(0x18), (rax) as u64);
            // 32a53 mov rax,QWORD PTR [r14+rbx*1+0x10]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x10)) as u64;
            // 32a58 mov QWORD PTR [rdi+0x10],rax
            m.w64(rdi.wrapping_add(0x10), (rax) as u64);
            // 32a5c mov rax,QWORD PTR [r14+rbx*1]
            rax = m.r64(r14.wrapping_add(rbx)) as u64;
            // 32a60 mov rbx,QWORD PTR [r14+rbx*1+0x8]
            rbx = m.r64(r14.wrapping_add(rbx).wrapping_add(0x8)) as u64;
            // 32a65 mov QWORD PTR [rdi+0x8],rbx
            m.w64(rdi.wrapping_add(0x8), (rbx) as u64);
            // 32a69 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 32a6c cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // 32a6f ja 0x32ba1
            if fl.a() { pc = 0x32ba1; continue; }
            pc = 0x32a75;
        }
        0x32a75 => {
            // block 32a75
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32a75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32a75 lea rax,[rip+0x1b0] # 0x32c2c
            rax = m.env.code_va(0x32c2c);
            // 32a7c movsxd rbx,DWORD PTR [rax+r11*4]
            rbx = sx::<32>(m.r32(rax.wrapping_add(r11.wrapping_mul(4))) as u64) as u64;
            // 32a80 add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 32a83 jmp rbx
            pc = (rbx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x32a85 => {
            // block 32a85
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32a85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32a85 mov DWORD PTR [rdi+0x38],0x0
            m.w32(rdi.wrapping_add(0x38), (0x0u64) as u32);
            // 32a8c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32a8e je 0x32ae5
            if fl.e() { pc = 0x32ae5; continue; }
            pc = 0x32a90;
        }
        0x32a90 => {
            // block 32a90
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32a90 movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32a94 lea eax,[rax+rax*1+0x458]
            rax = (rax.wrapping_add(rax).wrapping_add(0x458)) as u32 as u64;
            // 32a9b mov DWORD PTR [rdi],eax
            m.w32(rdi, ((rax as u32 as u64)) as u32);
            // 32a9d test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // 32aa0 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32aa3 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 32aa6 lea eax,[rax*4+0x4]
            rax = (rax.wrapping_mul(4).wrapping_add(0x4)) as u32 as u64;
            // 32aad mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32ab0 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32ab5 => {
            // block 32ab5
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32ab5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ab5 mov DWORD PTR [rdi+0x38],0xa
            m.w32(rdi.wrapping_add(0x38), (0xau64) as u32);
            // 32abc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32abe je 0x32ae5
            if fl.e() { pc = 0x32ae5; continue; }
            pc = 0x32ac0;
        }
        0x32ac0 => {
            // block 32ac0
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32ac0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ac0 movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32ac4 lea eax,[rax+rax*1+0x458]
            rax = (rax.wrapping_add(rax).wrapping_add(0x458)) as u32 as u64;
            // 32acb mov DWORD PTR [rdi],eax
            m.w32(rdi, ((rax as u32 as u64)) as u32);
            // 32acd test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // 32ad0 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32ad3 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 32ad6 lea eax,[rax*8+0x5]
            rax = (rax.wrapping_mul(8).wrapping_add(0x5)) as u32 as u64;
            // 32add mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32ae0 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32ae5 => {
            // block 32ae5
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32ae5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ae5 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32ae7 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32aea mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32aed mov DWORD PTR [rdi+0x10],esi
            m.w32(rdi.wrapping_add(0x10), ((rsi as u32 as u64)) as u32);
            // 32af0 jmp 0x32c03
            pc = 0x32c03; continue;
        }
        0x32af5 => {
            // block 32af5
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32af5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32af5 mov DWORD PTR [rdi+0x38],0x10
            m.w32(rdi.wrapping_add(0x38), (0x10u64) as u32);
            // 32afc jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b01 => {
            // block 32b01
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b01 mov DWORD PTR [rdi+0x38],0x11
            m.w32(rdi.wrapping_add(0x38), (0x11u64) as u32);
            // 32b08 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b0d => {
            // block 32b0d
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b0d mov DWORD PTR [rdi+0x38],0x1
            m.w32(rdi.wrapping_add(0x38), (0x1u64) as u32);
            // 32b14 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32b16 je 0x32bc1
            if fl.e() { pc = 0x32bc1; continue; }
            pc = 0x32b1c;
        }
        0x32b1c => {
            // block 32b1c
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b1c mov DWORD PTR [rdi],0x2e4
            m.w32(rdi, (0x2e4u64) as u32);
            // 32b22 mov DWORD PTR [rdi+0x38],0x9
            m.w32(rdi.wrapping_add(0x38), (0x9u64) as u32);
            // 32b29 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b2b => {
            // block 32b2b
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b2b mov DWORD PTR [rdi+0x38],0x2
            m.w32(rdi.wrapping_add(0x38), (0x2u64) as u32);
            // 32b32 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32b34 je 0x32bd2
            if fl.e() { pc = 0x32bd2; continue; }
            pc = 0x32b3a;
        }
        0x32b3a => {
            // block 32b3a
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b3a test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // 32b3d mov eax,0x202
            rax = (0x202u64) as u32 as u64;
            // 32b42 mov ebx,0x1fc
            rbx = (0x1fcu64) as u32 as u64;
            // 32b47 cmovne ebx,eax
            if fl.ne() { rbx = ((rax as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // 32b4a mov DWORD PTR [rdi],ebx
            m.w32(rdi, ((rbx as u32 as u64)) as u32);
            // 32b4c movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32b50 lea eax,[rax*8+0x6]
            rax = (rax.wrapping_mul(8).wrapping_add(0x6)) as u32 as u64;
            // 32b57 mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32b5a jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b5c => {
            // block 32b5c
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b5c mov DWORD PTR [rdi+0x38],0x3
            m.w32(rdi.wrapping_add(0x38), (0x3u64) as u32);
            // 32b63 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32b65 je 0x32be3
            if fl.e() { pc = 0x32be3; continue; }
            pc = 0x32b67;
        }
        0x32b67 => {
            // block 32b67
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b67 test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // 32b6a mov eax,0x264
            rax = (0x264u64) as u32 as u64;
            // 32b6f mov ebx,0x256
            rbx = (0x256u64) as u32 as u64;
            // 32b74 cmovne ebx,eax
            if fl.ne() { rbx = ((rax as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // 32b77 mov DWORD PTR [rdi],ebx
            m.w32(rdi, ((rbx as u32 as u64)) as u32);
            // 32b79 movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32b7d lea eax,[rax*8+0x7]
            rax = (rax.wrapping_mul(8).wrapping_add(0x7)) as u32 as u64;
            // 32b84 mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32b87 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b89 => {
            // block 32b89
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b89 mov DWORD PTR [rdi+0x38],0xb
            m.w32(rdi.wrapping_add(0x38), (0xbu64) as u32);
            // 32b90 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32b92 je 0x32bf4
            if fl.e() { pc = 0x32bf4; continue; }
            pc = 0x32b94;
        }
        0x32b94 => {
            // block 32b94
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b94 mov DWORD PTR [rdi],0x634
            m.w32(rdi, (0x634u64) as u32);
            // 32b9a mov DWORD PTR [rdi+0x38],0xc
            m.w32(rdi.wrapping_add(0x38), (0xcu64) as u32);
            pc = 0x32ba1;
        }
        0x32ba1 => {
            // block 32ba1
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32ba1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ba1 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32ba3 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32ba6 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32ba9 mov DWORD PTR [rdi+0x10],esi
            m.w32(rdi.wrapping_add(0x10), ((rsi as u32 as u64)) as u32);
            // 32bac mov DWORD PTR [rdi+0x18],edx
            m.w32(rdi.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // 32baf mov BYTE PTR [rdi+0x20],r8b
            m.w8(rdi.wrapping_add(0x20), ((r8 as u8 as u64)) as u8);
            // 32bb3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 32bb5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32bb7 je 0x32c0c
            if fl.e() { pc = 0x32c0c; continue; }
            pc = 0x32bb9;
        }
        0x32bb9 => {
            // block 32bb9
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32bb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32bb9 movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32bbd inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 32bbf jmp 0x32c0c
            pc = 0x32c0c; continue;
        }
        0x32bc1 => {
            // block 32bc1
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32bc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32bc1 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32bc3 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32bc6 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32bc9 mov DWORD PTR [rdi+0x10],0x1
            m.w32(rdi.wrapping_add(0x10), (0x1u64) as u32);
            // 32bd0 jmp 0x32c03
            pc = 0x32c03; continue;
        }
        0x32bd2 => {
            // block 32bd2
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32bd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32bd2 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32bd4 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32bd7 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32bda mov DWORD PTR [rdi+0x10],0x2
            m.w32(rdi.wrapping_add(0x10), (0x2u64) as u32);
            // 32be1 jmp 0x32c03
            pc = 0x32c03; continue;
        }
        0x32be3 => {
            // block 32be3
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32be3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32be3 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32be5 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32be8 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32beb mov DWORD PTR [rdi+0x10],0x3
            m.w32(rdi.wrapping_add(0x10), (0x3u64) as u32);
            // 32bf2 jmp 0x32c03
            pc = 0x32c03; continue;
        }
        0x32bf4 => {
            // block 32bf4
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32bf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32bf4 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32bf6 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32bf9 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32bfc mov DWORD PTR [rdi+0x10],0x5
            m.w32(rdi.wrapping_add(0x10), (0x5u64) as u32);
            pc = 0x32c03;
        }
        0x32c03 => {
            // block 32c03
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c03 mov DWORD PTR [rdi+0x18],edx
            m.w32(rdi.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // 32c06 mov BYTE PTR [rdi+0x20],r8b
            m.w8(rdi.wrapping_add(0x20), ((r8 as u8 as u64)) as u8);
            // 32c0a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x32c0c;
        }
        0x32c0c => {
            // block 32c0c
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c0c mov DWORD PTR [rdi+0x1c],eax
            m.w32(rdi.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 32c0f cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 32c12 ja 0x32c27
            if fl.a() { pc = 0x32c27; continue; }
            pc = 0x32c14;
        }
        0x32c14 => {
            // block 32c14
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c14 mov eax,0xfc800400
            rax = (0xfc800400u64) as u32 as u64;
            // 32c19 bt eax,edx
            fl.cf = bit::<32>((rax as u32 as u64), (rdx as u32 as u64));
            // 32c1c jae 0x32c27
            if fl.ae() { pc = 0x32c27; continue; }
            pc = 0x32c1e;
        }
        0x32c1e => {
            // block 32c1e
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c1e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32c1f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32c21 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32c22 jmp 0x325ca
            { let r = { let t = m.env.code_va(0x325ca); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x32c27 => {
            // block 32c27
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c27 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32c28 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32c2a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32c2b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3b22 size=690
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3b22<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3b22;
    loop {
        match pc {
        0xd3b22 => {
            // block d3b22
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3b22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3b22 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3b23 mov rbp,rsp
            rbp = rsp;
            // d3b26 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3b28 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d3b2a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d3b2c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d3b2e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3b2f sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d3b36 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d3b39 mov rbx,rdi
            rbx = rdi;
            // d3b3c lea rax,[rbp-0xa0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d3b43 mov QWORD PTR [rbp-0x98],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u64);
            // d3b4e mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // d3b59 mov QWORD PTR [rbp-0xa8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rax) as u64);
            // d3b60 mov DWORD PTR [rbp-0xac],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff54), (0x0u64) as u32);
            // d3b6a mov DWORD PTR [rbp-0xb0],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u32);
            // d3b74 mov DWORD PTR [rcx],0x0
            m.w32(rcx, (0x0u64) as u32);
            // d3b7a mov DWORD PTR [rdx],0x0
            m.w32(rdx, (0x0u64) as u32);
            // d3b80 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // d3b83 je 0xd3d7f
            if fl.e() { pc = 0xd3d7f; continue; }
            pc = 0xd3b89;
        }
        0xd3b89 => {
            // block d3b89
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3b89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3b89 mov al,BYTE PTR [rbx+0xa]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0xa)) as u64) as u8 as u64);
            // d3b8c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3b8e je 0xd3d7f
            if fl.e() { pc = 0xd3d7f; continue; }
            pc = 0xd3b94;
        }
        0xd3b94 => {
            // block d3b94
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3b94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3b94 mov QWORD PTR [rbp-0xc8],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rdx) as u64);
            // d3b9b mov QWORD PTR [rbp-0xc0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rcx) as u64);
            // d3ba2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3ba5 mov rax,QWORD PTR [rax+0x470]
            rax = m.r64(rax.wrapping_add(0x470)) as u64;
            // d3bac lea rsi,[rbp-0xac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff54);
            // d3bb3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d3bb5 mov rdi,rbx
            rdi = rbx;
            // d3bb8 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3bba mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3bbd mov rax,QWORD PTR [rax+0x470]
            rax = m.r64(rax.wrapping_add(0x470)) as u64;
            // d3bc4 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // d3bcb mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // d3bd0 mov rdi,rbx
            rdi = rbx;
            // d3bd3 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3bd5 mov DWORD PTR [rbp-0xb8],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff48), ((r14 as u32 as u64)) as u32);
            // d3bdc xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d3bdf lea r15,[rbp-0xa8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            pc = 0xd3be6;
        }
        0xd3be6 => {
            // block d3be6
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3be6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3be6 lea rax,[rip+0x2f313] # 0x102f00
            rax = m.env.data_va(95949);
            // d3bed mov esi,DWORD PTR [rax+r14*4]
            rsi = (m.r32(rax.wrapping_add(r14.wrapping_mul(4))) as u64) as u32 as u64;
            // d3bf1 xorps xmm0,xmm0
            x0 = 0;
            // d3bf4 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d3bf8 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d3bfc movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d3c00 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d3c04 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // d3c0b mov DWORD PTR [rbp-0x38],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u32);
            // d3c12 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // d3c1a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3c1d mov rax,QWORD PTR [rax+0xc18]
            rax = m.r64(rax.wrapping_add(0xc18)) as u64;
            // d3c24 mov rdi,rbx
            rdi = rbx;
            // d3c27 lea rdx,[rbp-0x90]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d3c2e call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3c30 cmp DWORD PTR [rbp-0x8c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff74)) as u64, 0x0u64).1;
            // d3c37 je 0xd3cb2
            if fl.e() { pc = 0xd3cb2; continue; }
            pc = 0xd3c39;
        }
        0xd3c39 => {
            // block d3c39
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c39 xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d3c3b jne 0xd3cb2
            if fl.ne() { pc = 0xd3cb2; continue; }
            pc = 0xd3c3d;
        }
        0xd3c3d => {
            // block d3c3d
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c3d mov r13d,DWORD PTR [rbp-0x84]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c)) as u64) as u32 as u64;
            // d3c44 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // d3c47 and eax,0xfffffff7
            { let r = (rax as u32 as u64) & 0xfffffff7u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3c4a jne 0xd3c5f
            if fl.ne() { pc = 0xd3c5f; continue; }
            pc = 0xd3c4c;
        }
        0xd3c4c => {
            // block d3c4c
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c4c mov ecx,DWORD PTR [rbp-0xac]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff54)) as u64) as u32 as u64;
            // d3c52 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3c54 je 0xd3c5f
            if fl.e() { pc = 0xd3c5f; continue; }
            pc = 0xd3c56;
        }
        0xd3c56 => {
            // block d3c56
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c56 mov r12d,DWORD PTR [rbp-0x88]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // d3c5d jmp 0xd3c75
            pc = 0xd3c75; continue;
        }
        0xd3c5f => {
            // block d3c5f
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c5f mov r12d,DWORD PTR [rbp-0x88]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // d3c66 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // d3c69 jne 0xd3c7d
            if fl.ne() { pc = 0xd3c7d; continue; }
            pc = 0xd3c6b;
        }
        0xd3c6b => {
            // block d3c6b
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c6b mov eax,DWORD PTR [rbp-0xb0]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64) as u32 as u64;
            // d3c71 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d3c73 je 0xd3c7d
            if fl.e() { pc = 0xd3c7d; continue; }
            pc = 0xd3c75;
        }
        0xd3c75 => {
            // block d3c75
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c75 sub r13d,r12d
            { let (r, f) = sub::<32>((r13 as u32 as u64), (r12 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // d3c78 lea r13d,[r12+r13*4+0x3]
            r13 = (r12.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x3)) as u32 as u64;
            pc = 0xd3c7d;
        }
        0xd3c7d => {
            // block d3c7d
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c7d mov DWORD PTR [rbp-0xb4],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff4c), ((r12 as u32 as u64)) as u32);
            // d3c84 cmp r12d,r13d
            fl = sub::<32>((r12 as u32 as u64), (r13 as u32 as u64)).1;
            // d3c87 jg 0xd3cb2
            if fl.g() { pc = 0xd3cb2; continue; }
            pc = 0xd3c89;
        }
        0xd3c89 => {
            // block d3c89
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c89 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            pc = 0xd3c8c;
        }
        0xd3c8c => {
            // block d3c8c
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c8c mov rdi,r15
            rdi = r15;
            // d3c8f lea rsi,[rbp-0xb4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff4c);
            // d3c96 call 0xebea0
            { let r = { let t = m.env.stub_va(0x66d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3c9b mov DWORD PTR [rbp-0xb4],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff4c), ((r12 as u32 as u64)) as u32);
            // d3ca2 lea eax,[r12+0x1]
            rax = (r12.wrapping_add(0x1)) as u32 as u64;
            // d3ca7 dec r12d
            { let (r, f) = dec::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // d3caa cmp r12d,r13d
            fl = sub::<32>((r12 as u32 as u64), (r13 as u32 as u64)).1;
            // d3cad mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // d3cb0 jl 0xd3c8c
            if fl.l() { pc = 0xd3c8c; continue; }
            pc = 0xd3cb2;
        }
        0xd3cb2 => {
            // block d3cb2
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3cb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cb2 inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // d3cb5 cmp r14,0x10
            fl = sub::<64>(r14, 0x10u64).1;
            // d3cb9 jb 0xd3be6
            if fl.b() { pc = 0xd3be6; continue; }
            pc = 0xd3cbf;
        }
        0xd3cbf => {
            // block d3cbf
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3cbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cbf mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d3cc2 call 0xd19e0
            { let r = { let t = m.env.code_va(0xd19e0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3cc7 mov ecx,DWORD PTR [rbp-0xb8]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // d3ccd dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d3ccf movzx r8d,cl
            r8 = ((rcx as u8 as u64)) as u32 as u64;
            // d3cd3 mov rdi,QWORD PTR [rbp-0xa8]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // d3cda lea r9d,[r8-0x1]
            r9 = (r8.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // d3cde xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // d3ce1 lea rdx,[rbp-0xa0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d3ce8 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // d3ceb je 0xd3d54
            if fl.e() { pc = 0xd3d54; continue; }
            pc = 0xd3ced;
        }
        0xd3ced => {
            // block d3ced
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3ced rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3ced xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // d3cf0 mov r11d,r8d
            r11 = ((r8 as u32 as u64)) as u32 as u64;
            pc = 0xd3cf3;
        }
        0xd3cf3 => {
            // block d3cf3
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3cf3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cf3 mov r14d,DWORD PTR [rdi+0x1c]
            r14 = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // d3cf7 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0xd3cfa;
        }
        0xd3cfa => {
            // block d3cfa
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3cfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cfa mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // d3cfc mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // d3d00 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // d3d03 je 0xd3d12
            if fl.e() { pc = 0xd3d12; continue; }
            pc = 0xd3d05;
        }
        0xd3d05 => {
            // block d3d05
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d05 mov rdi,rcx
            rdi = rcx;
            // d3d08 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // d3d0b test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // d3d0e jne 0xd3d05
            if fl.ne() { pc = 0xd3d05; continue; }
            pc = 0xd3d10;
        }
        0xd3d10 => {
            // block d3d10
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d10 jmp 0xd3d21
            pc = 0xd3d21; continue;
        }
        0xd3d12 => {
            // block d3d12
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d12 mov rcx,rdi
            rcx = rdi;
            pc = 0xd3d15;
        }
        0xd3d15 => {
            // block d3d15
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d15 mov rdi,QWORD PTR [rcx+0x10]
            rdi = m.r64(rcx.wrapping_add(0x10)) as u64;
            // d3d19 cmp QWORD PTR [rdi],rcx
            fl = sub::<64>(m.r64(rdi) as u64, rcx).1;
            // d3d1c mov rcx,rdi
            rcx = rdi;
            // d3d1f jne 0xd3d15
            if fl.ne() { pc = 0xd3d15; continue; }
            pc = 0xd3d21;
        }
        0xd3d21 => {
            // block d3d21
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d21 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // d3d24 mov rcx,rdx
            rcx = rdx;
            // d3d27 je 0xd3d36
            if fl.e() { pc = 0xd3d36; continue; }
            pc = 0xd3d29;
        }
        0xd3d29 => {
            // block d3d29
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d29 mov ecx,DWORD PTR [rdi+0x1c]
            rcx = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // d3d2c lea ebx,[rsi+0x1]
            rbx = (rsi.wrapping_add(0x1)) as u32 as u64;
            // d3d2f cmp ecx,ebx
            fl = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64)).1;
            // d3d31 je 0xd3cfa
            if fl.e() { pc = 0xd3cfa; continue; }
            pc = 0xd3d33;
        }
        0xd3d33 => {
            // block d3d33
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d33 mov rcx,rdi
            rcx = rdi;
            pc = 0xd3d36;
        }
        0xd3d36 => {
            // block d3d36
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d36 cmp r10d,r14d
            fl = sub::<32>((r10 as u32 as u64), (r14 as u32 as u64)).1;
            // d3d39 jge 0xd3d40
            if fl.ge() { pc = 0xd3d40; continue; }
            pc = 0xd3d3b;
        }
        0xd3d3b => {
            // block d3d3b
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d3b cmp r11d,r14d
            fl = sub::<32>((r11 as u32 as u64), (r14 as u32 as u64)).1;
            // d3d3e jle 0xd3d54
            if fl.le() { pc = 0xd3d54; continue; }
            pc = 0xd3d40;
        }
        0xd3d40 => {
            // block d3d40
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d40 lea r10d,[rsi+0x1]
            r10 = (rsi.wrapping_add(0x1)) as u32 as u64;
            // d3d44 lea r11d,[rsi+r8*1+0x1]
            r11 = (rsi.wrapping_add(r8).wrapping_add(0x1)) as u32 as u64;
            // d3d49 add esi,r8d
            { let (r, f) = add::<32>((rsi as u32 as u64), (r8 as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // d3d4c cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // d3d4f mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // d3d52 jne 0xd3cf3
            if fl.ne() { pc = 0xd3cf3; continue; }
            pc = 0xd3d54;
        }
        0xd3d54 => {
            // block d3d54
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d54 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d3d5b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3d5d cmp r10d,eax
            fl = sub::<32>((r10 as u32 as u64), (rax as u32 as u64)).1;
            // d3d60 jg 0xd3d88
            if fl.g() { pc = 0xd3d88; continue; }
            pc = 0xd3d62;
        }
        0xd3d62 => {
            // block d3d62
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d62 cmp r9d,eax
            fl = sub::<32>((r9 as u32 as u64), (rax as u32 as u64)).1;
            // d3d65 jg 0xd3d88
            if fl.g() { pc = 0xd3d88; continue; }
            pc = 0xd3d67;
        }
        0xd3d67 => {
            // block d3d67
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d67 mov rax,QWORD PTR [rbp-0xc8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64;
            // d3d6e mov DWORD PTR [rax],r10d
            m.w32(rax, ((r10 as u32 as u64)) as u32);
            // d3d71 mov rax,QWORD PTR [rbp-0xc0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // d3d78 mov DWORD PTR [rax],r9d
            m.w32(rax, ((r9 as u32 as u64)) as u32);
            // d3d7b mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // d3d7d jmp 0xd3d88
            pc = 0xd3d88; continue;
        }
        0xd3d7f => {
            // block d3d7f
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d7f lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d3d86 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3d88;
        }
        0xd3d88 => {
            // block d3d88
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d88 mov rsi,QWORD PTR [rbp-0xa0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // d3d8f call 0xebeac
            { let r = { let t = m.env.stub_va(0x679); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3d94 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d3d96 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d3d9d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3d9e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d3da0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d3da2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3da4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3da6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3da7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd3da8 => {
            // block d3da8
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3da8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3da8 mov r14,rax
            r14 = rax;
            pc = 0xd3dab;
        }
        0xd3dab => {
            // block d3dab
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3dab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3dab mov rsi,QWORD PTR [rbp-0xa0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // d3db2 mov rdi,r15
            rdi = r15;
            // d3db5 call 0xebeac
            { let r = { let t = m.env.stub_va(0x679); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3dba mov rdi,r14
            rdi = r14;
            // d3dbd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3dc2 mov r14,rax
            r14 = rax;
            // d3dc5 jmp 0xd3dab
            pc = 0xd3dab; continue;
        }
        0xd3dc7 => {
            // block d3dc7
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3dc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3dc7 mov r14,rax
            r14 = rax;
            // d3dca lea r15,[rbp-0xa8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d3dd1 jmp 0xd3dab
            pc = 0xd3dab; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9740 size=668
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9740<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9740;
    loop {
        match pc {
        0xe9740 => {
            // block e9740
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9740 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9740 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9741 mov rbp,rsp
            rbp = rsp;
            // e9744 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9746 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9748 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e974a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e974c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e974d sub rsp,0xc8
            { let (r, f) = sub::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // e9754 mov ebx,r8d
            rbx = ((r8 as u32 as u64)) as u32 as u64;
            // e9757 mov r15,rcx
            r15 = rcx;
            // e975a mov DWORD PTR [rbp-0xe4],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rdx as u32 as u64)) as u32);
            // e9760 mov r13,rsi
            r13 = rsi;
            // e9763 mov r14,rdi
            r14 = rdi;
            // e9766 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e9769 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e976c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e976f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e9771 je 0xe99a7
            if fl.e() { pc = 0xe99a7; continue; }
            pc = 0xe9777;
        }
        0xe9777 => {
            // block e9777
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9777 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9777 mov BYTE PTR [rbp-0xc0],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u8);
            // e977e mov WORD PTR [rbp-0xbe],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff42), (0x0u64) as u16);
            // e9787 mov DWORD PTR [rbp-0xbc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff44), (0x0u64) as u32);
            // e9791 mov DWORD PTR [rbp-0xb8],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (0x0u64) as u32);
            // e979b mov BYTE PTR [rbp-0x30],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u8);
            // e979f test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e97a1 je 0xe97d3
            if fl.e() { pc = 0xe97d3; continue; }
            pc = 0xe97a3;
        }
        0xe97a3 => {
            // block e97a3
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e97a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e97a3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e97a6 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e97ad mov rdi,r14
            rdi = r14;
            // e97b0 call QWORD PTR [rax+0x98]
            { let r = { let t = m.r64(rax.wrapping_add(0x98)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e97b6 mov ax,WORD PTR [rbp-0xbe]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff42)) as u64) as u16 as u64);
            // e97bd mov DWORD PTR [rbp-0xe4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rax as u32 as u64)) as u32);
            // e97c3 mov rax,QWORD PTR [r14+0x288]
            rax = m.r64(r14.wrapping_add(0x288)) as u64;
            // e97ca mov QWORD PTR [r14+0x280],rax
            m.w64(r14.wrapping_add(0x280), (rax) as u64);
            // e97d1 jmp 0xe97fe
            pc = 0xe97fe; continue;
        }
        0xe97d3 => {
            // block e97d3
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e97d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e97d3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e97d6 mov ecx,DWORD PTR [rbp-0xe4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c)) as u64) as u32 as u64;
            // e97dc movzx esi,cx
            rsi = ((rcx as u16 as u64)) as u32 as u64;
            // e97df mov rdi,r14
            rdi = r14;
            // e97e2 call QWORD PTR [rax+0x40]
            { let r = { let t = m.r64(rax.wrapping_add(0x40)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e97e5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e97e8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e97ea je 0xe99a7
            if fl.e() { pc = 0xe99a7; continue; }
            pc = 0xe97f0;
        }
        0xe97f0 => {
            // block e97f0
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e97f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e97f0 mov rax,QWORD PTR [r14+0x280]
            rax = m.r64(r14.wrapping_add(0x280)) as u64;
            // e97f7 mov QWORD PTR [r14+0x288],rax
            m.w64(r14.wrapping_add(0x288), (rax) as u64);
            pc = 0xe97fe;
        }
        0xe97fe => {
            // block e97fe
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e97fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e97fe add rax,0x18
            { let (r, f) = add::<64>(rax, 0x18u64);
              rax = r;
              fl = f; }
            // e9802 mov QWORD PTR [r14+0x288],rax
            m.w64(r14.wrapping_add(0x288), (rax) as u64);
            // e9809 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e980c lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e9813 mov rdi,r14
            rdi = r14;
            // e9816 call QWORD PTR [rax+0x98]
            { let r = { let t = m.r64(rax.wrapping_add(0x98)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e981c mov eax,DWORD PTR [rbp-0xb8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // e9822 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e9825 jne 0xe9837
            if fl.ne() { pc = 0xe9837; continue; }
            pc = 0xe9827;
        }
        0xe9827 => {
            // block e9827
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9827 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9827 mov ax,WORD PTR [rbp-0xbe]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff42)) as u64) as u16 as u64);
            // e982e mov WORD PTR [r15],ax
            m.w16(r15, ((rax as u16 as u64)) as u16);
            // e9832 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            pc = 0xe9837;
        }
        0xe9837 => {
            // block e9837
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9837 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9837 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e983a jne 0xe9989
            if fl.ne() { pc = 0xe9989; continue; }
            pc = 0xe9840;
        }
        0xe9840 => {
            // block e9840
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9840 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9840 lea rbx,[rbp-0xd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e9847 lea r12,[rbp-0xdc]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff24);
            pc = 0xe984e;
        }
        0xe984e => {
            // block e984e
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e984e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e984e mov r15,QWORD PTR [r14+0x288]
            r15 = m.r64(r14.wrapping_add(0x288)) as u64;
            // e9855 test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // e9859 jne 0xe9860
            if fl.ne() { pc = 0xe9860; continue; }
            pc = 0xe985b;
        }
        0xe985b => {
            // block e985b
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e985b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e985b inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e985e jmp 0xe9864
            pc = 0xe9864; continue;
        }
        0xe9860 => {
            // block e9860
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9860 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9860 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0xe9864;
        }
        0xe9864 => {
            // block e9864
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9864 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9864 mov rdi,r15
            rdi = r15;
            // e9867 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e986c mov rdi,rbx
            rdi = rbx;
            // e986f mov rsi,r15
            rsi = r15;
            // e9872 mov rdx,rax
            rdx = rax;
            // e9875 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e987a cmp BYTE PTR [rbp-0xc0],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64, 0x0u64).1;
            // e9881 lea r15,[rip+0x53a9a] # 0x13d322
            r15 = m.env.data_va(334575);
            // e9888 je 0xe9905
            if fl.e() { pc = 0xe9905; continue; }
            pc = 0xe988a;
        }
        0xe988a => {
            // block e988a
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e988a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e988a mov ebx,0x9
            rbx = (0x9u64) as u32 as u64;
            pc = 0xe988f;
        }
        0xe988f => {
            // block e988f
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e988f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e988f mov DWORD PTR [rbp-0xdc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff24), (0x0u64) as u32);
            // e9899 mov rdi,QWORD PTR [r14+0x288]
            rdi = m.r64(r14.wrapping_add(0x288)) as u64;
            // e98a0 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e98a3 jne 0xe98aa
            if fl.ne() { pc = 0xe98aa; continue; }
            pc = 0xe98a5;
        }
        0xe98a5 => {
            // block e98a5
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98a5 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e98a8 jmp 0xe98ae
            pc = 0xe98ae; continue;
        }
        0xe98aa => {
            // block e98aa
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98aa mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe98ae;
        }
        0xe98ae => {
            // block e98ae
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98ae add rdi,rbx
            { let (r, f) = add::<64>(rdi, rbx);
              rdi = r;
              fl = f; }
            // e98b1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e98b3 mov rsi,r15
            rsi = r15;
            // e98b6 mov rdx,r12
            rdx = r12;
            // e98b9 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e98be mov al,BYTE PTR [rbp-0xdc]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff24)) as u64) as u8 as u64);
            // e98c4 mov BYTE PTR [rbp-0xdd],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff23), ((rax as u8 as u64)) as u8);
            // e98ca mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // e98ce cmp rcx,QWORD PTR [r13+0x10]
            fl = sub::<64>(rcx, m.r64(r13.wrapping_add(0x10)) as u64).1;
            // e98d2 jae 0xe98dc
            if fl.ae() { pc = 0xe98dc; continue; }
            pc = 0xe98d4;
        }
        0xe98d4 => {
            // block e98d4
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98d4 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // e98d6 inc QWORD PTR [r13+0x8]
            { let (r, f) = inc::<64>(m.r64(r13.wrapping_add(0x8)) as u64, fl); m.w64(r13.wrapping_add(0x8), (r) as u64); fl = f; }
            // e98da jmp 0xe98eb
            pc = 0xe98eb; continue;
        }
        0xe98dc => {
            // block e98dc
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98dc mov rdi,r13
            rdi = r13;
            // e98df lea rsi,[rbp-0xdd]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff23);
            // e98e6 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe98eb;
        }
        0xe98eb => {
            // block e98eb
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98eb movzx eax,BYTE PTR [rbp-0xc0]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64) as u32 as u64;
            // e98f2 add rax,rax
            { let (r, f) = add::<64>(rax, rax);
              rax = r;
              fl = f; }
            // e98f5 lea rcx,[rbx+0x2]
            rcx = rbx.wrapping_add(0x2);
            // e98f9 add rbx,0xfffffffffffffff9
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_fff9u64);
              rbx = r;
              fl = f; }
            // e98fd cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // e9900 mov rbx,rcx
            rbx = rcx;
            // e9903 jl 0xe988f
            if fl.l() { pc = 0xe988f; continue; }
            pc = 0xe9905;
        }
        0xe9905 => {
            // block e9905
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9905 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9905 add QWORD PTR [r14+0x288],0x18
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x288)) as u64, 0x18u64);
              m.w64(r14.wrapping_add(0x288), (r) as u64);
              fl = f; }
            // e990d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e9910 mov rax,QWORD PTR [rax+0x98]
            rax = m.r64(rax.wrapping_add(0x98)) as u64;
            // e9917 mov rdi,r14
            rdi = r14;
            // e991a lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e9921 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9923 lea rbx,[rbp-0xd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e992a mov eax,DWORD PTR [rbp-0xb8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // e9930 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e9933 je 0xe9974
            if fl.e() { pc = 0xe9974; continue; }
            pc = 0xe9935;
        }
        0xe9935 => {
            // block e9935
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9935 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // e9938 jne 0xe9993
            if fl.ne() { pc = 0xe9993; continue; }
            pc = 0xe993a;
        }
        0xe993a => {
            // block e993a
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e993a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e993a mov eax,DWORD PTR [rbp-0xe4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c)) as u64) as u32 as u64;
            // e9940 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e9942 mov DWORD PTR [rbp-0xe4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rax as u32 as u64)) as u32);
            // e9948 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // e994b movzx ecx,WORD PTR [rbp-0xbe]
            rcx = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff42)) as u64) as u32 as u64;
            // e9952 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // e9954 jne 0xe9993
            if fl.ne() { pc = 0xe9993; continue; }
            pc = 0xe9956;
        }
        0xe9956 => {
            // block e9956
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9956 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9956 add QWORD PTR [r14+0x288],0x18
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x288)) as u64, 0x18u64);
              m.w64(r14.wrapping_add(0x288), (r) as u64);
              fl = f; }
            // e995e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e9961 mov rax,QWORD PTR [rax+0x98]
            rax = m.r64(rax.wrapping_add(0x98)) as u64;
            // e9968 mov rdi,r14
            rdi = r14;
            // e996b lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e9972 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe9974;
        }
        0xe9974 => {
            // block e9974
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9974 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9974 mov rdi,rbx
            rdi = rbx;
            // e9977 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e997c cmp DWORD PTR [rbp-0xb8],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64, 0x1u64).1;
            // e9983 je 0xe984e
            if fl.e() { pc = 0xe984e; continue; }
            pc = 0xe9989;
        }
        0xe9989 => {
            // block e9989
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9989 mov r12d,DWORD PTR [r13+0x8]
            r12 = (m.r32(r13.wrapping_add(0x8)) as u64) as u32 as u64;
            // e998d sub r12d,DWORD PTR [r13+0x0]
            { let (r, f) = sub::<32>((r12 as u32 as u64), m.r32(r13) as u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e9991 jmp 0xe99a7
            pc = 0xe99a7; continue;
        }
        0xe9993 => {
            // block e9993
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9993 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9993 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            // e9997 sub r12,QWORD PTR [r13+0x0]
            { let (r, f) = sub::<64>(r12, m.r64(r13) as u64);
              r12 = r;
              fl = f; }
            // e999b lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e99a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe99a7;
        }
        0xe99a7 => {
            // block e99a7
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e99a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e99a7 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // e99aa add rsp,0xc8
            { let (r, f) = add::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // e99b1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e99b2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e99b4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e99b6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e99b8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e99ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e99bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe99bc => {
            // block e99bc
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e99bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e99bc mov rbx,rax
            rbx = rax;
            // e99bf lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e99c6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e99cb mov rdi,rbx
            rdi = rbx;
            // e99ce call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e99d3 jmp 0xe99bc
            pc = 0xe99bc; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5b092 size=641
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005b092<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5b092;
    loop {
        match pc {
        0x5b092 => {
            // block 5b092
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b092 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5b093 mov rbp,rsp
            rbp = rsp;
            // 5b096 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 5b098 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 5b09b jne 0x5b0a4
            if fl.ne() { pc = 0x5b0a4; continue; }
            pc = 0x5b09d;
        }
        0x5b09d => {
            // block 5b09d
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b09d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b09d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 5b09f jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b0a4 => {
            // block 5b0a4
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0a4 cmp rsi,0x64
            fl = sub::<64>(rsi, 0x64u64).1;
            // 5b0a8 jne 0x5b0e0
            if fl.ne() { pc = 0x5b0e0; continue; }
            pc = 0x5b0aa;
        }
        0x5b0aa => {
            // block 5b0aa
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0aa mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5b0af cmp rdi,0x12ba
            fl = sub::<64>(rdi, 0x12bau64).1;
            // 5b0b6 jg 0x5b19b
            if fl.g() { pc = 0x5b19b; continue; }
            pc = 0x5b0bc;
        }
        0x5b0bc => {
            // block 5b0bc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0bc cmp rdi,0x95d
            fl = sub::<64>(rdi, 0x95du64).1;
            // 5b0c3 jg 0x5b1c8
            if fl.g() { pc = 0x5b1c8; continue; }
            pc = 0x5b0c9;
        }
        0x5b0c9 => {
            // block 5b0c9
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0c9 cmp rdi,0x5da
            fl = sub::<64>(rdi, 0x5dau64).1;
            // 5b0d0 jne 0x5b27f
            if fl.ne() { pc = 0x5b27f; continue; }
            pc = 0x5b0d6;
        }
        0x5b0d6 => {
            // block 5b0d6
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0d6 mov eax,0xe
            rax = (0xeu64) as u32 as u64;
            // 5b0db jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b0e0 => {
            // block 5b0e0
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0e0 mov rax,rdi
            rax = rdi;
            // 5b0e3 cqo
            rdx = if (rax as i64) < 0 { u64::MAX } else { 0 };
            // 5b0e5 idiv rsi
            { let (q, r) = idiv::<64>(rdx, rax, rsi); rax = q; rdx = r; }
            // 5b0e8 mov rdx,rax
            rdx = rax;
            // 5b0eb mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5b0f0 cmp ecx,0xe
            fl = sub::<32>((rcx as u32 as u64), 0xeu64).1;
            // 5b0f3 ja 0x5b311
            if fl.a() { pc = 0x5b311; continue; }
            pc = 0x5b0f9;
        }
        0x5b0f9 => {
            // block 5b0f9
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0f9 cvtsi2ss xmm0,rdx
            x0 = set_lo32(x0, (sx::<64>(rdx) as f32).to_bits());
            // 5b0fe mulss xmm0,DWORD PTR [rip+0xa56aa] # 0x1007b0
            x0 = set_lo32(x0, (f32_of(x0) * f32_of((m.r32(m.env.data_va(85885)) as u128))).to_bits());
            // 5b106 cvttss2si rdx,xmm0
            rdx = cvtt64(f32_of(x0) as f64) as u64;
            // 5b10b mov esi,0x2b6a
            rsi = (0x2b6au64) as u32 as u64;
            // 5b110 bt esi,ecx
            fl.cf = bit::<32>((rsi as u32 as u64), (rcx as u32 as u64));
            // 5b113 jae 0x5b210
            if fl.ae() { pc = 0x5b210; continue; }
            pc = 0x5b119;
        }
        0x5b119 => {
            // block 5b119
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b119 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b119 mov eax,0xd
            rax = (0xdu64) as u32 as u64;
            // 5b11e cmp rdx,0x641
            fl = sub::<64>(rdx, 0x641u64).1;
            // 5b125 jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b12b;
        }
        0x5b12b => {
            // block 5b12b
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b12b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b12b mov eax,0x6
            rax = (0x6u64) as u32 as u64;
            // 5b130 cmp rdx,0x993
            fl = sub::<64>(rdx, 0x993u64).1;
            // 5b137 jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b13d;
        }
        0x5b13d => {
            // block 5b13d
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b13d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b13d mov eax,0x5
            rax = (0x5u64) as u32 as u64;
            // 5b142 cmp rdx,0xa29
            fl = sub::<64>(rdx, 0xa29u64).1;
            // 5b149 jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b14f;
        }
        0x5b14f => {
            // block 5b14f
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b14f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b14f mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5b154 cmp rdx,0xc1d
            fl = sub::<64>(rdx, 0xc1du64).1;
            // 5b15b jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b161;
        }
        0x5b161 => {
            // block 5b161
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b161 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b161 mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // 5b166 cmp rdx,0x1325
            fl = sub::<64>(rdx, 0x1325u64).1;
            // 5b16d jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b173;
        }
        0x5b173 => {
            // block 5b173
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b173 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b173 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // 5b178 cmp rdx,0x13ed
            fl = sub::<64>(rdx, 0x13edu64).1;
            // 5b17f jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b185;
        }
        0x5b185 => {
            // block 5b185
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b185 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b185 cmp rdx,0x17d5
            fl = sub::<64>(rdx, 0x17d5u64).1;
            // 5b18c mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 5b191 mov eax,0xb
            rax = (0xbu64) as u32 as u64;
            // 5b196 jmp 0x5b277
            pc = 0x5b277; continue;
        }
        0x5b19b => {
            // block 5b19b
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b19b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b19b cmp rdi,0x2ed3
            fl = sub::<64>(rdi, 0x2ed3u64).1;
            // 5b1a2 jg 0x5b1f9
            if fl.g() { pc = 0x5b1f9; continue; }
            pc = 0x5b1a4;
        }
        0x5b1a4 => {
            // block 5b1a4
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1a4 cmp rdi,0x1387
            fl = sub::<64>(rdi, 0x1387u64).1;
            // 5b1ab jg 0x5b293
            if fl.g() { pc = 0x5b293; continue; }
            pc = 0x5b1b1;
        }
        0x5b1b1 => {
            // block 5b1b1
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1b1 cmp rdi,0x12bb
            fl = sub::<64>(rdi, 0x12bbu64).1;
            // 5b1b8 jne 0x5b2c3
            if fl.ne() { pc = 0x5b2c3; continue; }
            pc = 0x5b1be;
        }
        0x5b1be => {
            // block 5b1be
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1be mov eax,0xa
            rax = (0xau64) as u32 as u64;
            // 5b1c3 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b1c8 => {
            // block 5b1c8
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1c8 cmp rdi,0xbb7
            fl = sub::<64>(rdi, 0xbb7u64).1;
            // 5b1cf jg 0x5b2a3
            if fl.g() { pc = 0x5b2a3; continue; }
            pc = 0x5b1d5;
        }
        0x5b1d5 => {
            // block 5b1d5
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1d5 cmp rdi,0x9c3
            fl = sub::<64>(rdi, 0x9c3u64).1;
            // 5b1dc jg 0x5b2d3
            if fl.g() { pc = 0x5b2d3; continue; }
            pc = 0x5b1e2;
        }
        0x5b1e2 => {
            // block 5b1e2
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1e2 cmp rdi,0x95e
            fl = sub::<64>(rdi, 0x95eu64).1;
            // 5b1e9 jne 0x5b2f3
            if fl.ne() { pc = 0x5b2f3; continue; }
            pc = 0x5b1ef;
        }
        0x5b1ef => {
            // block 5b1ef
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1ef mov eax,0x7
            rax = (0x7u64) as u32 as u64;
            // 5b1f4 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b1f9 => {
            // block 5b1f9
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1f9 cmp rdi,0x2ed4
            fl = sub::<64>(rdi, 0x2ed4u64).1;
            // 5b200 jne 0x5b2b3
            if fl.ne() { pc = 0x5b2b3; continue; }
            pc = 0x5b206;
        }
        0x5b206 => {
            // block 5b206
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b206 mov eax,0xc
            rax = (0xcu64) as u32 as u64;
            // 5b20b jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b210 => {
            // block 5b210
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b210 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b210 mov esi,0x5494
            rsi = (0x5494u64) as u32 as u64;
            // 5b215 bt esi,ecx
            fl.cf = bit::<32>((rsi as u32 as u64), (rcx as u32 as u64));
            // 5b218 jae 0x5b311
            if fl.ae() { pc = 0x5b311; continue; }
            pc = 0x5b21e;
        }
        0x5b21e => {
            // block 5b21e
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b21e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b21e mov eax,0xe
            rax = (0xeu64) as u32 as u64;
            // 5b223 cmp rdx,0x63f
            fl = sub::<64>(rdx, 0x63fu64).1;
            // 5b22a jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b230;
        }
        0x5b230 => {
            // block 5b230
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b230 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b230 mov eax,0x7
            rax = (0x7u64) as u32 as u64;
            // 5b235 cmp rdx,0x9c3
            fl = sub::<64>(rdx, 0x9c3u64).1;
            // 5b23c jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b242;
        }
        0x5b242 => {
            // block 5b242
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b242 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b242 mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            // 5b247 cmp rdx,0xc1a
            fl = sub::<64>(rdx, 0xc1au64).1;
            // 5b24e jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b254;
        }
        0x5b254 => {
            // block 5b254
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b254 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b254 mov eax,0xa
            rax = (0xau64) as u32 as u64;
            // 5b259 cmp rdx,0x1320
            fl = sub::<64>(rdx, 0x1320u64).1;
            // 5b260 jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b266;
        }
        0x5b266 => {
            // block 5b266
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b266 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b266 cmp rdx,0x17cf
            fl = sub::<64>(rdx, 0x17cfu64).1;
            // 5b26d mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 5b272 mov eax,0xc
            rax = (0xcu64) as u32 as u64;
            pc = 0x5b277;
        }
        0x5b277 => {
            // block 5b277
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b277 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b277 cmovl eax,ecx
            if fl.l() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 5b27a jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b27f => {
            // block 5b27f
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b27f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b27f cmp rdi,0x5dc
            fl = sub::<64>(rdi, 0x5dcu64).1;
            // 5b286 jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b28c;
        }
        0x5b28c => {
            // block 5b28c
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b28c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b28c mov eax,0xd
            rax = (0xdu64) as u32 as u64;
            // 5b291 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b293 => {
            // block 5b293
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b293 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b293 cmp rdi,0x1388
            fl = sub::<64>(rdi, 0x1388u64).1;
            // 5b29a jne 0x5b2e3
            if fl.ne() { pc = 0x5b2e3; continue; }
            pc = 0x5b29c;
        }
        0x5b29c => {
            // block 5b29c
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b29c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b29c mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // 5b2a1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2a3 => {
            // block 5b2a3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2a3 cmp rdi,0xbb8
            fl = sub::<64>(rdi, 0xbb8u64).1;
            // 5b2aa jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2ac;
        }
        0x5b2ac => {
            // block 5b2ac
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2ac mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5b2b1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2b3 => {
            // block 5b2b3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2b3 cmp rdi,0x2ee0
            fl = sub::<64>(rdi, 0x2ee0u64).1;
            // 5b2ba jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2bc;
        }
        0x5b2bc => {
            // block 5b2bc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2bc mov eax,0xb
            rax = (0xbu64) as u32 as u64;
            // 5b2c1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2c3 => {
            // block 5b2c3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2c3 cmp rdi,0x12c0
            fl = sub::<64>(rdi, 0x12c0u64).1;
            // 5b2ca jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2cc;
        }
        0x5b2cc => {
            // block 5b2cc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2cc mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // 5b2d1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2d3 => {
            // block 5b2d3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2d3 cmp rdi,0x9c4
            fl = sub::<64>(rdi, 0x9c4u64).1;
            // 5b2da jne 0x5b303
            if fl.ne() { pc = 0x5b303; continue; }
            pc = 0x5b2dc;
        }
        0x5b2dc => {
            // block 5b2dc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2dc mov eax,0x5
            rax = (0x5u64) as u32 as u64;
            // 5b2e1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2e3 => {
            // block 5b2e3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2e3 cmp rdi,0x176a
            fl = sub::<64>(rdi, 0x176au64).1;
            // 5b2ea jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2ec;
        }
        0x5b2ec => {
            // block 5b2ec
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2ec mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 5b2f1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2f3 => {
            // block 5b2f3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2f3 cmp rdi,0x960
            fl = sub::<64>(rdi, 0x960u64).1;
            // 5b2fa jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2fc;
        }
        0x5b2fc => {
            // block 5b2fc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2fc mov eax,0x6
            rax = (0x6u64) as u32 as u64;
            // 5b301 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b303 => {
            // block 5b303
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b303 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b303 cmp rdi,0xbb5
            fl = sub::<64>(rdi, 0xbb5u64).1;
            // 5b30a jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b30c;
        }
        0x5b30c => {
            // block 5b30c
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b30c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b30c mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            pc = 0x5b311;
        }
        0x5b311 => {
            // block 5b311
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b311 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5b312 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7eca size=630
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7eca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd7eca;
    loop {
        match pc {
        0xd7eca => {
            // block d7eca
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7eca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7eca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7ecb mov rbp,rsp
            rbp = rsp;
            // d7ece push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d7ed0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7ed2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d7ed4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d7ed6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7ed7 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // d7edb mov r15,rdi
            r15 = rdi;
            // d7ede dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // d7ee0 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // d7ee3 ja 0xd7efe
            if fl.a() { pc = 0xd7efe; continue; }
            pc = 0xd7ee5;
        }
        0xd7ee5 => {
            // block d7ee5
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7ee5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7ee5 lea rax,[rip+0x238] # 0xd8124
            rax = m.env.code_va(0xd8124);
            // d7eec movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // d7ef0 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d7ef3 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd7ef5 => {
            // block d7ef5
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7ef5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7ef5 lea rax,[r15+0x344]
            rax = r15.wrapping_add(0x344);
            // d7efc jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd7efe => {
            // block d7efe
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7efe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7efe lea rax,[r15+0x340]
            rax = r15.wrapping_add(0x340);
            pc = 0xd7f05;
        }
        0xd7f05 => {
            // block d7f05
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f05 mov ebx,DWORD PTR [rax]
            rbx = (m.r32(rax) as u64) as u32 as u64;
            // d7f07 mov rax,QWORD PTR [r15+0x288]
            rax = m.r64(r15.wrapping_add(0x288)) as u64;
            // d7f0e mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // d7f12 mov eax,DWORD PTR [r15+0x298]
            rax = (m.r32(r15.wrapping_add(0x298)) as u64) as u32 as u64;
            // d7f19 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7f1c shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d7f1f cmp DWORD PTR [r15+0x370],0x1
            fl = sub::<32>(m.r32(r15.wrapping_add(0x370)) as u64, 0x1u64).1;
            // d7f27 jne 0xd7f30
            if fl.ne() { pc = 0xd7f30; continue; }
            pc = 0xd7f29;
        }
        0xd7f29 => {
            // block d7f29
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f29 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // d7f2e jmp 0xd7f32
            pc = 0xd7f32; continue;
        }
        0xd7f30 => {
            // block d7f30
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f30 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7f32;
        }
        0xd7f32 => {
            // block d7f32
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f32 mov rdi,r15
            rdi = r15;
            // d7f35 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // d7f38 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7f3d mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // d7f40 lea r14,[r15+0x378]
            r14 = r15.wrapping_add(0x378);
            // d7f47 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d7f49 je 0xd808a
            if fl.e() { pc = 0xd808a; continue; }
            pc = 0xd7f4f;
        }
        0xd7f4f => {
            // block d7f4f
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f4f mov QWORD PTR [rbp-0x50],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r14) as u64);
            // d7f53 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d7f56 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d7f59 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7f5b mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d7f5f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7f61 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xd7f65;
        }
        0xd7f65 => {
            // block d7f65
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f65 mov DWORD PTR [rbp-0x30],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rcx as u32 as u64)) as u32);
            // d7f68 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // d7f6c call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7f71 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7f73 je 0xd7f9e
            if fl.e() { pc = 0xd7f9e; continue; }
            pc = 0xd7f75;
        }
        0xd7f75 => {
            // block d7f75
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f75 cmp ebx,DWORD PTR [r15+0x338]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r15.wrapping_add(0x338)) as u64).1;
            // d7f7c jne 0xd7f9e
            if fl.ne() { pc = 0xd7f9e; continue; }
            pc = 0xd7f7e;
        }
        0xd7f7e => {
            // block d7f7e
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f7e cmp DWORD PTR [r15+0x370],0x1
            fl = sub::<32>(m.r32(r15.wrapping_add(0x370)) as u64, 0x1u64).1;
            // d7f86 jne 0xd7f8f
            if fl.ne() { pc = 0xd7f8f; continue; }
            pc = 0xd7f88;
        }
        0xd7f88 => {
            // block d7f88
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f88 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d7f8d jmp 0xd7f94
            pc = 0xd7f94; continue;
        }
        0xd7f8f => {
            // block d7f8f
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f8f mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            pc = 0xd7f94;
        }
        0xd7f94 => {
            // block d7f94
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f94 mov rdi,r15
            rdi = r15;
            // d7f97 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7f9c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7f9e;
        }
        0xd7f9e => {
            // block d7f9e
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f9e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7fa1 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d7fa6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7fab xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7fae mov rdi,r15
            rdi = r15;
            // d7fb1 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d7fb3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7fb6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7fb9 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7fbe mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d7fc3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7fc8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7fcb mov rdi,r15
            rdi = r15;
            // d7fce call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7fd1 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7fd4 mov rdi,r15
            rdi = r15;
            // d7fd7 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7fdd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7fe0 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d7fe5 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7fea xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7fed mov rdi,r15
            rdi = r15;
            // d7ff0 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // d7ff4 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ff7 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // d7ffb mov edx,DWORD PTR [rax+r12*4]
            rdx = (m.r32(rax.wrapping_add(r12.wrapping_mul(4))) as u64) as u32 as u64;
            // d7fff mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // d8002 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // d8004 jne 0xd800b
            if fl.ne() { pc = 0xd800b; continue; }
            pc = 0xd8006;
        }
        0xd8006 => {
            // block d8006
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d8006 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8006 mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d8009 jmp 0xd802e
            pc = 0xd802e; continue;
        }
        0xd800b => {
            // block d800b
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d800b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d800b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d800d lea rdi,[rip+0x60dcf] # 0x138de3
            rdi = m.env.data_va(316848);
            // d8014 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // d8017 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d801c mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d8020 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d8022 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // d8026 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d8029 mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d802c ja 0xd8084
            if fl.a() { pc = 0xd8084; continue; }
            pc = 0xd802e;
        }
        0xd802e => {
            // block d802e
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d802e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d802e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8030 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // d8033 div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // d8035 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d8039 cmp BYTE PTR [r14],0x0
            fl = sub::<8>(m.r8(r14) as u64, 0x0u64).1;
            // d803d jne 0xd806a
            if fl.ne() { pc = 0xd806a; continue; }
            pc = 0xd803f;
        }
        0xd803f => {
            // block d803f
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d803f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d803f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8041 lea rdi,[rip+0x60db5] # 0x138dfd
            rdi = m.env.data_va(316874);
            // d8048 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d804c mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // d804f call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8054 mov rax,QWORD PTR [rip+0x68385] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // d805b mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // d805e call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8063 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // d8066 mov r14,QWORD PTR [rbp-0x50]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0xd806a;
        }
        0xd806a => {
            // block d806a
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d806a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d806a add ebx,0x100
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x100u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d8070 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // d8074 add r13d,0x1900
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x1900u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d807b cmp r12d,ecx
            fl = sub::<32>((r12 as u32 as u64), (rcx as u32 as u64)).1;
            // d807e jb 0xd7f65
            if fl.b() { pc = 0xd7f65; continue; }
            pc = 0xd8084;
        }
        0xd8084 => {
            // block d8084
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d8084 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8084 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d8088 jmp 0xd8092
            pc = 0xd8092; continue;
        }
        0xd808a => {
            // block d808a
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d808a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d808a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d808c mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d8090 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd8092;
        }
        0xd8092 => {
            // block d8092
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d8092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8092 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d8094 mov rdi,r15
            rdi = r15;
            // d8097 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d809c test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // d809e mov al,BYTE PTR [r14]
            rax = (rax & !0xff) | ((m.r8(r14) as u64) as u8 as u64);
            // d80a1 je 0xd80ab
            if fl.e() { pc = 0xd80ab; continue; }
            pc = 0xd80a3;
        }
        0xd80a3 => {
            // block d80a3
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80a3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d80a5 je 0xd80c1
            if fl.e() { pc = 0xd80c1; continue; }
            pc = 0xd80a7;
        }
        0xd80a7 => {
            // block d80a7
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80a7 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d80a9 jmp 0xd80d5
            pc = 0xd80d5; continue;
        }
        0xd80ab => {
            // block d80ab
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80ab mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // d80ad test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d80af jne 0xd80d5
            if fl.ne() { pc = 0xd80d5; continue; }
            pc = 0xd80b1;
        }
        0xd80b1 => {
            // block d80b1
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80b1 lea rdi,[rip+0x60d78] # 0x138e30
            rdi = m.env.data_va(316925);
            // d80b8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d80ba call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d80bf jmp 0xd80d5
            pc = 0xd80d5; continue;
        }
        0xd80c1 => {
            // block d80c1
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80c1 lea rdi,[rip+0x60d4b] # 0x138e13
            rdi = m.env.data_va(316896);
            // d80c8 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d80ca xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d80cc mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d80d0 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd80d5;
        }
        0xd80d5 => {
            // block d80d5
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80d5 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d80d7 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // d80db pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d80dc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d80de pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d80e0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d80e2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d80e4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d80e5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd80e6 => {
            // block d80e6
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80e6 lea rax,[r15+0x354]
            rax = r15.wrapping_add(0x354);
            // d80ed jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd80f2 => {
            // block d80f2
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80f2 lea rax,[r15+0x358]
            rax = r15.wrapping_add(0x358);
            // d80f9 jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd80fe => {
            // block d80fe
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80fe lea rax,[r15+0x348]
            rax = r15.wrapping_add(0x348);
            // d8105 jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd810a => {
            // block d810a
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d810a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d810a lea rax,[r15+0x34c]
            rax = r15.wrapping_add(0x34c);
            // d8111 jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd8116 => {
            // block d8116
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d8116 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8116 lea rax,[r15+0x350]
            rax = r15.wrapping_add(0x350);
            // d811d jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9b24 size=604
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949, stub:0x961
pub fn f_000e9b24<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9b24;
    loop {
        match pc {
        0xe9b24 => {
            // block e9b24
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b24 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9b25 mov rbp,rsp
            rbp = rsp;
            // e9b28 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9b2a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9b2c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9b2e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9b30 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9b31 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9b38 mov r15,rdi
            r15 = rdi;
            // e9b3b mov rbx,QWORD PTR [rip+0x56896] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e9b42 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e9b45 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // e9b49 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e9b4c cmp QWORD PTR [r15+0x78],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x78)) as u64, 0x0u64).1;
            // e9b51 je 0xe9d35
            if fl.e() { pc = 0xe9d35; continue; }
            pc = 0xe9b57;
        }
        0xe9b57 => {
            // block e9b57
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b57 mov rax,QWORD PTR [r15+0x80]
            rax = m.r64(r15.wrapping_add(0x80)) as u64;
            // e9b5e test rax,rax
            fl = logic::<64>(rax & rax);
            // e9b61 je 0xe9d55
            if fl.e() { pc = 0xe9d55; continue; }
            pc = 0xe9b67;
        }
        0xe9b67 => {
            // block e9b67
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b67 mov ecx,DWORD PTR [r15+0x18c]
            rcx = (m.r32(r15.wrapping_add(0x18c)) as u64) as u32 as u64;
            // e9b6e test cl,0x10
            fl = logic::<8>((rcx as u8 as u64) & 0x10u64);
            // e9b71 jne 0xe9bad
            if fl.ne() { pc = 0xe9bad; continue; }
            pc = 0xe9b73;
        }
        0xe9b73 => {
            // block e9b73
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b73 test cl,0x8
            fl = logic::<8>((rcx as u8 as u64) & 0x8u64);
            // e9b76 je 0xe9d32
            if fl.e() { pc = 0xe9d32; continue; }
            pc = 0xe9b7c;
        }
        0xe9b7c => {
            // block e9b7c
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b7c lea rsi,[r15+0x108]
            rsi = r15.wrapping_add(0x108);
            // e9b83 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e9b8a mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e9b8f rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            // e9b92 cmp BYTE PTR [r15+0x192],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x192)) as u64, 0x0u64).1;
            // e9b9a je 0xe9c42
            if fl.e() { pc = 0xe9c42; continue; }
            pc = 0xe9ba0;
        }
        0xe9ba0 => {
            // block e9ba0
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9ba0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ba0 mov r14,QWORD PTR [r15+0x20]
            r14 = m.r64(r15.wrapping_add(0x20)) as u64;
            // e9ba4 sub r14,QWORD PTR [r15+0x18]
            { let (r, f) = sub::<64>(r14, m.r64(r15.wrapping_add(0x18)) as u64);
              r14 = r;
              fl = f; }
            // e9ba8 jmp 0xe9cc5
            pc = 0xe9cc5; continue;
        }
        0xe9bad => {
            // block e9bad
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9bad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9bad mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // e9bb1 cmp rax,QWORD PTR [r15+0x28]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x28)) as u64).1;
            // e9bb5 je 0xe9bd4
            if fl.e() { pc = 0xe9bd4; continue; }
            pc = 0xe9bb7;
        }
        0xe9bb7 => {
            // block e9bb7
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9bb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9bb7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e9bba mov r13d,0xffffffff
            r13 = (0xffffffffu64) as u32 as u64;
            // e9bc0 mov esi,0xffffffff
            rsi = (0xffffffffu64) as u32 as u64;
            // e9bc5 mov rdi,r15
            rdi = r15;
            // e9bc8 call QWORD PTR [rax+0x68]
            { let r = { let t = m.r64(rax.wrapping_add(0x68)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9bcb cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // e9bce je 0xe9d35
            if fl.e() { pc = 0xe9d35; continue; }
            pc = 0xe9bd4;
        }
        0xe9bd4 => {
            // block e9bd4
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9bd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9bd4 lea r14,[r15+0x88]
            r14 = r15.wrapping_add(0x88);
            // e9bdb mov r13d,0xffffffff
            r13 = (0xffffffffu64) as u32 as u64;
            pc = 0xe9be1;
        }
        0xe9be1 => {
            // block e9be1
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9be1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9be1 mov rdx,QWORD PTR [r15+0x40]
            rdx = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9be5 mov rdi,QWORD PTR [r15+0x80]
            rdi = m.r64(r15.wrapping_add(0x80)) as u64;
            // e9bec mov rcx,QWORD PTR [r15+0x60]
            rcx = m.r64(r15.wrapping_add(0x60)) as u64;
            // e9bf0 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // e9bf3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e9bf6 mov rsi,r14
            rsi = r14;
            // e9bf9 lea r8,[rbp-0xb8]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // e9c00 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c03 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e9c05 mov r12,QWORD PTR [rbp-0xb8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // e9c0c mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9c10 mov rcx,QWORD PTR [r15+0x78]
            rcx = m.r64(r15.wrapping_add(0x78)) as u64;
            // e9c14 sub r12,rdi
            { let (r, f) = sub::<64>(r12, rdi);
              r12 = r;
              fl = f; }
            // e9c17 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e9c1c mov rdx,r12
            rdx = r12;
            // e9c1f call 0xec1e8
            { let r = { let t = m.env.stub_va(0x9b5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c24 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // e9c27 jne 0xe9c33
            if fl.ne() { pc = 0xe9c33; continue; }
            pc = 0xe9c29;
        }
        0xe9c29 => {
            // block e9c29
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c29 cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // e9c2c je 0xe9be1
            if fl.e() { pc = 0xe9be1; continue; }
            pc = 0xe9c2e;
        }
        0xe9c2e => {
            // block e9c2e
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c2e cmp ebx,0x2
            fl = sub::<32>((rbx as u32 as u64), 0x2u64).1;
            // e9c31 jne 0xe9c6d
            if fl.ne() { pc = 0xe9c6d; continue; }
            pc = 0xe9c33;
        }
        0xe9c33 => {
            // block e9c33
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c33 mov rbx,QWORD PTR [rip+0x5679e] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e9c3a mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e9c3d jmp 0xe9d35
            pc = 0xe9d35; continue;
        }
        0xe9c42 => {
            // block e9c42
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c42 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e9c45 mov rdi,rax
            rdi = rax;
            // e9c48 call QWORD PTR [rcx+0x30]
            { let r = { let t = m.r64(rcx.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c4b mov rcx,QWORD PTR [r15+0x48]
            rcx = m.r64(r15.wrapping_add(0x48)) as u64;
            // e9c4f mov r14,QWORD PTR [r15+0x50]
            r14 = m.r64(r15.wrapping_add(0x50)) as u64;
            // e9c53 sub r14,rcx
            { let (r, f) = sub::<64>(r14, rcx);
              r14 = r;
              fl = f; }
            // e9c56 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9c58 jle 0xe9c8d
            if fl.le() { pc = 0xe9c8d; continue; }
            pc = 0xe9c5a;
        }
        0xe9c5a => {
            // block e9c5a
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c5a cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e9c5c mov rcx,QWORD PTR [r15+0x20]
            rcx = m.r64(r15.wrapping_add(0x20)) as u64;
            // e9c60 sub rcx,QWORD PTR [r15+0x18]
            { let (r, f) = sub::<64>(rcx, m.r64(r15.wrapping_add(0x18)) as u64);
              rcx = r;
              fl = f; }
            // e9c64 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e9c68 add r14,rcx
            { let (r, f) = add::<64>(r14, rcx);
              r14 = r;
              fl = f; }
            // e9c6b jmp 0xe9cc5
            pc = 0xe9cc5; continue;
        }
        0xe9c6d => {
            // block e9c6d
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c6d mov rdi,QWORD PTR [r15+0x78]
            rdi = m.r64(r15.wrapping_add(0x78)) as u64;
            // e9c71 call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c76 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9c78 mov rbx,QWORD PTR [rip+0x56759] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e9c7f mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e9c82 jne 0xe9d35
            if fl.ne() { pc = 0xe9d35; continue; }
            pc = 0xe9c88;
        }
        0xe9c88 => {
            // block e9c88
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c88 jmp 0xe9d32
            pc = 0xe9d32; continue;
        }
        0xe9c8d => {
            // block e9c8d
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c8d mov r8,QWORD PTR [r15+0x18]
            r8 = m.r64(r15.wrapping_add(0x18)) as u64;
            // e9c91 cmp r8,QWORD PTR [r15+0x20]
            fl = sub::<64>(r8, m.r64(r15.wrapping_add(0x20)) as u64).1;
            // e9c95 je 0xe9cc5
            if fl.e() { pc = 0xe9cc5; continue; }
            pc = 0xe9c97;
        }
        0xe9c97 => {
            // block e9c97
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c97 mov rdx,QWORD PTR [r15+0x40]
            rdx = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9c9b mov rdi,QWORD PTR [r15+0x80]
            rdi = m.r64(r15.wrapping_add(0x80)) as u64;
            // e9ca2 sub r8,QWORD PTR [r15+0x10]
            { let (r, f) = sub::<64>(r8, m.r64(r15.wrapping_add(0x10)) as u64);
              r8 = r;
              fl = f; }
            // e9ca6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e9ca9 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e9cb0 call QWORD PTR [rax+0x40]
            { let r = { let t = m.r64(rax.wrapping_add(0x40)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9cb3 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e9cb5 sub r14,rax
            { let (r, f) = sub::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // e9cb8 add r14,QWORD PTR [r15+0x48]
            { let (r, f) = add::<64>(r14, m.r64(r15.wrapping_add(0x48)) as u64);
              r14 = r;
              fl = f; }
            // e9cbc sub r14,QWORD PTR [r15+0x40]
            { let (r, f) = sub::<64>(r14, m.r64(r15.wrapping_add(0x40)) as u64);
              r14 = r;
              fl = f; }
            // e9cc0 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // e9cc3 jmp 0xe9cc8
            pc = 0xe9cc8; continue;
        }
        0xe9cc5 => {
            // block e9cc5
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9cc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9cc5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe9cc8;
        }
        0xe9cc8 => {
            // block e9cc8
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9cc8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9cc8 mov rdi,QWORD PTR [r15+0x78]
            rdi = m.r64(r15.wrapping_add(0x78)) as u64;
            // e9ccc neg r14
            { let (r, f) = neg::<64>(r14); r14 = r; fl = f; }
            // e9ccf mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e9cd4 mov rsi,r14
            rsi = r14;
            // e9cd7 call 0xec1dc
            { let r = { let t = m.env.stub_va(0x9a9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9cdc test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9cde je 0xe9ce8
            if fl.e() { pc = 0xe9ce8; continue; }
            pc = 0xe9ce0;
        }
        0xe9ce0 => {
            // block e9ce0
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9ce0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ce0 mov r13d,0xffffffff
            r13 = (0xffffffffu64) as u32 as u64;
            // e9ce6 jmp 0xe9d35
            pc = 0xe9d35; continue;
        }
        0xe9ce8 => {
            // block e9ce8
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9ce8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ce8 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // e9ceb je 0xe9d03
            if fl.e() { pc = 0xe9d03; continue; }
            pc = 0xe9ced;
        }
        0xe9ced => {
            // block e9ced
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9ced rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ced lea rdi,[r15+0x88]
            rdi = r15.wrapping_add(0x88);
            // e9cf4 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e9cfb mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e9d00 rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            pc = 0xe9d03;
        }
        0xe9d03 => {
            // block e9d03
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d03 mov rax,QWORD PTR [r15+0x40]
            rax = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9d07 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // e9d0b mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // e9d0f mov DWORD PTR [r15+0x18c],0x0
            m.w32(r15.wrapping_add(0x18c), (0x0u64) as u32);
            // e9d1a mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            // e9d22 mov QWORD PTR [r15+0x18],0x0
            m.w64(r15.wrapping_add(0x18), (0x0u64) as u64);
            // e9d2a mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            pc = 0xe9d32;
        }
        0xe9d32 => {
            // block e9d32
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d32 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe9d35;
        }
        0xe9d35 => {
            // block e9d35
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d35 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // e9d39 jne 0xe9d50
            if fl.ne() { pc = 0xe9d50; continue; }
            pc = 0xe9d3b;
        }
        0xe9d3b => {
            // block e9d3b
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d3b mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // e9d3e add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9d45 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9d46 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9d48 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9d4a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9d4c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e9d4e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9d4f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe9d50 => {
            // block e9d50
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d50 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0xe9d55 => {
            // block e9d55
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d55 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // e9d5a call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9d5f mov rbx,rax
            rbx = rax;
            // e9d62 mov rdi,rbx
            rdi = rbx;
            // e9d65 call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9d6a mov rsi,QWORD PTR [rip+0x5646f] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // e9d71 mov rdx,QWORD PTR [rip+0x56430] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // e9d78 mov rdi,rbx
            rdi = rbx;
            // e9d7b call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe171e size=592
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e171e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe171e;
    loop {
        match pc {
        0xe171e => {
            // block e171e
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e171e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e171e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e171f mov rbp,rsp
            rbp = rsp;
            // e1722 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e1724 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1726 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e1728 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e172a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e172b sub rsp,0x138
            { let (r, f) = sub::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // e1732 mov rbx,rdx
            rbx = rdx;
            // e1735 mov r13,rsi
            r13 = rsi;
            // e1738 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e173f lea r14,[rbp-0x128]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e1746 mov rax,QWORD PTR [rip+0x5ea7b] # 0x1401c8
            rax = m.r64(m.env.data_va(346517)) as u64;
            // e174d lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // e1751 mov QWORD PTR [rbp-0x138],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rcx) as u64);
            // e1758 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // e175c mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // e1763 mov QWORD PTR [rbp-0x130],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x0u64) as u64);
            // e176e mov rsi,r14
            rsi = r14;
            // e1771 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1776 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // e177e mov DWORD PTR [rbp-0x30],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0xffffffffu64) as u32);
            // e1785 mov r12,QWORD PTR [rip+0x5ec2c] # 0x1403b8
            r12 = m.r64(m.env.data_va(347013)) as u64;
            // e178c lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // e1791 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // e1798 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // e179f add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // e17a3 mov QWORD PTR [rbp-0xc0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r12) as u64);
            // e17aa mov rdi,r14
            rdi = r14;
            // e17ad call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e17b2 mov r15,QWORD PTR [rip+0x5ebef] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // e17b9 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // e17bd mov QWORD PTR [rbp-0x128],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (r15) as u64);
            // e17c4 mov QWORD PTR [rbp-0xd0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (0x0u64) as u64);
            // e17cf mov QWORD PTR [rbp-0xd8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (0x0u64) as u64);
            // e17da mov QWORD PTR [rbp-0xe0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x0u64) as u64);
            // e17e5 mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // e17f0 mov DWORD PTR [rbp-0xc8],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (0x8u64) as u32);
            // e17fa mov rdi,r14
            rdi = r14;
            // e17fd mov rsi,r13
            rsi = r13;
            // e1800 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1805 xorps xmm0,xmm0
            x0 = 0;
            // e1808 movaps XMMWORD PTR [rbp-0x150],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_feb0), x0);
            // e180f mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // e181a lea r13,[rbp-0x138]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // e1821 lea r14,[rbp-0x150]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // e1828 jmp 0xe182f
            pc = 0xe182f; continue;
        }
        0xe182a => {
            // block e182a
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e182a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e182a add QWORD PTR [rbx+0x8],0x18
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, 0x18u64);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            pc = 0xe182f;
        }
        0xe182f => {
            // block e182f
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e182f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e182f mov edx,0x2c
            rdx = (0x2cu64) as u32 as u64;
            // e1834 mov rdi,r13
            rdi = r13;
            // e1837 mov rsi,r14
            rsi = r14;
            // e183a call 0xebf00
            { let r = { let t = m.env.stub_va(0x6cd); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e183f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e1842 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e1846 test BYTE PTR [rcx+rax*1+0x20],0x5
            fl = logic::<8>(m.r8(rcx.wrapping_add(rax).wrapping_add(0x20)) as u64 & 0x5u64);
            // e184b jne 0xe18e0
            if fl.ne() { pc = 0xe18e0; continue; }
            pc = 0xe1851;
        }
        0xe1851 => {
            // block e1851
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1851 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e1855 cmp rdi,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rdi, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // e1859 je 0xe1865
            if fl.e() { pc = 0xe1865; continue; }
            pc = 0xe185b;
        }
        0xe185b => {
            // block e185b
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e185b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e185b mov rsi,r14
            rsi = r14;
            // e185e call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1863 jmp 0xe182a
            pc = 0xe182a; continue;
        }
        0xe1865 => {
            // block e1865
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1865 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1865 mov rdi,rbx
            rdi = rbx;
            // e1868 mov rsi,r14
            rsi = r14;
            // e186b call 0xebefa
            { let r = { let t = m.env.stub_va(0x6c7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1870 jmp 0xe182f
            pc = 0xe182f; continue;
        }
        0xe1872 => {
            // block e1872
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1872 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1872 mov rbx,rax
            rbx = rax;
            // e1875 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // e187c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1881 mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // e1888 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // e188f mov QWORD PTR [rbp-0xc0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r12) as u64);
            // e1896 mov QWORD PTR [rbp-0x128],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (r15) as u64);
            pc = 0xe189d;
        }
        0xe189d => {
            // block e189d
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e189d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e189d lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // e18a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18a9 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e18b0 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe18b5;
        }
        0xe18b5 => {
            // block e18b5
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18b5 mov rsi,QWORD PTR [rip+0x5e944] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // e18bc add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e18c0 lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // e18c7 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe18cc;
        }
        0xe18cc => {
            // block e18cc
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18cc lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e18d3 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18d8 mov rdi,rbx
            rdi = rbx;
            // e18db call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe18e0;
        }
        0xe18e0 => {
            // block e18e0
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18e0 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // e18e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18ec mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // e18f3 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // e18fa mov QWORD PTR [rbp-0xc0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r12) as u64);
            // e1901 mov QWORD PTR [rbp-0x128],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (r15) as u64);
            // e1908 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // e190f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1914 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e191b call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1920 mov rsi,QWORD PTR [rip+0x5e8d9] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // e1927 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e192b lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // e1932 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1937 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e193e call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1943 add rsp,0x138
            { let (r, f) = add::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // e194a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e194b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e194d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e194f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e1951 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e1953 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1954 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe1955 => {
            // block e1955
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1955 mov rbx,rax
            rbx = rax;
            // e1958 jmp 0xe18cc
            pc = 0xe18cc; continue;
        }
        0xe195d => {
            // block e195d
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e195d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e195d mov rbx,rax
            rbx = rax;
            // e1960 jmp 0xe18b5
            pc = 0xe18b5; continue;
        }
        0xe1965 => {
            // block e1965
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1965 mov rbx,rax
            rbx = rax;
            // e1968 jmp 0xe189d
            pc = 0xe189d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x154a4 size=588
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000154a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x154a4;
    loop {
        match pc {
        0x154a4 => {
            // block 154a4
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 154a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 154a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 154a5 mov rbp,rsp
            rbp = rsp;
            // 154a8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 154aa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 154ac push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 154ae push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 154b0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 154b1 sub rsp,0xd8
            { let (r, f) = sub::<64>(rsp, 0xd8u64);
              rsp = r;
              fl = f; }
            // 154b8 mov rax,QWORD PTR [rip+0x12ad61] # 0x140220
            rax = m.r64(m.env.data_va(346605)) as u64;
            // 154bf add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 154c3 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 154c6 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 154ca call 0x63dd3
            { let r = { let t = m.env.code_va(0x63dd3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 154cf mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 154d3 lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 154d7 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 154da je 0x1556c
            if fl.e() { pc = 0x1556c; continue; }
            pc = 0x154e0;
        }
        0x154e0 => {
            // block 154e0
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 154e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 154e0 lea r15,[rbp-0x80]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 154e4 lea r14,[rip+0x1ac515] # 0x1c1a00
            r14 = m.env.data_va(877005);
            // 154eb lea r13,[rbp-0x60]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            pc = 0x154ef;
        }
        0x154ef => {
            // block 154ef
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 154ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 154ef mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 154f2 mov rdi,r15
            rdi = r15;
            // 154f5 call 0x147d7
            { let r = { let t = m.env.code_va(0x147d7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 154fa mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 154fe mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 15502 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 15506 mov rcx,QWORD PTR [rbp-0x78]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1550a mov QWORD PTR [rbp-0x58],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rcx) as u64);
            // 1550e mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 15512 xorps xmm0,xmm0
            x0 = 0;
            // 15515 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // 15519 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 15521 mov eax,DWORD PTR [rbx+0x1c]
            rax = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 15524 mov DWORD PTR [rbp-0x48],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((rax as u32 as u64)) as u32);
            // 15527 mov rdi,r14
            rdi = r14;
            // 1552a mov rsi,r13
            rsi = r13;
            // 1552d call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15532 mov rdi,r13
            rdi = r13;
            // 15535 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1553a mov rdi,r15
            rdi = r15;
            // 1553d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15542 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 15546 test rax,rax
            fl = logic::<64>(rax & rax);
            // 15549 je 0x15558
            if fl.e() { pc = 0x15558; continue; }
            pc = 0x1554b;
        }
        0x1554b => {
            // block 1554b
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 1554b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1554b mov rbx,rax
            rbx = rax;
            // 1554e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 15551 test rax,rax
            fl = logic::<64>(rax & rax);
            // 15554 jne 0x1554b
            if fl.ne() { pc = 0x1554b; continue; }
            pc = 0x15556;
        }
        0x15556 => {
            // block 15556
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 15556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15556 jmp 0x15567
            pc = 0x15567; continue;
        }
        0x15558 => {
            // block 15558
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 15558 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15558 mov rax,rbx
            rax = rbx;
            pc = 0x1555b;
        }
        0x1555b => {
            // block 1555b
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 1555b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1555b mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 1555f cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 15562 mov rax,rbx
            rax = rbx;
            // 15565 jne 0x1555b
            if fl.ne() { pc = 0x1555b; continue; }
            pc = 0x15567;
        }
        0x15567 => {
            // block 15567
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 15567 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15567 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 1556a jne 0x154ef
            if fl.ne() { pc = 0x154ef; continue; }
            pc = 0x1556c;
        }
        0x1556c => {
            // block 1556c
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 1556c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1556c lea rsi,[rip+0x114bd0] # 0x12a143
            rsi = m.env.data_va(256272);
            // 15573 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 1557a mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 1557f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15584 mov DWORD PTR [rbp-0x88],0x10322950
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0x10322950u64) as u32);
            // 1558e lea rdi,[rip+0x1ac46b] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 15595 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 1559c call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155a1 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 155a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155ad lea rsi,[rip+0x114b9c] # 0x12a150
            rsi = m.env.data_va(256285);
            // 155b4 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 155bb mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 155c0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155c5 mov DWORD PTR [rbp-0xa8],0x10294700
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x10294700u64) as u32);
            // 155cf lea rdi,[rip+0x1ac42a] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 155d6 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 155dd call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155e2 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 155e9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155ee lea rsi,[rip+0x114b63] # 0x12a158
            rsi = m.env.data_va(256293);
            // 155f5 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 155fc mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 15601 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15606 mov DWORD PTR [rbp-0xc8],0x10538200
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (0x10538200u64) as u32);
            // 15610 lea rdi,[rip+0x1ac3e9] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 15617 lea rsi,[rbp-0xe0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 1561e call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15623 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 1562a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1562f lea rsi,[rip+0x114b38] # 0x12a16e
            rsi = m.env.data_va(256315);
            // 15636 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 1563d mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 15642 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15647 mov DWORD PTR [rbp-0xe8],0x10668200
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x10668200u64) as u32);
            // 15651 lea rdi,[rip+0x1ac3a8] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 15658 lea rsi,[rbp-0x100]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 1565f call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15664 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 1566b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15670 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15674 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 15678 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1567d add rsp,0xd8
            { let (r, f) = add::<64>(rsp, 0xd8u64);
              rsp = r;
              fl = f; }
            // 15684 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15685 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15687 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15689 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1568b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1568d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1568e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1568f => {
            // block 1568f
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 1568f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1568f jmp 0x156a8
            pc = 0x156a8; continue;
        }
        0x15691 => {
            // block 15691
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 15691 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15691 mov rbx,rax
            rbx = rax;
            // 15694 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 15698 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1569d lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            pc = 0x156a1;
        }
        0x156a1 => {
            // block 156a1
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 156a6 jmp 0x156ab
            pc = 0x156ab; continue;
        }
        0x156a8 => {
            // block 156a8
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156a8 mov rbx,rax
            rbx = rax;
            pc = 0x156ab;
        }
        0x156ab => {
            // block 156ab
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156ab mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 156af lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 156b3 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 156b8 mov rdi,rbx
            rdi = rbx;
            // 156bb call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 156c0 mov rbx,rax
            rbx = rax;
            // 156c3 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 156ca jmp 0x156a1
            pc = 0x156a1; continue;
        }
        0x156cc => {
            // block 156cc
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156cc mov rbx,rax
            rbx = rax;
            // 156cf lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 156d6 jmp 0x156a1
            pc = 0x156a1; continue;
        }
        0x156d8 => {
            // block 156d8
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156d8 mov rbx,rax
            rbx = rax;
            // 156db lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 156e2 jmp 0x156a1
            pc = 0x156a1; continue;
        }
        0x156e4 => {
            // block 156e4
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156e4 mov rbx,rax
            rbx = rax;
            // 156e7 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 156ee jmp 0x156a1
            pc = 0x156a1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5be20 size=537
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0005be20<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x5be20;
    loop {
        match pc {
        0x5be20 => {
            // block 5be20
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5be20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5be20 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5be21 mov rbp,rsp
            rbp = rsp;
            // 5be24 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5be26 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5be28 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 5be2a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5be2c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5be2d sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 5be34 mov QWORD PTR [rbp-0x160],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rcx) as u64);
            // 5be3b mov DWORD PTR [rbp-0x158],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea8), ((rdx as u32 as u64)) as u32);
            // 5be41 mov QWORD PTR [rbp-0x168],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rsi) as u64);
            // 5be48 mov r13,rdi
            r13 = rdi;
            // 5be4b mov edx,DWORD PTR [rbp+0x18]
            rdx = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 5be4e mov rax,QWORD PTR [rip+0xe4583] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // 5be55 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 5be58 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 5be5c lea ecx,[r9-0x1]
            rcx = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5be60 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5be65 shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5be67 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 5be69 je 0x5bead
            if fl.e() { pc = 0x5bead; continue; }
            pc = 0x5be6b;
        }
        0x5be6b => {
            // block 5be6b
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5be6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5be6b mov rcx,QWORD PTR [rbp-0x168]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 5be72 mov ecx,DWORD PTR [rcx]
            rcx = (m.r32(rcx) as u64) as u32 as u64;
            // 5be74 cvtsi2sd xmm1,rcx
            x1 = set_lo64(x1, (sx::<64>(rcx) as f64).to_bits());
            // 5be79 lea rcx,[rbp-0x130]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 5be80 lea rdx,[rbp-0xb0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 5be87 mov rsi,r8
            rsi = r8;
            // 5be8a mov edi,DWORD PTR [rbp+0x18]
            rdi = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            pc = 0x5be8d;
        }
        0x5be8d => {
            // block 5be8d
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5be8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5be8d movapd xmm2,xmm0
            x2 = x0;
            // 5be91 divsd xmm2,QWORD PTR [rsi]
            x2 = set_lo64(x2, (f64_of(x2) / f64_of((m.r64(rsi) as u128))).to_bits());
            // 5be95 movsd QWORD PTR [rcx],xmm2
            m.w64(rcx, x2 as u64);
            // 5be99 movsd QWORD PTR [rdx],xmm1
            m.w64(rdx, x1 as u64);
            // 5be9d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 5bea1 add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 5bea5 add rdx,0x8
            { let (r, f) = add::<64>(rdx, 0x8u64);
              rdx = r;
              fl = f; }
            // 5bea9 dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 5beab jne 0x5be8d
            if fl.ne() { pc = 0x5be8d; continue; }
            pc = 0x5bead;
        }
        0x5bead => {
            // block 5bead
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bead rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bead test r13,r13
            fl = logic::<64>(r13 & r13);
            // 5beb0 sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5beb3 cmp QWORD PTR [rbp-0x160],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64, 0x0u64).1;
            // 5bebb sete dl
            rdx = (rdx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5bebe test r8,r8
            fl = logic::<64>(r8 & r8);
            // 5bec1 sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5bec4 cmp DWORD PTR [rbp-0x158],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64, 0x0u64).1;
            // 5becb je 0x5c003
            if fl.e() { pc = 0x5c003; continue; }
            pc = 0x5bed1;
        }
        0x5bed1 => {
            // block 5bed1
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bed1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bed1 or cl,dl
            { let r = (rcx as u8 as u64) | (rdx as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 5bed3 or cl,bl
            { let r = (rcx as u8 as u64) | (rbx as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 5bed5 jne 0x5c003
            if fl.ne() { pc = 0x5c003; continue; }
            pc = 0x5bedb;
        }
        0x5bedb => {
            // block 5bedb
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bedb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bedb cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 5bee0 addsd xmm0,QWORD PTR [rip+0xa48a0] # 0x100788
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(85845)) as u128))).to_bits());
            // 5bee8 movsd QWORD PTR [rbp-0x148],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), x0 as u64);
            // 5bef0 mov eax,DWORD PTR [rbp+0x18]
            rax = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 5bef3 lea eax,[rax-0x1]
            rax = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5bef6 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 5bef9 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 5bf00 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0x5bf02;
        }
        0x5bf02 => {
            // block 5bf02
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bf02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bf02 mov DWORD PTR [rbp-0x154],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feac), ((rcx as u32 as u64)) as u32);
            // 5bf08 mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // 5bf0f mov eax,DWORD PTR [rbp+0x18]
            rax = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 5bf12 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 5bf14 mov r14,QWORD PTR [rbp-0x160]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 5bf1b lea r12,[rbp-0x130]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 5bf22 lea rbx,[rbp-0xb0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 5bf29 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // 5bf2c je 0x5bfdf
            if fl.e() { pc = 0x5bfdf; continue; }
            pc = 0x5bf32;
        }
        0x5bf32 => {
            // block 5bf32
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bf32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bf32 movsd xmm0,QWORD PTR [rbx]
            x0 = m.r64(rbx) as u128;
            // 5bf36 movsd QWORD PTR [rbp-0x138],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), x0 as u64);
            // 5bf3e movsd xmm1,QWORD PTR [r12]
            x1 = m.r64(r12) as u128;
            // 5bf44 movsd QWORD PTR [rbp-0x140],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), x1 as u64);
            // 5bf4c divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 5bf50 mulsd xmm0,QWORD PTR [rip+0xa4838] # 0x100790
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85853)) as u128))).to_bits());
            // 5bf58 call 0xec290
            { let r = { let t = m.env.stub_va(0xa5d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[x0 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 5bf5d mulsd xmm0,QWORD PTR [r14]
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(r14) as u128))).to_bits());
            // 5bf62 cvtsd2ss xmm0,xmm0
            x0 = set_lo32(x0, (f64_of(x0) as f32).to_bits());
            // 5bf66 cvtss2sd xmm0,xmm0
            x0 = set_lo64(x0, (f32_of(x0) as f64).to_bits());
            // 5bf6a mulsd xmm0,QWORD PTR [rbp-0x148]
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u128))).to_bits());
            // 5bf72 addsd xmm0,QWORD PTR [rip+0xa295e] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 5bf7a cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 5bf7f cmp BYTE PTR [rbp+0x10],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0x10)) as u64, 0x0u64).1;
            // 5bf83 je 0x5bf87
            if fl.e() { pc = 0x5bf87; continue; }
            pc = 0x5bf85;
        }
        0x5bf85 => {
            // block 5bf85
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bf85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bf85 bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            pc = 0x5bf87;
        }
        0x5bf87 => {
            // block 5bf87
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bf87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bf87 mov DWORD PTR [r13+0x0],eax
            m.w32(r13, ((rax as u32 as u64)) as u32);
            // 5bf8b movsd xmm0,QWORD PTR [rbp-0x138]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u128;
            // 5bf93 addsd xmm0,QWORD PTR [rip+0xa3bed] # 0xffb88
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(82773)) as u128))).to_bits());
            // 5bf9b movsd xmm1,QWORD PTR [rbp-0x140]
            x1 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u128;
            // 5bfa3 ucomisd xmm0,xmm1
            fl = ucomi(f64_of(x0), f64_of(x1));
            // 5bfa7 jbe 0x5bfad
            if fl.be() { pc = 0x5bfad; continue; }
            pc = 0x5bfa9;
        }
        0x5bfa9 => {
            // block 5bfa9
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfa9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfa9 subsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) - f64_of(x1)).to_bits());
            pc = 0x5bfad;
        }
        0x5bfad => {
            // block 5bfad
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfad add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 5bfb1 movsd QWORD PTR [rbx],xmm0
            m.w64(rbx, x0 as u64);
            // 5bfb5 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 5bfb9 add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 5bfbd add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 5bfc1 dec r15d
            { let (r, f) = dec::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 5bfc4 jne 0x5bf32
            if fl.ne() { pc = 0x5bf32; continue; }
            pc = 0x5bfca;
        }
        0x5bfca => {
            // block 5bfca
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfca mov r13,QWORD PTR [rbp-0x150]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 5bfd1 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 5bfd8 lea r13,[r13+rax*4+0x0]
            r13 = r13.wrapping_add(rax.wrapping_mul(4));
            // 5bfdd jmp 0x5bfe6
            pc = 0x5bfe6; continue;
        }
        0x5bfdf => {
            // block 5bfdf
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfdf mov r13,QWORD PTR [rbp-0x150]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            pc = 0x5bfe6;
        }
        0x5bfe6 => {
            // block 5bfe6
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfe6 mov ecx,DWORD PTR [rbp-0x154]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_feac)) as u64) as u32 as u64;
            // 5bfec mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 5bff3 inc DWORD PTR [rax]
            { let (r, f) = inc::<32>(m.r32(rax) as u64, fl); m.w32(rax, (r) as u32); fl = f; }
            // 5bff5 inc ecx
            { let (r, f) = inc::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5bff7 cmp ecx,DWORD PTR [rbp-0x158]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64).1;
            // 5bffd jne 0x5bf02
            if fl.ne() { pc = 0x5bf02; continue; }
            pc = 0x5c003;
        }
        0x5c003 => {
            // block 5c003
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5c003 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c003 mov rax,QWORD PTR [rip+0xe43ce] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // 5c00a mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 5c00d cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 5c011 jne 0x5c034
            if fl.ne() { pc = 0x5c034; continue; }
            pc = 0x5c013;
        }
        0x5c013 => {
            // block 5c013
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5c013 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c013 mov eax,DWORD PTR [rbp-0x158]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64) as u32 as u64;
            // 5c019 mov ecx,DWORD PTR [rbp+0x18]
            rcx = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 5c01c imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5c01f shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 5c022 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 5c029 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5c02a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5c02c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5c02e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5c030 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5c032 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c033 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5c034 => {
            // block 5c034
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5c034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c034 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
