//! Mechanically lifted functions (scripts/lift_run.py), module h006. Each one is checked against the original
//! in the world harness (tests/world_h006.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xb7348 size=560
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7348<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb7348;
    loop {
        match pc {
        0xb7348 => {
            // block b7348
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7348 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7348 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7349 mov rbp,rsp
            rbp = rsp;
            // b734c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b734e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7350 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b7352 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b7354 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7355 sub rsp,0x128
            { let (r, f) = sub::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // b735c mov r14,rdi
            r14 = rdi;
            // b735f mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // b7362 mov rdi,rsi
            rdi = rsi;
            // b7365 call QWORD PTR [rax+0x18a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x18a8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b736b mov bx,ax
            rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // b736e lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b7375 lea r13,[rbp-0x140]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // b737c mov rax,QWORD PTR [rip+0x88e4d] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b7383 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b7387 mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // b738e add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b7392 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // b7399 mov rsi,r13
            rsi = r13;
            // b739c call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b73a1 mov QWORD PTR [rbp-0x150],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r14) as u64);
            // b73a8 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // b73b0 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // b73b7 mov r14,QWORD PTR [rip+0x89002] # 0x1403c0
            r14 = m.r64(m.env.data_va(347021)) as u64;
            // b73be lea r12,[r14+0x18]
            r12 = r14.wrapping_add(0x18);
            // b73c2 mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // b73c9 add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // b73cd mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // b73d4 mov rdi,r13
            rdi = r13;
            // b73d7 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b73dc mov r15,QWORD PTR [rip+0x88fc5] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // b73e3 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // b73e7 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // b73ee mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // b73f9 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // b7404 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // b740f mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // b741a mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // b7424 xorps xmm0,xmm0
            x0 = 0;
            // b7427 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // b742b mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // b7433 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b7437 mov rdi,r13
            rdi = r13;
            // b743a call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b743f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b7443 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7448 mov rax,QWORD PTR [rbp-0x148]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // b744f mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b7453 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b745a mov ecx,DWORD PTR [rax+rdi*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rdi).wrapping_add(0x8)) as u64) as u32 as u64;
            // b745e and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b7461 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b7464 mov DWORD PTR [rax+rdi*1+0x8],ecx
            m.w32(rax.wrapping_add(rdi).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b7468 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // b746b mov rbx,r13
            rbx = r13;
            // b746e call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7473 mov r13,QWORD PTR [rbp-0x150]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // b747a mov rdi,r13
            rdi = r13;
            // b747d mov rsi,rbx
            rsi = rbx;
            // b7480 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7485 mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // b748c mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // b7493 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // b749a lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b74a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b74a6 mov rdi,rbx
            rdi = rbx;
            // b74a9 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b74ae mov rsi,QWORD PTR [rip+0x88d53] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b74b5 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b74b9 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b74c0 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b74c5 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b74cc call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b74d1 mov rax,r13
            rax = r13;
            // b74d4 add rsp,0x128
            { let (r, f) = add::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // b74db pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b74dc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b74de pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b74e0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b74e2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b74e4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b74e5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb74e6 => {
            // block b74e6
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b74e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b74e6 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // b74ed mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // b74f4 mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // b74fb mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // b7502 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b7509 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b750e mov rdi,rbx
            rdi = rbx;
            // b7511 jmp 0xb7544
            pc = 0xb7544; continue;
        }
        0xb7513 => {
            // block b7513
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7513 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7513 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // b751a jmp 0xb7560
            pc = 0xb7560; continue;
        }
        0xb751c => {
            // block b751c
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b751c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b751c mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // b7523 jmp 0xb7549
            pc = 0xb7549; continue;
        }
        0xb7525 => {
            // block b7525
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7525 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7525 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // b752c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b7530 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7535 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b753c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7541 mov rdi,r13
            rdi = r13;
            pc = 0xb7544;
        }
        0xb7544 => {
            // block b7544
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7544 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7544 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb7549;
        }
        0xb7549 => {
            // block b7549
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7549 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7549 mov rsi,QWORD PTR [rip+0x88cb8] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b7550 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b7554 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b755b call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb7560;
        }
        0xb7560 => {
            // block b7560
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7560 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b7567 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b756c mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // b7573 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe75e6 size=806
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e75e6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe75e6;
    loop {
        match pc {
        0xe75e6 => {
            // block e75e6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e75e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e75e6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e75e7 mov rbp,rsp
            rbp = rsp;
            // e75ea cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // e75f0 jg 0xe761b
            if fl.g() { pc = 0xe761b; continue; }
            pc = 0xe75f2;
        }
        0xe75f2 => {
            // block e75f2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e75f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e75f2 cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // e75f8 jg 0xe7647
            if fl.g() { pc = 0xe7647; continue; }
            pc = 0xe75fa;
        }
        0xe75fa => {
            // block e75fa
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e75fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e75fa cmp edi,0x102447ff
            fl = sub::<32>((rdi as u32 as u64), 0x102447ffu64).1;
            // e7600 jg 0xe768f
            if fl.g() { pc = 0xe768f; continue; }
            pc = 0xe7606;
        }
        0xe7606 => {
            // block e7606
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7606 cmp edi,0xffffffff
            fl = sub::<32>((rdi as u32 as u64), 0xffffffffu64).1;
            // e7609 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe760f;
        }
        0xe760f => {
            // block e760f
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e760f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e760f lea rax,[rip+0x5217f] # 0x139795
            rax = m.env.data_va(319330);
            // e7616 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe761b => {
            // block e761b
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e761b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e761b cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // e7621 jg 0xe766b
            if fl.g() { pc = 0xe766b; continue; }
            pc = 0xe7623;
        }
        0xe7623 => {
            // block e7623
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7623 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7623 cmp edi,0x106466ff
            fl = sub::<32>((rdi as u32 as u64), 0x106466ffu64).1;
            // e7629 jg 0xe76cb
            if fl.g() { pc = 0xe76cb; continue; }
            pc = 0xe762f;
        }
        0xe762f => {
            // block e762f
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e762f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e762f cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // e7635 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe763b;
        }
        0xe763b => {
            // block e763b
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e763b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e763b lea rax,[rip+0x5202d] # 0x13966f
            rax = m.env.data_va(319036);
            // e7642 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7647 => {
            // block e7647
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7647 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7647 cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // e764d jg 0xe76f6
            if fl.g() { pc = 0xe76f6; continue; }
            pc = 0xe7653;
        }
        0xe7653 => {
            // block e7653
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7653 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7653 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // e7659 jne 0xe7726
            if fl.ne() { pc = 0xe7726; continue; }
            pc = 0xe765f;
        }
        0xe765f => {
            // block e765f
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e765f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e765f lea rax,[rip+0x51fc0] # 0x139626
            rax = m.env.data_va(318963);
            // e7666 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe766b => {
            // block e766b
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e766b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e766b cmp edi,0x1071084f
            fl = sub::<32>((rdi as u32 as u64), 0x1071084fu64).1;
            // e7671 jg 0xe770e
            if fl.g() { pc = 0xe770e; continue; }
            pc = 0xe7677;
        }
        0xe7677 => {
            // block e7677
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7677 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7677 cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // e767d jne 0xe773e
            if fl.ne() { pc = 0xe773e; continue; }
            pc = 0xe7683;
        }
        0xe7683 => {
            // block e7683
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7683 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7683 lea rax,[rip+0x51ffa] # 0x139684
            rax = m.env.data_va(319057);
            // e768a jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe768f => {
            // block e768f
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e768f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e768f cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // e7695 jg 0xe7756
            if fl.g() { pc = 0xe7756; continue; }
            pc = 0xe769b;
        }
        0xe769b => {
            // block e769b
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e769b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e769b cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // e76a1 jg 0xe77b2
            if fl.g() { pc = 0xe77b2; continue; }
            pc = 0xe76a7;
        }
        0xe76a7 => {
            // block e76a7
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76a7 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // e76ad je 0xe7822
            if fl.e() { pc = 0xe7822; continue; }
            pc = 0xe76b3;
        }
        0xe76b3 => {
            // block e76b3
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76b3 cmp edi,0x10266400
            fl = sub::<32>((rdi as u32 as u64), 0x10266400u64).1;
            // e76b9 jne 0xe782e
            if fl.ne() { pc = 0xe782e; continue; }
            pc = 0xe76bf;
        }
        0xe76bf => {
            // block e76bf
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76bf lea rax,[rip+0x51e5f] # 0x139525
            rax = m.env.data_va(318706);
            // e76c6 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe76cb => {
            // block e76cb
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76cb add edi,0xef9b9900
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xef9b9900u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // e76d1 cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // e76d4 ja 0xe78e4
            if fl.a() { pc = 0xe78e4; continue; }
            pc = 0xe76da;
        }
        0xe76da => {
            // block e76da
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76da lea rax,[rip+0x20f] # 0xe78f0
            rax = m.env.code_va(0xe78f0);
            // e76e1 movsxd rcx,DWORD PTR [rax+rdi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // e76e5 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // e76e8 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe76ea => {
            // block e76ea
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76ea lea rax,[rip+0x51fa7] # 0x139698
            rax = m.env.data_va(319077);
            // e76f1 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe76f6 => {
            // block e76f6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76f6 cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // e76fc jne 0xe7782
            if fl.ne() { pc = 0xe7782; continue; }
            pc = 0xe7702;
        }
        0xe7702 => {
            // block e7702
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7702 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7702 lea rax,[rip+0x51f40] # 0x139649
            rax = m.env.data_va(318998);
            // e7709 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe770e => {
            // block e770e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e770e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e770e cmp edi,0x10710850
            fl = sub::<32>((rdi as u32 as u64), 0x10710850u64).1;
            // e7714 jne 0xe779a
            if fl.ne() { pc = 0xe779a; continue; }
            pc = 0xe771a;
        }
        0xe771a => {
            // block e771a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e771a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e771a lea rax,[rip+0x5204c] # 0x13976d
            rax = m.env.data_va(319290);
            // e7721 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7726 => {
            // block e7726
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7726 cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // e772c jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe7732;
        }
        0xe7732 => {
            // block e7732
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7732 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7732 lea rax,[rip+0x51efd] # 0x139636
            rax = m.env.data_va(318979);
            // e7739 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe773e => {
            // block e773e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e773e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e773e cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // e7744 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe774a;
        }
        0xe774a => {
            // block e774a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e774a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e774a lea rax,[rip+0x52009] # 0x13975a
            rax = m.env.data_va(319271);
            // e7751 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7756 => {
            // block e7756
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7756 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7756 cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // e775c jg 0xe77d2
            if fl.g() { pc = 0xe77d2; continue; }
            pc = 0xe775e;
        }
        0xe775e => {
            // block e775e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e775e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e775e cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // e7764 je 0xe7846
            if fl.e() { pc = 0xe7846; continue; }
            pc = 0xe776a;
        }
        0xe776a => {
            // block e776a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e776a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e776a cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // e7770 jne 0xe7852
            if fl.ne() { pc = 0xe7852; continue; }
            pc = 0xe7776;
        }
        0xe7776 => {
            // block e7776
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7776 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7776 lea rax,[rip+0x51e41] # 0x1395be
            rax = m.env.data_va(318859);
            // e777d jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7782 => {
            // block e7782
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7782 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7782 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // e7788 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe778e;
        }
        0xe778e => {
            // block e778e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e778e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e778e lea rax,[rip+0x51ec7] # 0x13965c
            rax = m.env.data_va(319017);
            // e7795 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe779a => {
            // block e779a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e779a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e779a cmp edi,0x10710851
            fl = sub::<32>((rdi as u32 as u64), 0x10710851u64).1;
            // e77a0 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe77a6;
        }
        0xe77a6 => {
            // block e77a6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77a6 lea rax,[rip+0x51fd4] # 0x139781
            rax = m.env.data_va(319310);
            // e77ad jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe77b2 => {
            // block e77b2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77b2 cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // e77b8 jg 0xe77f2
            if fl.g() { pc = 0xe77f2; continue; }
            pc = 0xe77ba;
        }
        0xe77ba => {
            // block e77ba
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77ba cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // e77c0 jne 0xe78a0
            if fl.ne() { pc = 0xe78a0; continue; }
            pc = 0xe77c6;
        }
        0xe77c6 => {
            // block e77c6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77c6 lea rax,[rip+0x51d7f] # 0x13954c
            rax = m.env.data_va(318745);
            // e77cd jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe77d2 => {
            // block e77d2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77d2 cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // e77d8 jg 0xe780a
            if fl.g() { pc = 0xe780a; continue; }
            pc = 0xe77da;
        }
        0xe77da => {
            // block e77da
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77da cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // e77e0 jne 0xe78b1
            if fl.ne() { pc = 0xe78b1; continue; }
            pc = 0xe77e6;
        }
        0xe77e6 => {
            // block e77e6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77e6 lea rax,[rip+0x51df6] # 0x1395e3
            rax = m.env.data_va(318896);
            // e77ed jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe77f2 => {
            // block e77f2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77f2 cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // e77f8 jne 0xe78c2
            if fl.ne() { pc = 0xe78c2; continue; }
            pc = 0xe77fe;
        }
        0xe77fe => {
            // block e77fe
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77fe lea rax,[rip+0x51d93] # 0x139598
            rax = m.env.data_va(318821);
            // e7805 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe780a => {
            // block e780a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e780a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e780a cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // e7810 jne 0xe78d3
            if fl.ne() { pc = 0xe78d3; continue; }
            pc = 0xe7816;
        }
        0xe7816 => {
            // block e7816
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7816 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7816 lea rax,[rip+0x51de8] # 0x139605
            rax = m.env.data_va(318930);
            // e781d jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7822 => {
            // block e7822
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7822 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7822 lea rax,[rip+0x51d37] # 0x139560
            rax = m.env.data_va(318765);
            // e7829 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe782e => {
            // block e782e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e782e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e782e cmp edi,0x10266401
            fl = sub::<32>((rdi as u32 as u64), 0x10266401u64).1;
            // e7834 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe783a;
        }
        0xe783a => {
            // block e783a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e783a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e783a lea rax,[rip+0x51cf6] # 0x139537
            rax = m.env.data_va(318724);
            // e7841 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7846 => {
            // block e7846
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7846 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7846 lea rax,[rip+0x51d5c] # 0x1395a9
            rax = m.env.data_va(318838);
            // e784d jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7852 => {
            // block e7852
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7852 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7852 cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // e7858 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe785e;
        }
        0xe785e => {
            // block e785e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e785e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e785e lea rax,[rip+0x51d6f] # 0x1395d4
            rax = m.env.data_va(318881);
            // e7865 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe786a => {
            // block e786a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e786a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e786a lea rax,[rip+0x51e3d] # 0x1396ae
            rax = m.env.data_va(319099);
            // e7871 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7873 => {
            // block e7873
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7873 lea rax,[rip+0x51e4e] # 0x1396c8
            rax = m.env.data_va(319125);
            // e787a jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe787c => {
            // block e787c
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e787c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e787c lea rax,[rip+0x51e67] # 0x1396ea
            rax = m.env.data_va(319159);
            // e7883 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7885 => {
            // block e7885
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7885 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7885 lea rax,[rip+0x51e7c] # 0x139708
            rax = m.env.data_va(319189);
            // e788c jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe788e => {
            // block e788e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e788e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e788e lea rax,[rip+0x51e91] # 0x139726
            rax = m.env.data_va(319219);
            // e7895 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7897 => {
            // block e7897
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7897 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7897 lea rax,[rip+0x51ea6] # 0x139744
            rax = m.env.data_va(319249);
            // e789e jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78a0 => {
            // block e78a0
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78a0 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // e78a6 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe78a8;
        }
        0xe78a8 => {
            // block e78a8
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78a8 lea rax,[rip+0x51cc3] # 0x139572
            rax = m.env.data_va(318783);
            // e78af jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78b1 => {
            // block e78b1
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78b1 cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // e78b7 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe78b9;
        }
        0xe78b9 => {
            // block e78b9
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78b9 lea rax,[rip+0x51d36] # 0x1395f6
            rax = m.env.data_va(318915);
            // e78c0 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78c2 => {
            // block e78c2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78c2 cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // e78c8 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe78ca;
        }
        0xe78ca => {
            // block e78ca
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78ca lea rax,[rip+0x51cb4] # 0x139585
            rax = m.env.data_va(318802);
            // e78d1 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78d3 => {
            // block e78d3
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78d3 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // e78d9 jne 0xe78e4
            if fl.ne() { pc = 0xe78e4; continue; }
            pc = 0xe78db;
        }
        0xe78db => {
            // block e78db
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78db lea rax,[rip+0x51d32] # 0x139614
            rax = m.env.data_va(318945);
            // e78e2 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78e4 => {
            // block e78e4
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78e4 lea rax,[rip+0x513dd] # 0x138cc8
            rax = m.env.data_va(316565);
            pc = 0xe78eb;
        }
        0xe78eb => {
            // block e78eb
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78eb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e78ec ret
            return Ret { rax, rdx, xmm0: x0 };
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

/// parity: P2v @0xb68f8 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b68f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb68f8;
    loop {
        match pc {
        0xb68f8 => {
            // block b68f8
            #[cfg(feature = "trace")] eprintln!("f_000b68f8 pc b68f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b68f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b68f9 mov rbp,rsp
            rbp = rsp;
            // b68fc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b68fd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b68fe mov rbx,rdi
            rbx = rdi;
            // b6901 call 0xb6894
            { let r = { let t = m.env.code_va(0xb6894); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6906 mov rdi,rbx
            rdi = rbx;
            // b6909 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b690d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b690e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b690f jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6d58 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6d58<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd6d58;
    loop {
        match pc {
        0xd6d58 => {
            // block d6d58
            #[cfg(feature = "trace")] eprintln!("f_000d6d58 pc d6d58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6d58 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6d59 mov rbp,rsp
            rbp = rsp;
            // d6d5c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6d5d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d6d5e mov rbx,rdi
            rbx = rdi;
            // d6d61 call 0xd6c84
            { let r = { let t = m.env.code_va(0xd6c84); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d66 mov rdi,rbx
            rdi = rbx;
            // d6d69 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d6d6d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6d6e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6d6f jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe080c size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e080c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe080c;
    loop {
        match pc {
        0xe080c => {
            // block e080c
            #[cfg(feature = "trace")] eprintln!("f_000e080c pc e080c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e080c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e080d mov rbp,rsp
            rbp = rsp;
            // e0810 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0811 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0812 mov rbx,rdi
            rbx = rdi;
            // e0815 call 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e081a mov rdi,rbx
            rdi = rbx;
            // e081d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e0821 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0822 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0823 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe8086 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e8086<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe8086;
    loop {
        match pc {
        0xe8086 => {
            // block e8086
            #[cfg(feature = "trace")] eprintln!("f_000e8086 pc e8086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8086 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e8087 mov rbp,rsp
            rbp = rsp;
            // e808a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e808b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e808c mov rbx,rdi
            rbx = rdi;
            // e808f call 0xe7ef4
            { let r = { let t = m.env.code_va(0xe7ef4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8094 mov rdi,rbx
            rdi = rbx;
            // e8097 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e809b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e809c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e809d jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea5a2 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea5a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xea5a2;
    loop {
        match pc {
        0xea5a2 => {
            // block ea5a2
            #[cfg(feature = "trace")] eprintln!("f_000ea5a2 pc ea5a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea5a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea5a3 mov rbp,rsp
            rbp = rsp;
            // ea5a6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea5a7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ea5a8 mov rbx,rdi
            rbx = rdi;
            // ea5ab call 0xea4f0
            { let r = { let t = m.env.code_va(0xea4f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea5b0 mov rdi,rbx
            rdi = rbx;
            // ea5b3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ea5b7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea5b8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea5b9 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe7cde size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7cde<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe7cde;
    loop {
        match pc {
        0xe7cde => {
            // block e7cde
            #[cfg(feature = "trace")] eprintln!("f_000e7cde pc e7cde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7cde push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7cdf mov rbp,rsp
            rbp = rsp;
            // e7ce2 cmp edi,0x1ff
            fl = sub::<32>((rdi as u32 as u64), 0x1ffu64).1;
            // e7ce8 ja 0xe7cf9
            if fl.a() { pc = 0xe7cf9; continue; }
            pc = 0xe7cea;
        }
        0xe7cea => {
            // block e7cea
            #[cfg(feature = "trace")] eprintln!("f_000e7cde pc e7cea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7cea mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // e7cec lea rcx,[rip+0xd8cfd] # 0x1c09f0
            rcx = m.env.data_va(872893);
            // e7cf3 mov rax,QWORD PTR [rcx+rax*8]
            rax = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // e7cf7 jmp 0xe7d00
            pc = 0xe7d00; continue;
        }
        0xe7cf9 => {
            // block e7cf9
            #[cfg(feature = "trace")] eprintln!("f_000e7cde pc e7cf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7cf9 lea rax,[rip+0x50fc8] # 0x138cc8
            rax = m.env.data_va(316565);
            pc = 0xe7d00;
        }
        0xe7d00 => {
            // block e7d00
            #[cfg(feature = "trace")] eprintln!("f_000e7cde pc e7d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7d00 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7d01 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb796e size=46
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b796e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb796e;
    loop {
        match pc {
        0xb796e => {
            // block b796e
            #[cfg(feature = "trace")] eprintln!("f_000b796e pc b796e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b796e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b796f mov rbp,rsp
            rbp = rsp;
            // b7972 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7974 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7975 mov rbx,rdi
            rbx = rdi;
            // b7978 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b797b call QWORD PTR [rax+0x1d18]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7981 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // b7984 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7987 mov rdi,rbx
            rdi = rbx;
            // b798a call QWORD PTR [rax+0x1d20]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d20)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7990 shl rax,0x20
            { let (r, f) = shl::<64>(rax, 0x20u64, fl); rax = r; fl = f; }
            // b7994 or rax,r14
            { let r = rax | r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // b7997 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7998 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b799a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b799b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe09b6 size=50
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e09b6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe09b6;
    loop {
        match pc {
        0xe09b6 => {
            // block e09b6
            #[cfg(feature = "trace")] eprintln!("f_000e09b6 pc e09b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e09b6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e09b7 mov rbp,rsp
            rbp = rsp;
            // e09ba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e09bb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e09bc mov rbx,rdi
            rbx = rdi;
            // e09bf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e09c2 call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e09c5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e09c7 jne 0xe09e1
            if fl.ne() { pc = 0xe09e1; continue; }
            pc = 0xe09c9;
        }
        0xe09c9 => {
            // block e09c9
            #[cfg(feature = "trace")] eprintln!("f_000e09b6 pc e09c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e09c9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e09cc mov rdi,rbx
            rdi = rbx;
            // e09cf call QWORD PTR [rax+0x70]
            { let r = { let t = m.r64(rax.wrapping_add(0x70)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e09d2 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e09d4 jne 0xe09e1
            if fl.ne() { pc = 0xe09e1; continue; }
            pc = 0xe09d6;
        }
        0xe09d6 => {
            // block e09d6
            #[cfg(feature = "trace")] eprintln!("f_000e09b6 pc e09d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e09d6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e09d9 mov rdi,rbx
            rdi = rbx;
            // e09dc call QWORD PTR [rax+0x78]
            { let r = { let t = m.r64(rax.wrapping_add(0x78)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e09df xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe09e1;
        }
        0xe09e1 => {
            // block e09e1
            #[cfg(feature = "trace")] eprintln!("f_000e09b6 pc e09e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e09e1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e09e5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e09e6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e09e7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbacd size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbacd<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbbacd;
    loop {
        match pc {
        0xbbacd => {
            // block bbacd
            #[cfg(feature = "trace")] eprintln!("f_000bbacd pc bbacd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbacd push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbace mov rbp,rsp
            rbp = rsp;
            // bbad1 mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // bbad6 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbad9 ja 0xbbb05
            if fl.a() { pc = 0xbbb05; continue; }
            pc = 0xbbadb;
        }
        0xbbadb => {
            // block bbadb
            #[cfg(feature = "trace")] eprintln!("f_000bbacd pc bbadb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbadb mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbadd test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbae0 je 0xbbaee
            if fl.e() { pc = 0xbbaee; continue; }
            pc = 0xbbae2;
        }
        0xbbae2 => {
            // block bbae2
            #[cfg(feature = "trace")] eprintln!("f_000bbacd pc bbae2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbae2 lea rcx,[rip+0x47017] # 0x102b00
            rcx = m.env.data_va(94925);
            // bbae9 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // bbaec jmp 0xbbb05
            pc = 0xbbb05; continue;
        }
        0xbbaee => {
            // block bbaee
            #[cfg(feature = "trace")] eprintln!("f_000bbacd pc bbaee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbaee lea rcx,[rip+0x4702b] # 0x102b20
            rcx = m.env.data_va(94957);
            // bbaf5 lea rsi,[rip+0x47044] # 0x102b40
            rsi = m.env.data_va(94989);
            // bbafc test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // bbafe cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // bbb02 mov eax,DWORD PTR [rsi+rax*4]
            rax = (m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbb05;
        }
        0xbbb05 => {
            // block bbb05
            #[cfg(feature = "trace")] eprintln!("f_000bbacd pc bbb05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb05 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbb06 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbb07 size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbb07<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbbb07;
    loop {
        match pc {
        0xbbb07 => {
            // block bbb07
            #[cfg(feature = "trace")] eprintln!("f_000bbb07 pc bbb07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb07 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbb08 mov rbp,rsp
            rbp = rsp;
            // bbb0b mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // bbb10 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbb13 ja 0xbbb43
            if fl.a() { pc = 0xbbb43; continue; }
            pc = 0xbbb15;
        }
        0xbbb15 => {
            // block bbb15
            #[cfg(feature = "trace")] eprintln!("f_000bbb07 pc bbb15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb15 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbb17 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // bbb19 je 0xbbb2b
            if fl.e() { pc = 0xbbb2b; continue; }
            pc = 0xbbb1b;
        }
        0xbbb1b => {
            // block bbb1b
            #[cfg(feature = "trace")] eprintln!("f_000bbb07 pc bbb1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb1b lea rcx,[rip+0x4709e] # 0x102bc0
            rcx = m.env.data_va(95117);
            // bbb22 lea rdx,[rip+0x47077] # 0x102ba0
            rdx = m.env.data_va(95085);
            // bbb29 jmp 0xbbb39
            pc = 0xbbb39; continue;
        }
        0xbbb2b => {
            // block bbb2b
            #[cfg(feature = "trace")] eprintln!("f_000bbb07 pc bbb2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb2b lea rcx,[rip+0x4704e] # 0x102b80
            rcx = m.env.data_va(95053);
            // bbb32 lea rdx,[rip+0x47027] # 0x102b60
            rdx = m.env.data_va(95021);
            pc = 0xbbb39;
        }
        0xbbb39 => {
            // block bbb39
            #[cfg(feature = "trace")] eprintln!("f_000bbb07 pc bbb39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb39 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbb3c cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbb40 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbb43;
        }
        0xbbb43 => {
            // block bbb43
            #[cfg(feature = "trace")] eprintln!("f_000bbb07 pc bbb43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb43 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbb44 ret
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

/// parity: P2v @0xe7d04 size=486
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7d04<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe7d04;
    loop {
        match pc {
        0xe7d04 => {
            // block e7d04
            #[cfg(feature = "trace")] eprintln!("f_000e7d04 pc e7d04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7d04 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7d05 mov rbp,rsp
            rbp = rsp;
            // e7d08 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e7d0a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e7d0c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e7d0e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e7d10 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e7d11 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e7d15 mov rbx,rdi
            rbx = rdi;
            // e7d18 lea rax,[rip+0x5ec11] # 0x146930
            rax = m.env.data_va(372989);
            // e7d1f mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e7d22 lea rdi,[rbx+0x1b0]
            rdi = rbx.wrapping_add(0x1b0);
            // e7d29 mov QWORD PTR [rbp-0x30],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdi) as u64);
            // e7d2d lea r15,[rbx+0x18]
            r15 = rbx.wrapping_add(0x18);
            // e7d31 mov rax,QWORD PTR [rip+0x58480] # 0x1401b8
            rax = m.r64(m.env.data_va(346501)) as u64;
            // e7d38 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // e7d3c mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // e7d40 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // e7d44 mov QWORD PTR [rbx+0x1b0],rax
            m.w64(rbx.wrapping_add(0x1b0), (rax) as u64);
            // e7d4b mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // e7d53 mov rsi,r15
            rsi = r15;
            // e7d56 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7d5b lea rax,[rbx+0x8]
            rax = rbx.wrapping_add(0x8);
            // e7d5f mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // e7d63 mov QWORD PTR [rbx+0x238],0x0
            m.w64(rbx.wrapping_add(0x238), (0x0u64) as u64);
            // e7d6e mov DWORD PTR [rbx+0x240],0xffffffff
            m.w32(rbx.wrapping_add(0x240), (0xffffffffu64) as u32);
            // e7d78 mov r14,QWORD PTR [rip+0x58621] # 0x1403a0
            r14 = m.r64(m.env.data_va(346989)) as u64;
            // e7d7f lea r12,[r14+0x18]
            r12 = r14.wrapping_add(0x18);
            // e7d83 mov QWORD PTR [rbx+0x8],r12
            m.w64(rbx.wrapping_add(0x8), (r12) as u64);
            // e7d87 add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // e7d8b mov QWORD PTR [rbx+0x1b0],r14
            m.w64(rbx.wrapping_add(0x1b0), (r14) as u64);
            // e7d92 mov rdi,r15
            rdi = r15;
            // e7d95 call 0xeb90c
            { let r = { let t = m.env.stub_va(0xd9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7d9a lea rax,[rbx+0x248]
            rax = rbx.wrapping_add(0x248);
            // e7da1 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // e7da5 mov QWORD PTR [rbx+0x258],0x0
            m.w64(rbx.wrapping_add(0x258), (0x0u64) as u64);
            // e7db0 mov QWORD PTR [rbx+0x250],0x0
            m.w64(rbx.wrapping_add(0x250), (0x0u64) as u64);
            // e7dbb mov QWORD PTR [rbx+0x248],0x0
            m.w64(rbx.wrapping_add(0x248), (0x0u64) as u64);
            // e7dc6 lea rax,[rbx+0x268]
            rax = rbx.wrapping_add(0x268);
            // e7dcd mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // e7dd1 mov QWORD PTR [rbx+0x278],0x0
            m.w64(rbx.wrapping_add(0x278), (0x0u64) as u64);
            // e7ddc mov QWORD PTR [rbx+0x270],0x0
            m.w64(rbx.wrapping_add(0x270), (0x0u64) as u64);
            // e7de7 mov QWORD PTR [rbx+0x268],0x0
            m.w64(rbx.wrapping_add(0x268), (0x0u64) as u64);
            // e7df2 lea r13,[rbx+0x290]
            r13 = rbx.wrapping_add(0x290);
            // e7df9 mov esi,0x90
            rsi = (0x90u64) as u32 as u64;
            // e7dfe mov rdi,r13
            rdi = r13;
            // e7e01 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7e06 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e7e09 mov rax,QWORD PTR [rax+0x90]
            rax = m.r64(rax.wrapping_add(0x90)) as u64;
            // e7e10 mov rdi,rbx
            rdi = rbx;
            // e7e13 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7e15 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e7e19 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e7e1a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e7e1c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e7e1e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e7e20 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e7e22 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7e23 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe7e24 => {
            // block e7e24
            #[cfg(feature = "trace")] eprintln!("f_000e7d04 pc e7e24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7e24 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // e7e28 jmp 0xe7ed7
            pc = 0xe7ed7; continue;
        }
        0xe7e2d => {
            // block e7e2d
            #[cfg(feature = "trace")] eprintln!("f_000e7d04 pc e7e2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7e2d mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // e7e31 jmp 0xe7ec3
            pc = 0xe7ec3; continue;
        }
        0xe7e36 => {
            // block e7e36
            #[cfg(feature = "trace")] eprintln!("f_000e7d04 pc e7e36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7e36 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // e7e3a lea rdi,[rbx+0x308]
            rdi = rbx.wrapping_add(0x308);
            // e7e41 lea rax,[rbx+0x2f0]
            rax = rbx.wrapping_add(0x2f0);
            // e7e48 mov QWORD PTR [rbp-0x70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rax) as u64);
            // e7e4c lea rax,[rbx+0x2d8]
            rax = rbx.wrapping_add(0x2d8);
            // e7e53 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // e7e57 lea rax,[rbx+0x2c0]
            rax = rbx.wrapping_add(0x2c0);
            // e7e5e mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // e7e62 lea rax,[rbx+0x2a8]
            rax = rbx.wrapping_add(0x2a8);
            // e7e69 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e7e6d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7e72 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // e7e76 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7e7b mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e7e7f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7e84 mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e7e88 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7e8d mov rdi,QWORD PTR [rbp-0x58]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e7e91 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7e96 mov rdi,r13
            rdi = r13;
            // e7e99 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7e9e mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e7ea2 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7ea7 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // e7eab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7eb0 mov QWORD PTR [rbx+0x8],r12
            m.w64(rbx.wrapping_add(0x8), (r12) as u64);
            // e7eb4 mov QWORD PTR [rbx+0x1b0],r14
            m.w64(rbx.wrapping_add(0x1b0), (r14) as u64);
            // e7ebb mov rdi,r15
            rdi = r15;
            // e7ebe call 0xeb912
            { let r = { let t = m.env.stub_va(0xdf); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe7ec3;
        }
        0xe7ec3 => {
            // block e7ec3
            #[cfg(feature = "trace")] eprintln!("f_000e7d04 pc e7ec3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ec3 mov rsi,QWORD PTR [rip+0x58326] # 0x1401f0
            rsi = m.r64(m.env.data_va(346557)) as u64;
            // e7eca add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e7ece mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e7ed2 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe7ed7;
        }
        0xe7ed7 => {
            // block e7ed7
            #[cfg(feature = "trace")] eprintln!("f_000e7d04 pc e7ed7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ed7 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e7edb call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7ee0 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e7ee4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7110 size=600
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7110<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd7110;
    loop {
        match pc {
        0xd7110 => {
            // block d7110
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7110 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7110 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7111 mov rbp,rsp
            rbp = rsp;
            // d7114 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d7116 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7118 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d711a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d711c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d711d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d711e mov r12,rsi
            r12 = rsi;
            // d7121 mov r14,rdi
            r14 = rdi;
            // d7124 lea rsi,[rip+0xe6655] # 0x1bd780
            rsi = m.env.data_va(859981);
            // d712b mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // d7130 mov rdi,r12
            rdi = r12;
            // d7133 call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7138 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d713a jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd7140;
        }
        0xd7140 => {
            // block d7140
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7140 movzx eax,BYTE PTR [r12+0xd]
            rax = (m.r8(r12.wrapping_add(0xd)) as u64) as u32 as u64;
            // d7146 cmp eax,0x61
            fl = sub::<32>((rax as u32 as u64), 0x61u64).1;
            // d7149 jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd714f;
        }
        0xd714f => {
            // block d714f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d714f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d714f mov bx,WORD PTR [r12+0xe]
            rbx = (rbx & !0xffff) | ((m.r16(r12.wrapping_add(0xe)) as u64) as u16 as u64);
            // d7155 rol bx,0x8
            { let (r, f) = rol::<16>((rbx as u16 as u64), 0x8u64, fl); rbx = (rbx & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d7159 lea rsi,[r12+0x10]
            rsi = r12.wrapping_add(0x10);
            // d715e lea rdi,[r14+0x2e8]
            rdi = r14.wrapping_add(0x2e8);
            // d7165 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d716a movzx ebx,bx
            rbx = ((rbx as u16 as u64)) as u32 as u64;
            // d716d movzx eax,BYTE PTR [rbx+r12*1+0x10]
            rax = (m.r8(rbx.wrapping_add(r12).wrapping_add(0x10)) as u64) as u32 as u64;
            // d7173 cmp eax,0x62
            fl = sub::<32>((rax as u32 as u64), 0x62u64).1;
            // d7176 jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd717c;
        }
        0xd717c => {
            // block d717c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d717c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d717c mov r13w,WORD PTR [rbx+r12*1+0x11]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x11)) as u64) as u16 as u64);
            // d7182 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d7187 lea rsi,[r12+rbx*1+0x13]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x13);
            // d718c lea r15,[r14+0x300]
            r15 = r14.wrapping_add(0x300);
            // d7193 mov rdi,r15
            rdi = r15;
            // d7196 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d719b movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d719f lea rbx,[rax+rbx*1+0x13]
            rbx = rax.wrapping_add(rbx).wrapping_add(0x13);
            // d71a4 movzx eax,BYTE PTR [r12+rbx*1]
            rax = (m.r8(r12.wrapping_add(rbx)) as u64) as u32 as u64;
            // d71a9 cmp eax,0x63
            fl = sub::<32>((rax as u32 as u64), 0x63u64).1;
            // d71ac jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd71ae;
        }
        0xd71ae => {
            // block d71ae
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d71ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d71ae mov r13w,WORD PTR [rbx+r12*1+0x1]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x1)) as u64) as u16 as u64);
            // d71b4 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d71b9 lea rsi,[r12+rbx*1+0x3]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x3);
            // d71be lea rdi,[r14+0x2b8]
            rdi = r14.wrapping_add(0x2b8);
            // d71c5 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d71ca movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d71ce lea rbx,[rax+rbx*1+0x3]
            rbx = rax.wrapping_add(rbx).wrapping_add(0x3);
            // d71d3 movzx eax,BYTE PTR [r12+rbx*1]
            rax = (m.r8(r12.wrapping_add(rbx)) as u64) as u32 as u64;
            // d71d8 cmp eax,0x64
            fl = sub::<32>((rax as u32 as u64), 0x64u64).1;
            // d71db jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd71dd;
        }
        0xd71dd => {
            // block d71dd
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d71dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d71dd mov r13w,WORD PTR [rbx+r12*1+0x1]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x1)) as u64) as u16 as u64);
            // d71e3 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d71e8 lea rsi,[r12+rbx*1+0x3]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x3);
            // d71ed lea rdi,[r14+0x2d0]
            rdi = r14.wrapping_add(0x2d0);
            // d71f4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d71f9 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d71fd lea rax,[rax+rbx*1+0x3]
            rax = rax.wrapping_add(rbx).wrapping_add(0x3);
            // d7202 movzx ecx,BYTE PTR [r12+rax*1]
            rcx = (m.r8(r12.wrapping_add(rax)) as u64) as u32 as u64;
            // d7207 cmp ecx,0x65
            fl = sub::<32>((rcx as u32 as u64), 0x65u64).1;
            // d720a jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd720c;
        }
        0xd720c => {
            // block d720c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d720c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d720c mov ecx,DWORD PTR [rax+r12*1+0x1]
            rcx = (m.r32(rax.wrapping_add(r12).wrapping_add(0x1)) as u64) as u32 as u64;
            // d7211 bswap ecx
            rcx = (rcx as u32).swap_bytes() as u64;
            // d7213 mov DWORD PTR [r14+0x36c],ecx
            m.w32(r14.wrapping_add(0x36c), ((rcx as u32 as u64)) as u32);
            // d721a mov cl,BYTE PTR [r14+0x300]
            rcx = (rcx & !0xff) | ((m.r8(r14.wrapping_add(0x300)) as u64) as u8 as u64);
            // d7221 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d7224 jne 0xd723d
            if fl.ne() { pc = 0xd723d; continue; }
            pc = 0xd7226;
        }
        0xd7226 => {
            // block d7226
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7226 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7226 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d722a jmp 0xd7244
            pc = 0xd7244; continue;
        }
        0xd722c => {
            // block d722c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d722c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d722c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd722e;
        }
        0xd722e => {
            // block d722e
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d722e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d722e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d7232 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7233 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7235 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d7237 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7239 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d723b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d723c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd723d => {
            // block d723d
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d723d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d723d mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7244;
        }
        0xd7244 => {
            // block d7244
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7244 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7247 cmp edx,0x35
            fl = sub::<32>((rdx as u32 as u64), 0x35u64).1;
            // d724a je 0xd729b
            if fl.e() { pc = 0xd729b; continue; }
            pc = 0xd724c;
        }
        0xd724c => {
            // block d724c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d724c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d724c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d724e jne 0xd7256
            if fl.ne() { pc = 0xd7256; continue; }
            pc = 0xd7250;
        }
        0xd7250 => {
            // block d7250
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7250 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7254 jmp 0xd725d
            pc = 0xd725d; continue;
        }
        0xd7256 => {
            // block d7256
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7256 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7256 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd725d;
        }
        0xd725d => {
            // block d725d
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d725d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d725d movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7260 cmp edx,0x36
            fl = sub::<32>((rdx as u32 as u64), 0x36u64).1;
            // d7263 je 0xd729b
            if fl.e() { pc = 0xd729b; continue; }
            pc = 0xd7265;
        }
        0xd7265 => {
            // block d7265
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7265 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7267 jne 0xd726f
            if fl.ne() { pc = 0xd726f; continue; }
            pc = 0xd7269;
        }
        0xd7269 => {
            // block d7269
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7269 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d726d jmp 0xd7276
            pc = 0xd7276; continue;
        }
        0xd726f => {
            // block d726f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d726f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d726f mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7276;
        }
        0xd7276 => {
            // block d7276
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7276 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7279 cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // d727c je 0xd729b
            if fl.e() { pc = 0xd729b; continue; }
            pc = 0xd727e;
        }
        0xd727e => {
            // block d727e
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d727e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d727e test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7280 jne 0xd7288
            if fl.ne() { pc = 0xd7288; continue; }
            pc = 0xd7282;
        }
        0xd7282 => {
            // block d7282
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7282 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7286 jmp 0xd728f
            pc = 0xd728f; continue;
        }
        0xd7288 => {
            // block d7288
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7288 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7288 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd728f;
        }
        0xd728f => {
            // block d728f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d728f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d728f movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7292 cmp edx,0x78
            fl = sub::<32>((rdx as u32 as u64), 0x78u64).1;
            // d7295 jne 0xd7361
            if fl.ne() { pc = 0xd7361; continue; }
            pc = 0xd729b;
        }
        0xd729b => {
            // block d729b
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d729b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d729b test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d729d jne 0xd72a5
            if fl.ne() { pc = 0xd72a5; continue; }
            pc = 0xd729f;
        }
        0xd729f => {
            // block d729f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d729f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d729f lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72a3 jmp 0xd72ac
            pc = 0xd72ac; continue;
        }
        0xd72a5 => {
            // block d72a5
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72a5 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72ac;
        }
        0xd72ac => {
            // block d72ac
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72ac movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72af cmp edx,0x35
            fl = sub::<32>((rdx as u32 as u64), 0x35u64).1;
            // d72b2 je 0xd731a
            if fl.e() { pc = 0xd731a; continue; }
            pc = 0xd72b4;
        }
        0xd72b4 => {
            // block d72b4
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72b4 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72b6 jne 0xd72be
            if fl.ne() { pc = 0xd72be; continue; }
            pc = 0xd72b8;
        }
        0xd72b8 => {
            // block d72b8
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72b8 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72bc jmp 0xd72c5
            pc = 0xd72c5; continue;
        }
        0xd72be => {
            // block d72be
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72be mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72c5;
        }
        0xd72c5 => {
            // block d72c5
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72c5 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72c8 cmp edx,0x36
            fl = sub::<32>((rdx as u32 as u64), 0x36u64).1;
            // d72cb jne 0xd72e7
            if fl.ne() { pc = 0xd72e7; continue; }
            pc = 0xd72cd;
        }
        0xd72cd => {
            // block d72cd
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72cd test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72cf jne 0xd72d7
            if fl.ne() { pc = 0xd72d7; continue; }
            pc = 0xd72d1;
        }
        0xd72d1 => {
            // block d72d1
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72d1 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72d5 jmp 0xd72de
            pc = 0xd72de; continue;
        }
        0xd72d7 => {
            // block d72d7
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72d7 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72de;
        }
        0xd72de => {
            // block d72de
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72de movzx edx,BYTE PTR [rdx+0x1]
            rdx = (m.r8(rdx.wrapping_add(0x1)) as u64) as u32 as u64;
            // d72e2 cmp edx,0x76
            fl = sub::<32>((rdx as u32 as u64), 0x76u64).1;
            // d72e5 je 0xd731a
            if fl.e() { pc = 0xd731a; continue; }
            pc = 0xd72e7;
        }
        0xd72e7 => {
            // block d72e7
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72e7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72e9 jne 0xd72f1
            if fl.ne() { pc = 0xd72f1; continue; }
            pc = 0xd72eb;
        }
        0xd72eb => {
            // block d72eb
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72eb lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72ef jmp 0xd72f8
            pc = 0xd72f8; continue;
        }
        0xd72f1 => {
            // block d72f1
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72f1 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72f8;
        }
        0xd72f8 => {
            // block d72f8
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72f8 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72fb cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // d72fe jne 0xd733a
            if fl.ne() { pc = 0xd733a; continue; }
            pc = 0xd7300;
        }
        0xd7300 => {
            // block d7300
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7300 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7300 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7302 jne 0xd730a
            if fl.ne() { pc = 0xd730a; continue; }
            pc = 0xd7304;
        }
        0xd7304 => {
            // block d7304
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7304 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7304 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7308 jmp 0xd7311
            pc = 0xd7311; continue;
        }
        0xd730a => {
            // block d730a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d730a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d730a mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7311;
        }
        0xd7311 => {
            // block d7311
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7311 movzx edx,BYTE PTR [rdx+0x1]
            rdx = (m.r8(rdx.wrapping_add(0x1)) as u64) as u32 as u64;
            // d7315 cmp edx,0x6b
            fl = sub::<32>((rdx as u32 as u64), 0x6bu64).1;
            // d7318 jne 0xd733a
            if fl.ne() { pc = 0xd733a; continue; }
            pc = 0xd731a;
        }
        0xd731a => {
            // block d731a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d731a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d731a lea rdi,[rax+r12*1+0x31]
            rdi = rax.wrapping_add(r12).wrapping_add(0x31);
            pc = 0xd731f;
        }
        0xd731f => {
            // block d731f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d731f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d731f lea rsi,[rip+0xe6467] # 0x1bd78d
            rsi = m.env.data_va(859994);
            // d7326 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // d732b call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7330 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d7332 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d7335 jmp 0xd722e
            pc = 0xd722e; continue;
        }
        0xd733a => {
            // block d733a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d733a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d733a test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d733c jne 0xd7343
            if fl.ne() { pc = 0xd7343; continue; }
            pc = 0xd733e;
        }
        0xd733e => {
            // block d733e
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d733e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d733e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // d7341 jmp 0xd734a
            pc = 0xd734a; continue;
        }
        0xd7343 => {
            // block d7343
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7343 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7343 mov r15,QWORD PTR [r14+0x310]
            r15 = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd734a;
        }
        0xd734a => {
            // block d734a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d734a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d734a movzx ecx,BYTE PTR [r15]
            rcx = (m.r8(r15) as u64) as u32 as u64;
            // d734e cmp ecx,0x78
            fl = sub::<32>((rcx as u32 as u64), 0x78u64).1;
            // d7351 jne 0xd735a
            if fl.ne() { pc = 0xd735a; continue; }
            pc = 0xd7353;
        }
        0xd7353 => {
            // block d7353
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7353 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7353 lea rdi,[rax+r12*1+0x51]
            rdi = rax.wrapping_add(r12).wrapping_add(0x51);
            // d7358 jmp 0xd731f
            pc = 0xd731f; continue;
        }
        0xd735a => {
            // block d735a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d735a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d735a lea rdi,[rax+r12*1+0x11]
            rdi = rax.wrapping_add(r12).wrapping_add(0x11);
            // d735f jmp 0xd731f
            pc = 0xd731f; continue;
        }
        0xd7361 => {
            // block d7361
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7361 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7361 lea rdi,[rax+r12*1+0x5]
            rdi = rax.wrapping_add(r12).wrapping_add(0x5);
            // d7366 jmp 0xd731f
            pc = 0xd731f; continue;
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

/// parity: P2v @0xd2238 size=13
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d2238<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd2238;
    loop {
        match pc {
        0xd2238 => {
            // block d2238
            #[cfg(feature = "trace")] eprintln!("f_000d2238 pc d2238 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d2238 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d2239 mov rbp,rsp
            rbp = rsp;
            // d223c call 0xd1b6c
            { let r = { let t = m.env.code_va(0xd1b6c); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d2241 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d2243 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d2244 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd2245 size=14
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d2245<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd2245;
    loop {
        match pc {
        0xd2245 => {
            // block d2245
            #[cfg(feature = "trace")] eprintln!("f_000d2245 pc d2245 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d2245 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d2246 mov rbp,rsp
            rbp = rsp;
            // d2249 call 0xd1b6c
            { let r = { let t = m.env.code_va(0xd1b6c); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d224e add eax,0xfffffffe
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffffeu64);
              rax = (r) as u32 as u64;
              fl = f; }
            // d2251 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d2252 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7ffc size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7ffc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb7ffc;
    loop {
        match pc {
        0xb7ffc => {
            // block b7ffc
            #[cfg(feature = "trace")] eprintln!("f_000b7ffc pc b7ffc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ffc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7ffd mov rbp,rsp
            rbp = rsp;
            // b8000 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8002 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8003 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b8006 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b8008 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b800b call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8011 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8013 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b8015 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b8018 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8019 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b801b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b801c jmp 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8022 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8022<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb8022;
    loop {
        match pc {
        0xb8022 => {
            // block b8022
            #[cfg(feature = "trace")] eprintln!("f_000b8022 pc b8022 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8022 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8023 mov rbp,rsp
            rbp = rsp;
            // b8026 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8028 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8029 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b802c mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b802e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b8031 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8037 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8039 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b803b mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b803e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b803f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8041 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8042 jmp 0xd1b6c
            { let r = { let t = m.env.code_va(0xd1b6c); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8048 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8048<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb8048;
    loop {
        match pc {
        0xb8048 => {
            // block b8048
            #[cfg(feature = "trace")] eprintln!("f_000b8048 pc b8048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8048 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8049 mov rbp,rsp
            rbp = rsp;
            // b804c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b804e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b804f mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b8052 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b8054 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b8057 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b805d mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b805f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b8061 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b8064 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8065 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8067 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8068 jmp 0xd2238
            { let r = { let t = m.env.code_va(0xd2238); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb806e size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b806e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb806e;
    loop {
        match pc {
        0xb806e => {
            // block b806e
            #[cfg(feature = "trace")] eprintln!("f_000b806e pc b806e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b806e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b806f mov rbp,rsp
            rbp = rsp;
            // b8072 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8074 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8075 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b8078 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b807a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b807d call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8083 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8085 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b8087 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b808a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b808b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b808d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b808e jmp 0xd2245
            { let r = { let t = m.env.code_va(0xd2245); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbc2c size=111
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbc2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbbc2c;
    loop {
        match pc {
        0xbbc2c => {
            // block bbc2c
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbc2d mov rbp,rsp
            rbp = rsp;
            // bbc30 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bbc31 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bbc32 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // bbc34 lea eax,[rdi-0x2]
            rax = (rdi.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // bbc37 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // bbc3a ja 0xbbc68
            if fl.a() { pc = 0xbbc68; continue; }
            pc = 0xbbc3c;
        }
        0xbbc3c => {
            // block bbc3c
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc3c call 0x5c889
            { let r = { let t = m.env.code_va(0x5c889); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbc41 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // bbc43 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bbc48 cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // bbc4b ja 0xbbc94
            if fl.a() { pc = 0xbbc94; continue; }
            pc = 0xbbc4d;
        }
        0xbbc4d => {
            // block bbc4d
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc4d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // bbc4f lea rcx,[rip+0x4707a] # 0x102cd0
            rcx = m.env.data_va(95389);
            // bbc56 lea rdx,[rip+0x47053] # 0x102cb0
            rdx = m.env.data_va(95357);
            // bbc5d test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // bbc5f cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbc63 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // bbc66 jmp 0xbbc94
            pc = 0xbbc94; continue;
        }
        0xbbc68 => {
            // block bbc68
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc68 mov eax,0x76
            rax = (0x76u64) as u32 as u64;
            // bbc6d test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // bbc6f je 0xbbc94
            if fl.e() { pc = 0xbbc94; continue; }
            pc = 0xbbc71;
        }
        0xbbc71 => {
            // block bbc71
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc71 cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // bbc74 jne 0xbbc8f
            if fl.ne() { pc = 0xbbc8f; continue; }
            pc = 0xbbc76;
        }
        0xbbc76 => {
            // block bbc76
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc76 movzx ecx,dx
            rcx = ((rdx as u16 as u64)) as u32 as u64;
            // bbc79 mov eax,0x65
            rax = (0x65u64) as u32 as u64;
            // bbc7e cmp ecx,0x63
            fl = sub::<32>((rcx as u32 as u64), 0x63u64).1;
            // bbc81 je 0xbbc94
            if fl.e() { pc = 0xbbc94; continue; }
            pc = 0xbbc83;
        }
        0xbbc83 => {
            // block bbc83
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc83 lea rax,[rip+0x47016] # 0x102ca0
            rax = m.env.data_va(95341);
            // bbc8a mov eax,DWORD PTR [rax+rcx*4]
            rax = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // bbc8d jmp 0xbbc94
            pc = 0xbbc94; continue;
        }
        0xbbc8f => {
            // block bbc8f
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc8f mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            pc = 0xbbc94;
        }
        0xbbc94 => {
            // block bbc94
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc94 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bbc98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bbc99 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbc9a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb7b0 size=131
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eb7b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xeb7b0;
    loop {
        match pc {
        0xeb7b0 => {
            // block eb7b0
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb7b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb7b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb7b1 mov rbp,rsp
            rbp = rsp;
            // eb7b4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb7b6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb7b8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // eb7ba push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eb7bc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb7bd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // eb7be mov r12,r8
            r12 = r8;
            // eb7c1 mov rbx,rcx
            rbx = rcx;
            // eb7c4 mov r13,rdx
            r13 = rdx;
            // eb7c7 mov r14,rsi
            r14 = rsi;
            // eb7ca mov r15,rdi
            r15 = rdi;
            // eb7cd jmp 0xeb7d3
            pc = 0xeb7d3; continue;
        }
        0xeb7cf => {
            // block eb7cf
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb7cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb7cf add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            pc = 0xeb7d3;
        }
        0xeb7d3 => {
            // block eb7d3
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb7d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb7d3 cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // eb7d6 je 0xeb824
            if fl.e() { pc = 0xeb824; continue; }
            pc = 0xeb7d8;
        }
        0xeb7d8 => {
            // block eb7d8
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb7d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb7d8 cmp r13,rbx
            fl = sub::<64>(r13, rbx).1;
            // eb7db je 0xeb809
            if fl.e() { pc = 0xeb809; continue; }
            pc = 0xeb7dd;
        }
        0xeb7dd => {
            // block eb7dd
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb7dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb7dd mov eax,DWORD PTR [r13+0x18]
            rax = (m.r32(r13.wrapping_add(0x18)) as u64) as u32 as u64;
            // eb7e1 cmp eax,DWORD PTR [r15+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(r15.wrapping_add(0x18)) as u64).1;
            // eb7e5 jge 0xeb7f8
            if fl.ge() { pc = 0xeb7f8; continue; }
            pc = 0xeb7e7;
        }
        0xeb7e7 => {
            // block eb7e7
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb7e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb7e7 mov rdi,r12
            rdi = r12;
            // eb7ea mov rsi,r13
            rsi = r13;
            // eb7ed call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb7f2 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // eb7f6 jmp 0xeb7cf
            pc = 0xeb7cf; continue;
        }
        0xeb7f8 => {
            // block eb7f8
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb7f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb7f8 mov rdi,r12
            rdi = r12;
            // eb7fb mov rsi,r15
            rsi = r15;
            // eb7fe call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb803 add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            // eb807 jmp 0xeb7cf
            pc = 0xeb7cf; continue;
        }
        0xeb809 => {
            // block eb809
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb809 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb809 sub r14,r15
            { let (r, f) = sub::<64>(r14, r15);
              r14 = r;
              fl = f; }
            // eb80c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xeb80e;
        }
        0xeb80e => {
            // block eb80e
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb80e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb80e lea rsi,[r15+rbx*1]
            rsi = r15.wrapping_add(rbx);
            // eb812 lea rdi,[r12+rbx*1]
            rdi = r12.wrapping_add(rbx);
            // eb816 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb81b add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // eb81f cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // eb822 jne 0xeb80e
            if fl.ne() { pc = 0xeb80e; continue; }
            pc = 0xeb824;
        }
        0xeb824 => {
            // block eb824
            #[cfg(feature = "trace")] eprintln!("f_000eb7b0 pc eb824 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb824 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // eb828 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb829 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eb82b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eb82d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb82f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb831 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb832 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc038 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc038<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbc038;
    loop {
        match pc {
        0xbc038 => {
            // block bc038
            #[cfg(feature = "trace")] eprintln!("f_000bc038 pc bc038 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc038 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc039 mov rbp,rsp
            rbp = rsp;
            // bc03c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc03e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc040 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc042 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc043 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc047 mov r14,rsi
            r14 = rsi;
            // bc04a mov r12d,edi
            r12 = ((rdi as u32 as u64)) as u32 as u64;
            // bc04d lea rbx,[r14+0x8]
            rbx = r14.wrapping_add(0x8);
            // bc051 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // bc055 mov rdi,r14
            rdi = r14;
            // bc058 call 0xebd80
            { let r = { let t = m.env.stub_va(0x54d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc05d mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // bc065 mov QWORD PTR [r14],rbx
            m.w64(r14, (rbx) as u64);
            // bc068 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // bc070 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // bc077 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // bc07c lea r15,[rbp-0x24]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            pc = 0xbc080;
        }
        0xbc080 => {
            // block bc080
            #[cfg(feature = "trace")] eprintln!("f_000bc038 pc bc080 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc080 lea esi,[rbx-0x1]
            rsi = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // bc083 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // bc086 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc08b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // bc08d je 0xbc09a
            if fl.e() { pc = 0xbc09a; continue; }
            pc = 0xbc08f;
        }
        0xbc08f => {
            // block bc08f
            #[cfg(feature = "trace")] eprintln!("f_000bc038 pc bc08f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc08f mov rdi,r14
            rdi = r14;
            // bc092 mov rsi,r15
            rsi = r15;
            // bc095 call 0xebd74
            { let r = { let t = m.env.stub_va(0x541); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc09a;
        }
        0xbc09a => {
            // block bc09a
            #[cfg(feature = "trace")] eprintln!("f_000bc038 pc bc09a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc09a mov DWORD PTR [rbp-0x24],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rbx as u32 as u64)) as u32);
            // bc09d cmp ebx,0x6c
            fl = sub::<32>((rbx as u32 as u64), 0x6cu64).1;
            // bc0a0 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // bc0a3 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // bc0a5 jb 0xbc080
            if fl.b() { pc = 0xbc080; continue; }
            pc = 0xbc0a7;
        }
        0xbc0a7 => {
            // block bc0a7
            #[cfg(feature = "trace")] eprintln!("f_000bc038 pc bc0a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0a7 cmp QWORD PTR [r14+0x10],0x0
            fl = sub::<64>(m.r64(r14.wrapping_add(0x10)) as u64, 0x0u64).1;
            // bc0ac setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc0af add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc0b3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc0b4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc0b6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc0b8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc0ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc0bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
