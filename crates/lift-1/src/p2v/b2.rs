//! Mechanically lifted functions (scripts/lift_run.py), module b2. Each one is checked against the original
//! in the world harness (tests/world_b2.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xafd60 size=3096
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000afd60<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xafd60;
    loop {
        match pc {
        0xafd60 => {
            // block afd60
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afd60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd60 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // afd61 mov rbp,rsp
            rbp = rsp;
            // afd64 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // afd66 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // afd68 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // afd6a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // afd6c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // afd6d sub rsp,0x1b8
            { let (r, f) = sub::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // afd74 mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // afd77 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // afd7a mov QWORD PTR [rbp-0x1c0],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe40), (rdi) as u64);
            // afd81 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // afd88 lea r15,[rbp-0x190]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // afd8f mov rax,QWORD PTR [rip+0x9043a] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // afd96 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // afd9a mov QWORD PTR [rbp-0x198],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rcx) as u64);
            // afda1 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // afda5 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // afdac mov rsi,r15
            rsi = r15;
            // afdaf call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afdb4 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // afdbf mov DWORD PTR [rbp-0x98],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0xffffffffu64) as u32);
            // afdc9 mov rbx,QWORD PTR [rip+0x905f0] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // afdd0 lea r13,[rbx+0x18]
            r13 = rbx.wrapping_add(0x18);
            // afdd4 mov QWORD PTR [rbp-0x198],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (r13) as u64);
            // afddb add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // afddf mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // afde6 mov rdi,r15
            rdi = r15;
            // afde9 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afdee mov QWORD PTR [rbp-0x1d0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (r13) as u64);
            // afdf5 mov QWORD PTR [rbp-0x1c8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (rbx) as u64);
            // afdfc mov rax,QWORD PTR [rip+0x905a5] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // afe03 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // afe07 mov QWORD PTR [rbp-0x1d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (rax) as u64);
            // afe0e mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // afe15 mov QWORD PTR [rbp-0x138],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (0x0u64) as u64);
            // afe20 mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // afe2b mov QWORD PTR [rbp-0x148],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (0x0u64) as u64);
            // afe36 mov QWORD PTR [rbp-0x150],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0x0u64) as u64);
            // afe41 mov DWORD PTR [rbp-0x130],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x10u64) as u32);
            // afe4b xorps xmm0,xmm0
            x0 = 0;
            // afe4e movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // afe55 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // afe5d lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // afe64 mov rdi,r15
            rdi = r15;
            // afe67 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afe6c lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // afe73 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afe78 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // afe7b call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afe80 mov DWORD PTR [rbp-0x1b8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe48), ((rax as u32 as u64)) as u32);
            // afe86 cmp DWORD PTR [rbp-0x1b8],0x2
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64, 0x2u64).1;
            // afe8d jne 0xafe9f
            if fl.ne() { pc = 0xafe9f; continue; }
            pc = 0xafe8f;
        }
        0xafe8f => {
            // block afe8f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afe8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afe8f mov r15d,r12d
            r15 = ((r12 as u32 as u64)) as u32 as u64;
            // afe92 mov r13d,r15d
            r13 = ((r15 as u32 as u64)) as u32 as u64;
            // afe95 shr r13d,0x4
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x4u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // afe99 and r13d,0x3f
            { let r = (r13 as u32 as u64) & 0x3fu64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // afe9d jmp 0xafead
            pc = 0xafead; continue;
        }
        0xafe9f => {
            // block afe9f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afe9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afe9f mov r15d,r12d
            r15 = ((r12 as u32 as u64)) as u32 as u64;
            // afea2 mov r13d,r15d
            r13 = ((r15 as u32 as u64)) as u32 as u64;
            // afea5 shr r13d,0x18
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x18u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // afea9 and r13d,0x7
            { let r = (r13 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xafead;
        }
        0xafead => {
            // block afead
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afead rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afead mov al,BYTE PTR [rip+0x986bd] # 0x148570
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(380221)) as u64) as u8 as u64);
            // afeb3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // afeb5 jne 0xb000f
            if fl.ne() { pc = 0xb000f; continue; }
            pc = 0xafebb;
        }
        0xafebb => {
            // block afebb
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afebb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afebb lea rdi,[rip+0x986ae] # 0x148570
            rdi = m.env.data_va(380221);
            // afec2 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afec7 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // afec9 je 0xb000f
            if fl.e() { pc = 0xb000f; continue; }
            pc = 0xafecf;
        }
        0xafecf => {
            // block afecf
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afecf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afecf lea r14,[rip+0x9839a] # 0x148270
            r14 = m.env.data_va(379453);
            // afed6 lea rsi,[rip+0x7f872] # 0x12f74f
            rsi = m.env.data_va(278300);
            // afedd mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // afee2 mov rdi,r14
            rdi = r14;
            // afee5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afeea lea r12,[rip+0x98397] # 0x148288
            r12 = m.env.data_va(379477);
            // afef1 lea rsi,[rip+0x7f870] # 0x12f768
            rsi = m.env.data_va(278325);
            // afef8 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // afefd mov rdi,r12
            rdi = r12;
            // aff00 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff05 lea r12,[rip+0x98394] # 0x1482a0
            r12 = m.env.data_va(379501);
            // aff0c lea rsi,[rip+0x84e7f] # 0x134d92
            rsi = m.env.data_va(300383);
            // aff13 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // aff18 mov rdi,r12
            rdi = r12;
            // aff1b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff20 lea r12,[rip+0x98391] # 0x1482b8
            r12 = m.env.data_va(379525);
            // aff27 lea rsi,[rip+0x84e69] # 0x134d97
            rsi = m.env.data_va(300388);
            // aff2e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // aff33 mov rdi,r12
            rdi = r12;
            // aff36 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff3b lea r12,[rip+0x9838e] # 0x1482d0
            r12 = m.env.data_va(379549);
            // aff42 lea rsi,[rip+0x7f864] # 0x12f7ad
            rsi = m.env.data_va(278394);
            // aff49 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // aff4e mov rdi,r12
            rdi = r12;
            // aff51 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff56 lea r12,[rip+0x9838b] # 0x1482e8
            r12 = m.env.data_va(379573);
            // aff5d lea rsi,[rip+0x84e38] # 0x134d9c
            rsi = m.env.data_va(300393);
            // aff64 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // aff69 mov rdi,r12
            rdi = r12;
            // aff6c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff71 lea r12,[rip+0x98388] # 0x148300
            r12 = m.env.data_va(379597);
            // aff78 lea rsi,[rip+0x84ee1] # 0x134e60
            rsi = m.env.data_va(300589);
            // aff7f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aff84 mov rdi,r12
            rdi = r12;
            // aff87 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff8c lea r12,[rip+0x98385] # 0x148318
            r12 = m.env.data_va(379621);
            // aff93 lea rsi,[rip+0x84ec8] # 0x134e62
            rsi = m.env.data_va(300591);
            // aff9a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aff9f mov rdi,r12
            rdi = r12;
            // affa2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // affa7 lea r12,[rip+0x98382] # 0x148330
            r12 = m.env.data_va(379645);
            // affae lea rsi,[rip+0x84eaf] # 0x134e64
            rsi = m.env.data_va(300593);
            // affb5 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // affba mov rdi,r12
            rdi = r12;
            // affbd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // affc2 lea r12,[rip+0x9837f] # 0x148348
            r12 = m.env.data_va(379669);
            // affc9 lea rsi,[rip+0x84e9a] # 0x134e6a
            rsi = m.env.data_va(300599);
            // affd0 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // affd5 mov rdi,r12
            rdi = r12;
            // affd8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // affdd lea rdi,[rip+0x9837c] # 0x148360
            rdi = m.env.data_va(379693);
            // affe4 mov esi,0x210
            rsi = (0x210u64) as u32 as u64;
            // affe9 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // affee lea rdi,[rip+0xfffffffffffb8771] # 0x68766
            rdi = m.env.code_va(0x68766);
            // afff5 lea rdx,[rip+0xfffffffffff4f174] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // afffc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // afffe call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0003 lea rdi,[rip+0x98566] # 0x148570
            rdi = m.env.data_va(380221);
            // b000a call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb000f;
        }
        0xb000f => {
            // block b000f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b000f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b000f mov al,BYTE PTR [rip+0x9886b] # 0x148880
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(381005)) as u64) as u8 as u64);
            // b0015 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b0017 jne 0xb013b
            if fl.ne() { pc = 0xb013b; continue; }
            pc = 0xb001d;
        }
        0xb001d => {
            // block b001d
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b001d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b001d lea rdi,[rip+0x9885c] # 0x148880
            rdi = m.env.data_va(381005);
            // b0024 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0029 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b002b je 0xb013b
            if fl.e() { pc = 0xb013b; continue; }
            pc = 0xb0031;
        }
        0xb0031 => {
            // block b0031
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0031 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0031 lea r14,[rip+0x98548] # 0x148580
            r14 = m.env.data_va(380237);
            // b0038 lea rsi,[rip+0x84e31] # 0x134e70
            rsi = m.env.data_va(300605);
            // b003f mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // b0044 mov rdi,r14
            rdi = r14;
            // b0047 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b004c lea rbx,[rip+0x98545] # 0x148598
            rbx = m.env.data_va(380261);
            // b0053 lea rsi,[rip+0x836c5] # 0x13371f
            rsi = m.env.data_va(294636);
            // b005a mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b005f mov rdi,rbx
            rdi = rbx;
            // b0062 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0067 lea rbx,[rip+0x98542] # 0x1485b0
            rbx = m.env.data_va(380285);
            // b006e lea rsi,[rip+0x836b0] # 0x133725
            rsi = m.env.data_va(294642);
            // b0075 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b007a mov rdi,rbx
            rdi = rbx;
            // b007d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0082 lea rbx,[rip+0x9853f] # 0x1485c8
            rbx = m.env.data_va(380309);
            // b0089 lea rsi,[rip+0x8369b] # 0x13372b
            rsi = m.env.data_va(294648);
            // b0090 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b0095 mov rdi,rbx
            rdi = rbx;
            // b0098 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b009d lea rbx,[rip+0x9853c] # 0x1485e0
            rbx = m.env.data_va(380333);
            // b00a4 lea rsi,[rip+0x83686] # 0x133731
            rsi = m.env.data_va(294654);
            // b00ab mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b00b0 mov rdi,rbx
            rdi = rbx;
            // b00b3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b00b8 lea rbx,[rip+0x98539] # 0x1485f8
            rbx = m.env.data_va(380357);
            // b00bf lea rsi,[rip+0x83671] # 0x133737
            rsi = m.env.data_va(294660);
            // b00c6 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b00cb mov rdi,rbx
            rdi = rbx;
            // b00ce call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b00d3 lea rbx,[rip+0x98536] # 0x148610
            rbx = m.env.data_va(380381);
            // b00da lea rsi,[rip+0x8365c] # 0x13373d
            rsi = m.env.data_va(294666);
            // b00e1 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b00e6 mov rdi,rbx
            rdi = rbx;
            // b00e9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b00ee lea rbx,[rip+0x98533] # 0x148628
            rbx = m.env.data_va(380405);
            // b00f5 lea rsi,[rip+0x83647] # 0x133743
            rsi = m.env.data_va(294672);
            // b00fc mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b0101 mov rdi,rbx
            rdi = rbx;
            // b0104 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0109 lea rdi,[rip+0x98530] # 0x148640
            rdi = m.env.data_va(380429);
            // b0110 mov esi,0x240
            rsi = (0x240u64) as u32 as u64;
            // b0115 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b011a lea rdi,[rip+0xfffffffffffb8670] # 0x68791
            rdi = m.env.code_va(0x68791);
            // b0121 lea rdx,[rip+0xfffffffffff4f048] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // b0128 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b012a call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b012f lea rdi,[rip+0x9874a] # 0x148880
            rdi = m.env.data_va(381005);
            // b0136 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb013b;
        }
        0xb013b => {
            // block b013b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b013b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b013b lea rsi,[rip+0x84d36] # 0x134e78
            rsi = m.env.data_va(300613);
            // b0142 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b0149 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // b014e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0153 lea rcx,[rip+0x84d2b] # 0x134e85
            rcx = m.env.data_va(300626);
            // b015a lea rsi,[rip+0x84d2b] # 0x134e8c
            rsi = m.env.data_va(300633);
            // b0161 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b0164 and edx,0x1
            { let r = (rdx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0167 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b016b add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // b016e xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // b0172 add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // b0176 mov rdi,rax
            rdi = rax;
            // b0179 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b017e mov rbx,rax
            rbx = rax;
            // b0181 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0184 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0188 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b018b lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b018f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0194 mov rsi,QWORD PTR [rip+0x8fff5] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b019b lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b019f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01a4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b01a7 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b01ab mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b01b0 mov rdi,rax
            rdi = rax;
            // b01b3 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01b5 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b01b8 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b01bc call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01c1 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b01c5 mov rdi,rbx
            rdi = rbx;
            // b01c8 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01cd mov rdi,rbx
            rdi = rbx;
            // b01d0 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01d5 lea rsi,[rip+0x84c9c] # 0x134e78
            rsi = m.env.data_va(300613);
            // b01dc mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // b01e1 mov rdi,rbx
            rdi = rbx;
            // b01e4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01e9 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b01ec and edx,0x2
            { let r = (rdx as u32 as u64) & 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b01ef lea rcx,[rip+0x84c9f] # 0x134e95
            rcx = m.env.data_va(300642);
            // b01f6 lea rsi,[rip+0x84c9f] # 0x134e9c
            rsi = m.env.data_va(300649);
            // b01fd cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b0201 xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // b0205 add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // b0209 mov rdi,rax
            rdi = rax;
            // b020c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0211 mov rbx,rax
            rbx = rax;
            // b0214 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0217 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b021b add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b021e lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // b0222 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0227 mov rsi,QWORD PTR [rip+0x8ff62] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b022e lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // b0232 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0237 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b023a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b023e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0243 mov rdi,rax
            rdi = rax;
            // b0246 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0248 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b024b lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // b024f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0254 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b0258 mov rdi,rbx
            rdi = rbx;
            // b025b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0260 mov rdi,rbx
            rdi = rbx;
            // b0263 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0268 lea rsi,[rip+0x84b5b] # 0x134dca
            rsi = m.env.data_va(300439);
            // b026f mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // b0274 mov rdi,rbx
            rdi = rbx;
            // b0277 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b027c mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b027f and edx,0x4
            { let r = (rdx as u32 as u64) & 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0282 lea rcx,[rip+0x7a990] # 0x12ac19
            rcx = m.env.data_va(259046);
            // b0289 lea rsi,[rip+0x7a979] # 0x12ac09
            rsi = m.env.data_va(259030);
            // b0290 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b0294 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b0297 xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // b029b add rdx,0x3
            { let (r, f) = add::<64>(rdx, 0x3u64);
              rdx = r;
              fl = f; }
            // b029f mov rdi,rax
            rdi = rax;
            // b02a2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02a7 mov rbx,rax
            rbx = rax;
            // b02aa mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b02ad mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b02b1 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b02b4 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b02b8 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02bd mov rsi,QWORD PTR [rip+0x8fecc] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b02c4 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b02c8 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02cd mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b02d0 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b02d4 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b02d9 mov rdi,rax
            rdi = rax;
            // b02dc call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02de mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b02e1 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b02e5 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02ea movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b02ee mov rdi,rbx
            rdi = rbx;
            // b02f1 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02f6 mov rdi,rbx
            rdi = rbx;
            // b02f9 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02fe lea rsi,[rip+0x84ba0] # 0x134ea5
            rsi = m.env.data_va(300658);
            // b0305 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // b030a mov rdi,rbx
            rdi = rbx;
            // b030d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0312 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b0315 and edx,0x8
            { let r = (rdx as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0318 lea rcx,[rip+0x84add] # 0x134dfc
            rcx = m.env.data_va(300489);
            // b031f lea rsi,[rip+0x84add] # 0x134e03
            rsi = m.env.data_va(300496);
            // b0326 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b032a shr rdx,0x3
            { let (r, f) = shr::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // b032e add rdx,0x5
            { let (r, f) = add::<64>(rdx, 0x5u64);
              rdx = r;
              fl = f; }
            // b0332 mov rdi,rax
            rdi = rax;
            // b0335 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b033a mov rbx,rax
            rbx = rax;
            // b033d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0340 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0344 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0347 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b034b call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0350 mov rsi,QWORD PTR [rip+0x8fe39] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0357 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b035b call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0360 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0363 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0367 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b036c mov rdi,rax
            rdi = rax;
            // b036f call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0371 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0374 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b0378 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b037d movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b0381 mov rdi,rbx
            rdi = rbx;
            // b0384 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0389 mov rdi,rbx
            rdi = rbx;
            // b038c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0391 lea rsi,[rip+0x84aa8] # 0x134e40
            rsi = m.env.data_va(300557);
            // b0398 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // b039d mov rdi,rbx
            rdi = rbx;
            // b03a0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03a5 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // b03a8 test ch,0x10
            fl = logic::<8>(((rcx >> 8) as u8 as u64) & 0x10u64);
            // b03ab mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // b03b0 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // b03b5 cmovne esi,ecx
            if fl.ne() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // b03b8 mov rdi,rax
            rdi = rax;
            // b03bb call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03c0 mov rbx,rax
            rbx = rax;
            // b03c3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b03c6 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b03ca add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b03cd lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b03d1 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03d6 mov rsi,QWORD PTR [rip+0x8fdb3] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b03dd lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b03e1 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03e6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b03e9 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b03ed mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b03f2 mov rdi,rax
            rdi = rax;
            // b03f5 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03f7 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b03fa lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b03fe call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0403 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b0407 mov rdi,rbx
            rdi = rbx;
            // b040a call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b040f mov rdi,rbx
            rdi = rbx;
            // b0412 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0417 lea rsi,[rip+0x849c6] # 0x134de4
            rsi = m.env.data_va(300465);
            // b041e mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // b0423 mov rdi,rbx
            rdi = rbx;
            // b0426 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b042b mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b042e and edx,0x2000
            { let r = (rdx as u32 as u64) & 0x2000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0434 lea rcx,[rip+0x83e8a] # 0x1342c5
            rcx = m.env.data_va(297618);
            // b043b lea rsi,[rip+0x83e8f] # 0x1342d1
            rsi = m.env.data_va(297630);
            // b0442 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b0446 shr rdx,0xd
            { let (r, f) = shr::<64>(rdx, 0xdu64, fl); rdx = r; fl = f; }
            // b044a or rdx,0xa
            { let r = rdx | 0xau64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // b044e mov rdi,rax
            rdi = rax;
            // b0451 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0456 mov rbx,rax
            rbx = rax;
            // b0459 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b045c mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0460 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0463 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0467 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b046c mov rsi,QWORD PTR [rip+0x8fd1d] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0473 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0477 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b047c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b047f mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0483 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0488 mov rdi,rax
            rdi = rax;
            // b048b call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b048d mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0490 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0494 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0499 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b049d mov rdi,rbx
            rdi = rbx;
            // b04a0 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04a5 mov rdi,rbx
            rdi = rbx;
            // b04a8 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04ad lea rsi,[rip+0x835a5] # 0x133a59
            rsi = m.env.data_va(295462);
            // b04b4 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // b04b9 mov rdi,rbx
            rdi = rbx;
            // b04bc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04c1 lea rcx,[rip+0x849e8] # 0x134eb0
            rcx = m.env.data_va(300669);
            // b04c8 lea rsi,[rip+0x849e4] # 0x134eb3
            rsi = m.env.data_va(300672);
            // b04cf mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b04d2 test dh,0x40
            fl = logic::<8>(((rdx >> 8) as u8 as u64) & 0x40u64);
            // b04d5 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b04d9 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // b04de mov rdi,rax
            rdi = rax;
            // b04e1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04e6 mov rbx,rax
            rbx = rax;
            // b04e9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b04ec mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b04f0 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b04f3 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b04f7 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04fc mov rsi,QWORD PTR [rip+0x8fc8d] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0503 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0507 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b050c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b050f mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0513 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0518 mov rdi,rax
            rdi = rax;
            // b051b call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b051d mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0520 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0524 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0529 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b052d mov rdi,rbx
            rdi = rbx;
            // b0530 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0535 mov rdi,rbx
            rdi = rbx;
            // b0538 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b053d mov eax,DWORD PTR [rbp-0x1b8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64) as u32 as u64;
            // b0543 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // b0545 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // b0548 ja 0xb05d7
            if fl.a() { pc = 0xb05d7; continue; }
            pc = 0xb054e;
        }
        0xb054e => {
            // block b054e
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b054e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b054e lea rsi,[rip+0x84864] # 0x134db9
            rsi = m.env.data_va(300422);
            // b0555 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b055c mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // b0561 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0566 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // b0569 lea rcx,[rcx+rcx*2]
            rcx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // b056d lea rdx,[rip+0x97cfc] # 0x148270
            rdx = m.env.data_va(379453);
            // b0574 lea rsi,[rdx+rcx*8]
            rsi = rdx.wrapping_add(rcx.wrapping_mul(8));
            // b0578 mov rdi,rax
            rdi = rax;
            // b057b call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0580 mov rbx,rax
            rbx = rax;
            // b0583 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0586 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b058a add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b058d lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0591 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0596 mov rsi,QWORD PTR [rip+0x8fbf3] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b059d lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b05a1 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05a6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b05a9 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b05ad mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b05b2 mov rdi,rax
            rdi = rax;
            // b05b5 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05b7 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b05ba lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b05be call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05c3 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b05c7 mov rdi,rbx
            rdi = rbx;
            // b05ca call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05cf mov rdi,rbx
            rdi = rbx;
            // b05d2 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb05d7;
        }
        0xb05d7 => {
            // block b05d7
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b05d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b05d7 lea rsi,[rip+0x848d8] # 0x134eb6
            rsi = m.env.data_va(300675);
            // b05de lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b05e5 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // b05ea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05ef mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // b05f2 and ecx,0x8000000
            { let r = (rcx as u32 as u64) & 0x8000000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b05f8 lea rdx,[rip+0x8485d] # 0x134e5c
            rdx = m.env.data_va(300585);
            // b05ff lea rsi,[rip+0x7f0cf] # 0x12f6d5
            rsi = m.env.data_va(278178);
            // b0606 cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // b060a shr rcx,0x1b
            { let (r, f) = shr::<64>(rcx, 0x1bu64, fl); rcx = r; fl = f; }
            // b060e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // b0613 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // b0616 mov rdi,rax
            rdi = rax;
            // b0619 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b061e mov rbx,rax
            rbx = rax;
            // b0621 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0624 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0628 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b062b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b062f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0634 mov rsi,QWORD PTR [rip+0x8fb55] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b063b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b063f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0644 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0647 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b064b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0650 mov rdi,rax
            rdi = rax;
            // b0653 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0655 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0658 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b065c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0661 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b0665 mov rdi,rbx
            rdi = rbx;
            // b0668 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b066d mov rdi,rbx
            rdi = rbx;
            // b0670 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0675 lea rsi,[rip+0x84846] # 0x134ec2
            rsi = m.env.data_va(300687);
            // b067c mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // b0681 mov rdi,rbx
            rdi = rbx;
            // b0684 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0689 mov rbx,rax
            rbx = rax;
            // b068c test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // b068f js 0xb06b5
            if fl.s() { pc = 0xb06b5; continue; }
            pc = 0xb0691;
        }
        0xb0691 => {
            // block b0691
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0691 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0691 shr r15d,0x1c
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x1cu64, fl); r15 = (r) as u32 as u64; fl = f; }
            // b0695 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // b0698 lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // b069c lea rcx,[rip+0x97edd] # 0x148580
            rcx = m.env.data_va(380237);
            // b06a3 lea rsi,[rcx+rax*8]
            rsi = rcx.wrapping_add(rax.wrapping_mul(8));
            // b06a7 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // b06ae call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b06b3 jmp 0xb06cd
            pc = 0xb06cd; continue;
        }
        0xb06b5 => {
            // block b06b5
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06b5 lea rsi,[rip+0x847b4] # 0x134e70
            rsi = m.env.data_va(300605);
            // b06bc lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // b06c3 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // b06c8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb06cd;
        }
        0xb06cd => {
            // block b06cd
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06cd movzx edx,BYTE PTR [rbp-0x1b0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64) as u32 as u64;
            // b06d4 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b06d7 jne 0xb06e5
            if fl.ne() { pc = 0xb06e5; continue; }
            pc = 0xb06d9;
        }
        0xb06d9 => {
            // block b06d9
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06d9 lea rsi,[rbp-0x1af]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe51);
            // b06e0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b06e3 jmp 0xb06f3
            pc = 0xb06f3; continue;
        }
        0xb06e5 => {
            // block b06e5
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06e5 mov rdx,QWORD PTR [rbp-0x1a8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // b06ec mov rsi,QWORD PTR [rbp-0x1a0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            pc = 0xb06f3;
        }
        0xb06f3 => {
            // block b06f3
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06f3 mov rdi,rbx
            rdi = rbx;
            // b06f6 lea rbx,[rbp-0x190]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // b06fd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0702 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // b0709 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b070e mov rdi,QWORD PTR [rbp-0x1c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // b0715 mov rsi,rbx
            rsi = rbx;
            // b0718 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b071d mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // b0724 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // b072b mov rax,QWORD PTR [rbp-0x1c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // b0732 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // b0739 mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // b0740 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // b0747 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b074e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0753 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // b075a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b075f mov rsi,QWORD PTR [rip+0x8faa2] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b0766 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b076a lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b0771 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0776 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // b077d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0782 mov rax,QWORD PTR [rbp-0x1c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // b0789 add rsp,0x1b8
            { let (r, f) = add::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // b0790 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b0791 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b0793 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b0795 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b0797 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b0799 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b079a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb079b => {
            // block b079b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b079b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b079b mov r15,rax
            r15 = rax;
            pc = 0xb079e;
        }
        0xb079e => {
            // block b079e
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b079e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b079e mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // b07a5 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // b07ac mov rax,QWORD PTR [rbp-0x1c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // b07b3 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // b07ba mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // b07c1 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // b07c8 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b07cf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b07d4 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // b07db call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b07e0 mov rsi,QWORD PTR [rip+0x8fa21] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b07e7 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b07eb lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b07f2 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b07f7 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            pc = 0xb07fe;
        }
        0xb07fe => {
            // block b07fe
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b07fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b07fe call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0803 mov rdi,r15
            rdi = r15;
            // b0806 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b080b mov r15,rax
            r15 = rax;
            pc = 0xb080e;
        }
        0xb080e => {
            // block b080e
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b080e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b080e add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // b0812 mov rdi,r12
            rdi = r12;
            // b0815 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b081a cmp r14,r12
            fl = sub::<64>(r14, r12).1;
            // b081d jne 0xb080e
            if fl.ne() { pc = 0xb080e; continue; }
            pc = 0xb081f;
        }
        0xb081f => {
            // block b081f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b081f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b081f jmp 0xb0952
            pc = 0xb0952; continue;
        }
        0xb0824 => {
            // block b0824
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0824 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0824 mov r15,rax
            r15 = rax;
            // b0827 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b082b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0830 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0835 => {
            // block b0835
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0835 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0835 mov r15,rax
            r15 = rax;
            // b0838 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // b083c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0841 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0846 => {
            // block b0846
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0846 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0846 mov r15,rax
            r15 = rax;
            // b0849 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b084d call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0852 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0857 => {
            // block b0857
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0857 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0857 mov r15,rax
            r15 = rax;
            // b085a lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b085e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0863 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0868 => {
            // block b0868
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0868 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0868 mov r15,rax
            r15 = rax;
            // b086b lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b086f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0874 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0879 => {
            // block b0879
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0879 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0879 mov r15,rax
            r15 = rax;
            // b087c lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0880 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0885 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb088a => {
            // block b088a
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b088a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b088a mov r15,rax
            r15 = rax;
            // b088d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0891 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0896 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb089b => {
            // block b089b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b089b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b089b mov r15,rax
            r15 = rax;
            // b089e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b08a2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b08a7 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb08ac => {
            // block b08ac
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08ac mov r15,rax
            r15 = rax;
            pc = 0xb08af;
        }
        0xb08af => {
            // block b08af
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08af add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // b08b3 mov rdi,rbx
            rdi = rbx;
            // b08b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b08bb cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // b08be jne 0xb08af
            if fl.ne() { pc = 0xb08af; continue; }
            pc = 0xb08c0;
        }
        0xb08c0 => {
            // block b08c0
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08c0 jmp 0xb0966
            pc = 0xb0966; continue;
        }
        0xb08c5 => {
            // block b08c5
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08c5 lea rbx,[rbp-0x128]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // b08cc mov r15,rax
            r15 = rax;
            // b08cf jmp 0xb0922
            pc = 0xb0922; continue;
        }
        0xb08d1 => {
            // block b08d1
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08d1 lea rbx,[rbp-0x128]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // b08d8 mov r15,rax
            r15 = rax;
            // b08db jmp 0xb090b
            pc = 0xb090b; continue;
        }
        0xb08dd => {
            // block b08dd
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08dd lea rbx,[rbp-0x128]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // b08e4 mov r15,rax
            r15 = rax;
            // b08e7 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // b08ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b08f3 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b08fa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b08ff lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // b0906 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb090b;
        }
        0xb090b => {
            // block b090b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b090b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b090b mov rsi,QWORD PTR [rip+0x8f8f6] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b0912 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b0916 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b091d call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb0922;
        }
        0xb0922 => {
            // block b0922
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0922 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0922 mov rdi,rbx
            rdi = rbx;
            // b0925 jmp 0xb07fe
            pc = 0xb07fe; continue;
        }
        0xb092a => {
            // block b092a
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b092a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b092a mov r15,rax
            r15 = rax;
            // b092d lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0931 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0936 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb093b => {
            // block b093b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b093b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b093b mov r15,rax
            r15 = rax;
            // b093e lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // b0945 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b094a jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb094f => {
            // block b094f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b094f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b094f mov r15,rax
            r15 = rax;
            pc = 0xb0952;
        }
        0xb0952 => {
            // block b0952
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0952 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0952 lea rdi,[rip+0x97c17] # 0x148570
            rdi = m.env.data_va(380221);
            // b0959 call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b095e jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0963 => {
            // block b0963
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0963 mov r15,rax
            r15 = rax;
            pc = 0xb0966;
        }
        0xb0966 => {
            // block b0966
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0966 lea rdi,[rip+0x97f13] # 0x148880
            rdi = m.env.data_va(381005);
            // b096d call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0972 jmp 0xb079e
            pc = 0xb079e; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xaef04 size=3666
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000aef04<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xaef04;
    loop {
        match pc {
        0xaef04 => {
            // block aef04
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc aef04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aef04 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aef05 mov rbp,rsp
            rbp = rsp;
            // aef08 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // aef0a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // aef0c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // aef0e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // aef10 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // aef11 sub rsp,0x1b8
            { let (r, f) = sub::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // aef18 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // aef1b mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // aef1e mov r13,rdi
            r13 = rdi;
            // aef21 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // aef28 lea r14,[rbp-0x190]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // aef2f mov rax,QWORD PTR [rip+0x9129a] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // aef36 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // aef3a mov QWORD PTR [rbp-0x198],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rcx) as u64);
            // aef41 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // aef45 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // aef4c mov rsi,r14
            rsi = r14;
            // aef4f call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aef54 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // aef5f mov DWORD PTR [rbp-0x98],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0xffffffffu64) as u32);
            // aef69 mov rbx,QWORD PTR [rip+0x91450] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // aef70 lea rax,[rbx+0x18]
            rax = rbx.wrapping_add(0x18);
            // aef74 mov QWORD PTR [rbp-0x1d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (rax) as u64);
            // aef7b mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // aef82 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // aef86 mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // aef8d mov rdi,r14
            rdi = r14;
            // aef90 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aef95 mov QWORD PTR [rbp-0x1e0],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe20), (rbx) as u64);
            // aef9c mov rax,QWORD PTR [rip+0x91405] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // aefa3 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // aefa7 mov QWORD PTR [rbp-0x1d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (rax) as u64);
            // aefae mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // aefb5 mov QWORD PTR [rbp-0x138],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (0x0u64) as u64);
            // aefc0 mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // aefcb mov QWORD PTR [rbp-0x148],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (0x0u64) as u64);
            // aefd6 mov QWORD PTR [rbp-0x150],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0x0u64) as u64);
            // aefe1 mov DWORD PTR [rbp-0x130],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x10u64) as u32);
            // aefeb xorps xmm0,xmm0
            x0 = 0;
            // aefee movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // aeff5 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // aeffd lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // af004 mov rdi,r14
            rdi = r14;
            // af007 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af00c lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // af013 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af018 mov al,BYTE PTR [rip+0x98fda] # 0x147ff8
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(378821)) as u64) as u8 as u64);
            // af01e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af020 jne 0xaf115
            if fl.ne() { pc = 0xaf115; continue; }
            pc = 0xaf026;
        }
        0xaf026 => {
            // block af026
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af026 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af026 lea rdi,[rip+0x98fcb] # 0x147ff8
            rdi = m.env.data_va(378821);
            // af02d call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af032 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // af034 je 0xaf115
            if fl.e() { pc = 0xaf115; continue; }
            pc = 0xaf03a;
        }
        0xaf03a => {
            // block af03a
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af03a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af03a lea r14,[rip+0x98f0f] # 0x147f50
            r14 = m.env.data_va(378653);
            // af041 lea rsi,[rip+0x80707] # 0x12f74f
            rsi = m.env.data_va(278300);
            // af048 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af04d mov rdi,r14
            rdi = r14;
            // af050 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af055 lea rbx,[rip+0x98f0c] # 0x147f68
            rbx = m.env.data_va(378677);
            // af05c lea rsi,[rip+0x80705] # 0x12f768
            rsi = m.env.data_va(278325);
            // af063 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af068 mov rdi,rbx
            rdi = rbx;
            // af06b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af070 lea rbx,[rip+0x98f09] # 0x147f80
            rbx = m.env.data_va(378701);
            // af077 lea rsi,[rip+0x85d14] # 0x134d92
            rsi = m.env.data_va(300383);
            // af07e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af083 mov rdi,rbx
            rdi = rbx;
            // af086 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af08b lea rbx,[rip+0x98f06] # 0x147f98
            rbx = m.env.data_va(378725);
            // af092 lea rsi,[rip+0x85cfe] # 0x134d97
            rsi = m.env.data_va(300388);
            // af099 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af09e mov rdi,rbx
            rdi = rbx;
            // af0a1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af0a6 lea rbx,[rip+0x98f03] # 0x147fb0
            rbx = m.env.data_va(378749);
            // af0ad lea rsi,[rip+0x806f9] # 0x12f7ad
            rsi = m.env.data_va(278394);
            // af0b4 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af0b9 mov rdi,rbx
            rdi = rbx;
            // af0bc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af0c1 lea rbx,[rip+0x98f00] # 0x147fc8
            rbx = m.env.data_va(378773);
            // af0c8 lea rsi,[rip+0x85ccd] # 0x134d9c
            rsi = m.env.data_va(300393);
            // af0cf mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af0d4 mov rdi,rbx
            rdi = rbx;
            // af0d7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af0dc lea rbx,[rip+0x98efd] # 0x147fe0
            rbx = m.env.data_va(378797);
            // af0e3 lea rsi,[rip+0x89bde] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af0ea xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af0ec mov rdi,rbx
            rdi = rbx;
            // af0ef call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af0f4 lea rdi,[rip+0xfffffffffffb95ea] # 0x686e5
            rdi = m.env.code_va(0x686e5);
            // af0fb lea rdx,[rip+0xfffffffffff5006e] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // af102 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // af104 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af109 lea rdi,[rip+0x98ee8] # 0x147ff8
            rdi = m.env.data_va(378821);
            // af110 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf115;
        }
        0xaf115 => {
            // block af115
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af115 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af115 mov al,BYTE PTR [rip+0x98fed] # 0x148108
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(379093)) as u64) as u8 as u64);
            // af11b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af11d jne 0xaf27e
            if fl.ne() { pc = 0xaf27e; continue; }
            pc = 0xaf123;
        }
        0xaf123 => {
            // block af123
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af123 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af123 lea rdi,[rip+0x98fde] # 0x148108
            rdi = m.env.data_va(379093);
            // af12a call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af12f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // af131 je 0xaf27e
            if fl.e() { pc = 0xaf27e; continue; }
            pc = 0xaf137;
        }
        0xaf137 => {
            // block af137
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af137 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af137 lea rbx,[rip+0x98ec2] # 0x148000
            rbx = m.env.data_va(378829);
            // af13e lea rsi,[rip+0x8060a] # 0x12f74f
            rsi = m.env.data_va(278300);
            // af145 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af14a mov rdi,rbx
            rdi = rbx;
            // af14d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af152 lea r14,[rip+0x98ebf] # 0x148018
            r14 = m.env.data_va(378853);
            // af159 lea rsi,[rip+0x80608] # 0x12f768
            rsi = m.env.data_va(278325);
            // af160 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af165 mov rdi,r14
            rdi = r14;
            // af168 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af16d lea r14,[rip+0x98ebc] # 0x148030
            r14 = m.env.data_va(378877);
            // af174 lea rsi,[rip+0x85c17] # 0x134d92
            rsi = m.env.data_va(300383);
            // af17b mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af180 mov rdi,r14
            rdi = r14;
            // af183 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af188 lea r14,[rip+0x98eb9] # 0x148048
            r14 = m.env.data_va(378901);
            // af18f lea rsi,[rip+0x85c01] # 0x134d97
            rsi = m.env.data_va(300388);
            // af196 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af19b mov rdi,r14
            rdi = r14;
            // af19e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af1a3 lea r14,[rip+0x98eb6] # 0x148060
            r14 = m.env.data_va(378925);
            // af1aa lea rsi,[rip+0x805fc] # 0x12f7ad
            rsi = m.env.data_va(278394);
            // af1b1 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af1b6 mov rdi,r14
            rdi = r14;
            // af1b9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af1be lea r14,[rip+0x98eb3] # 0x148078
            r14 = m.env.data_va(378949);
            // af1c5 lea rsi,[rip+0x85bd5] # 0x134da1
            rsi = m.env.data_va(300398);
            // af1cc mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af1d1 mov rdi,r14
            rdi = r14;
            // af1d4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af1d9 lea r14,[rip+0x98eb0] # 0x148090
            r14 = m.env.data_va(378973);
            // af1e0 lea rsi,[rip+0x85bc0] # 0x134da7
            rsi = m.env.data_va(300404);
            // af1e7 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // af1ec mov rdi,r14
            rdi = r14;
            // af1ef call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af1f4 lea r14,[rip+0x98ead] # 0x1480a8
            r14 = m.env.data_va(378997);
            // af1fb lea rsi,[rip+0x85bae] # 0x134db0
            rsi = m.env.data_va(300413);
            // af202 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // af207 mov rdi,r14
            rdi = r14;
            // af20a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af20f lea r14,[rip+0x98eaa] # 0x1480c0
            r14 = m.env.data_va(379021);
            // af216 lea rsi,[rip+0x805fc] # 0x12f819
            rsi = m.env.data_va(278502);
            // af21d mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // af222 mov rdi,r14
            rdi = r14;
            // af225 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af22a lea r14,[rip+0x98ea7] # 0x1480d8
            r14 = m.env.data_va(379045);
            // af231 lea rsi,[rip+0x805fe] # 0x12f836
            rsi = m.env.data_va(278531);
            // af238 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // af23d mov rdi,r14
            rdi = r14;
            // af240 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af245 lea r14,[rip+0x98ea4] # 0x1480f0
            r14 = m.env.data_va(379069);
            // af24c lea rsi,[rip+0x89a75] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af253 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af255 mov rdi,r14
            rdi = r14;
            // af258 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af25d lea rdi,[rip+0xfffffffffffb94ac] # 0x68710
            rdi = m.env.code_va(0x68710);
            // af264 lea rdx,[rip+0xfffffffffff4ff05] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // af26b xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // af26d call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af272 lea rdi,[rip+0x98e8f] # 0x148108
            rdi = m.env.data_va(379093);
            // af279 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf27e;
        }
        0xaf27e => {
            // block af27e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af27e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af27e mov al,BYTE PTR [rip+0x98f94] # 0x148218
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(379365)) as u64) as u8 as u64);
            // af284 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af286 jne 0xaf3de
            if fl.ne() { pc = 0xaf3de; continue; }
            pc = 0xaf28c;
        }
        0xaf28c => {
            // block af28c
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af28c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af28c lea rdi,[rip+0x98f85] # 0x148218
            rdi = m.env.data_va(379365);
            // af293 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af298 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // af29a je 0xaf3de
            if fl.e() { pc = 0xaf3de; continue; }
            pc = 0xaf2a0;
        }
        0xaf2a0 => {
            // block af2a0
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af2a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af2a0 lea r14,[rip+0x98e69] # 0x148110
            r14 = m.env.data_va(379101);
            // af2a7 lea rsi,[rip+0x89a1a] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af2ae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af2b0 mov rdi,r14
            rdi = r14;
            // af2b3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af2b8 lea rbx,[rip+0x98e69] # 0x148128
            rbx = m.env.data_va(379125);
            // af2bf lea rsi,[rip+0x84459] # 0x13371f
            rsi = m.env.data_va(294636);
            // af2c6 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af2cb mov rdi,rbx
            rdi = rbx;
            // af2ce call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af2d3 lea rbx,[rip+0x98e66] # 0x148140
            rbx = m.env.data_va(379149);
            // af2da lea rsi,[rip+0x84444] # 0x133725
            rsi = m.env.data_va(294642);
            // af2e1 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af2e6 mov rdi,rbx
            rdi = rbx;
            // af2e9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af2ee lea rbx,[rip+0x98e63] # 0x148158
            rbx = m.env.data_va(379173);
            // af2f5 lea rsi,[rip+0x8442f] # 0x13372b
            rsi = m.env.data_va(294648);
            // af2fc mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af301 mov rdi,rbx
            rdi = rbx;
            // af304 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af309 lea rbx,[rip+0x98e60] # 0x148170
            rbx = m.env.data_va(379197);
            // af310 lea rsi,[rip+0x8441a] # 0x133731
            rsi = m.env.data_va(294654);
            // af317 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af31c mov rdi,rbx
            rdi = rbx;
            // af31f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af324 lea rbx,[rip+0x98e5d] # 0x148188
            rbx = m.env.data_va(379221);
            // af32b lea rsi,[rip+0x84405] # 0x133737
            rsi = m.env.data_va(294660);
            // af332 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af337 mov rdi,rbx
            rdi = rbx;
            // af33a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af33f lea rbx,[rip+0x98e5a] # 0x1481a0
            rbx = m.env.data_va(379245);
            // af346 lea rsi,[rip+0x843f0] # 0x13373d
            rsi = m.env.data_va(294666);
            // af34d mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af352 mov rdi,rbx
            rdi = rbx;
            // af355 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af35a lea rbx,[rip+0x98e57] # 0x1481b8
            rbx = m.env.data_va(379269);
            // af361 lea rsi,[rip+0x843db] # 0x133743
            rsi = m.env.data_va(294672);
            // af368 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af36d mov rdi,rbx
            rdi = rbx;
            // af370 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af375 lea rbx,[rip+0x98e54] # 0x1481d0
            rbx = m.env.data_va(379293);
            // af37c lea rsi,[rip+0x89945] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af383 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af385 mov rdi,rbx
            rdi = rbx;
            // af388 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af38d lea rbx,[rip+0x98e54] # 0x1481e8
            rbx = m.env.data_va(379317);
            // af394 lea rsi,[rip+0x8992d] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af39b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af39d mov rdi,rbx
            rdi = rbx;
            // af3a0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3a5 lea rbx,[rip+0x98e54] # 0x148200
            rbx = m.env.data_va(379341);
            // af3ac lea rsi,[rip+0x89915] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af3b3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af3b5 mov rdi,rbx
            rdi = rbx;
            // af3b8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3bd lea rdi,[rip+0xfffffffffffb9377] # 0x6873b
            rdi = m.env.code_va(0x6873b);
            // af3c4 lea rdx,[rip+0xfffffffffff4fda5] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // af3cb xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // af3cd call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3d2 lea rdi,[rip+0x98e3f] # 0x148218
            rdi = m.env.data_va(379365);
            // af3d9 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf3de;
        }
        0xaf3de => {
            // block af3de
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af3de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af3de mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // af3e1 call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3e6 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // af3e9 lea rcx,[rip+0x53080] # 0x102470
            rcx = m.env.data_va(93245);
            // af3f0 mov ebx,DWORD PTR [rcx+rax*4]
            rbx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // af3f3 and ebx,r12d
            { let r = (rbx as u32 as u64) & (r12 as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // af3f6 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // af3f9 jb 0xaf41a
            if fl.b() { pc = 0xaf41a; continue; }
            pc = 0xaf3fb;
        }
        0xaf3fb => {
            // block af3fb
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af3fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af3fb mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // af3fd lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // af401 lea rcx,[rip+0x98bf8] # 0x148000
            rcx = m.env.data_va(378829);
            // af408 lea rsi,[rcx+rax*8]
            rsi = rcx.wrapping_add(rax.wrapping_mul(8));
            // af40c lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // af413 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af418 jmp 0xaf453
            pc = 0xaf453; continue;
        }
        0xaf41a => {
            // block af41a
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af41a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af41a cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // af41d jne 0xaf43e
            if fl.ne() { pc = 0xaf43e; continue; }
            pc = 0xaf41f;
        }
        0xaf41f => {
            // block af41f
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af41f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af41f mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // af421 lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // af425 lea rcx,[rip+0x98b24] # 0x147f50
            rcx = m.env.data_va(378653);
            // af42c lea rsi,[rcx+rax*8]
            rsi = rcx.wrapping_add(rax.wrapping_mul(8));
            // af430 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // af437 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af43c jmp 0xaf453
            pc = 0xaf453; continue;
        }
        0xaf43e => {
            // block af43e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af43e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af43e lea rsi,[rip+0x89883] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af445 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // af44c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af44e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf453;
        }
        0xaf453 => {
            // block af453
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af453 lea rdi,[rbp-0x1c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // af45a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // af45f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // af461 call 0x5b5c9
            { let r = { let t = m.env.code_va(0x5b5c9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af466 lea rsi,[rip+0x8594c] # 0x134db9
            rsi = m.env.data_va(300422);
            // af46d lea rbx,[rbp-0x198]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af474 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // af479 mov rdi,rbx
            rdi = rbx;
            // af47c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af481 movzx edx,BYTE PTR [rbp-0x1b0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64) as u32 as u64;
            // af488 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // af48b jne 0xaf499
            if fl.ne() { pc = 0xaf499; continue; }
            pc = 0xaf48d;
        }
        0xaf48d => {
            // block af48d
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af48d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af48d lea rsi,[rbp-0x1af]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe51);
            // af494 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // af497 jmp 0xaf4a7
            pc = 0xaf4a7; continue;
        }
        0xaf499 => {
            // block af499
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af499 mov rdx,QWORD PTR [rbp-0x1a8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // af4a0 mov rsi,QWORD PTR [rbp-0x1a0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            pc = 0xaf4a7;
        }
        0xaf4a7 => {
            // block af4a7
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4a7 mov rdi,rax
            rdi = rax;
            // af4aa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af4af movzx edx,BYTE PTR [rbp-0x1b0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64) as u32 as u64;
            // af4b6 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // af4b8 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // af4ba jne 0xaf4c1
            if fl.ne() { pc = 0xaf4c1; continue; }
            pc = 0xaf4bc;
        }
        0xaf4bc => {
            // block af4bc
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4bc shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // af4bf jmp 0xaf4c8
            pc = 0xaf4c8; continue;
        }
        0xaf4c1 => {
            // block af4c1
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4c1 mov rdx,QWORD PTR [rbp-0x1a8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            pc = 0xaf4c8;
        }
        0xaf4c8 => {
            // block af4c8
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4c8 movzx esi,BYTE PTR [rbp-0x1c8]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64) as u32 as u64;
            // af4cf mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // af4d2 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // af4d5 jne 0xaf4dc
            if fl.ne() { pc = 0xaf4dc; continue; }
            pc = 0xaf4d7;
        }
        0xaf4d7 => {
            // block af4d7
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4d7 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // af4da jmp 0xaf4e3
            pc = 0xaf4e3; continue;
        }
        0xaf4dc => {
            // block af4dc
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4dc mov rsi,QWORD PTR [rbp-0x1c0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            pc = 0xaf4e3;
        }
        0xaf4e3 => {
            // block af4e3
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4e3 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // af4e6 jne 0xaf53e
            if fl.ne() { pc = 0xaf53e; continue; }
            pc = 0xaf4e8;
        }
        0xaf4e8 => {
            // block af4e8
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4e8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af4ea jne 0xaf4f5
            if fl.ne() { pc = 0xaf4f5; continue; }
            pc = 0xaf4ec;
        }
        0xaf4ec => {
            // block af4ec
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4ec lea rdi,[rbp-0x1af]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe51);
            // af4f3 jmp 0xaf4fc
            pc = 0xaf4fc; continue;
        }
        0xaf4f5 => {
            // block af4f5
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4f5 mov rdi,QWORD PTR [rbp-0x1a0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            pc = 0xaf4fc;
        }
        0xaf4fc => {
            // block af4fc
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4fc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // af4fe jne 0xaf509
            if fl.ne() { pc = 0xaf509; continue; }
            pc = 0xaf500;
        }
        0xaf500 => {
            // block af500
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af500 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af500 lea rsi,[rbp-0x1c7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe39);
            // af507 jmp 0xaf510
            pc = 0xaf510; continue;
        }
        0xaf509 => {
            // block af509
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af509 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af509 mov rsi,QWORD PTR [rbp-0x1b8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            pc = 0xaf510;
        }
        0xaf510 => {
            // block af510
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af510 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af512 jne 0xaf530
            if fl.ne() { pc = 0xaf530; continue; }
            pc = 0xaf514;
        }
        0xaf514 => {
            // block af514
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af514 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af514 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // af517 je 0xaf598
            if fl.e() { pc = 0xaf598; continue; }
            pc = 0xaf519;
        }
        0xaf519 => {
            // block af519
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af519 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af519 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // af51c movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // af51f cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // af521 jne 0xaf53e
            if fl.ne() { pc = 0xaf53e; continue; }
            pc = 0xaf523;
        }
        0xaf523 => {
            // block af523
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af523 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af523 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // af526 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // af529 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // af52c jne 0xaf519
            if fl.ne() { pc = 0xaf519; continue; }
            pc = 0xaf52e;
        }
        0xaf52e => {
            // block af52e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af52e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af52e jmp 0xaf598
            pc = 0xaf598; continue;
        }
        0xaf530 => {
            // block af530
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af530 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af530 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // af533 je 0xaf598
            if fl.e() { pc = 0xaf598; continue; }
            pc = 0xaf535;
        }
        0xaf535 => {
            // block af535
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af535 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af535 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af53a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // af53c je 0xaf598
            if fl.e() { pc = 0xaf598; continue; }
            pc = 0xaf53e;
        }
        0xaf53e => {
            // block af53e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af53e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af53e lea rsi,[rip+0x84a22] # 0x133f67
            rsi = m.env.data_va(296756);
            // af545 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af54c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // af551 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af556 movzx edx,BYTE PTR [rbp-0x1c8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64) as u32 as u64;
            // af55d test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // af560 jne 0xaf56e
            if fl.ne() { pc = 0xaf56e; continue; }
            pc = 0xaf562;
        }
        0xaf562 => {
            // block af562
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af562 lea rsi,[rbp-0x1c7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe39);
            // af569 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // af56c jmp 0xaf57c
            pc = 0xaf57c; continue;
        }
        0xaf56e => {
            // block af56e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af56e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af56e mov rdx,QWORD PTR [rbp-0x1c0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // af575 mov rsi,QWORD PTR [rbp-0x1b8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            pc = 0xaf57c;
        }
        0xaf57c => {
            // block af57c
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af57c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af57c mov rdi,rax
            rdi = rax;
            // af57f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af584 lea rsi,[rip+0x7b0f7] # 0x12a682
            rsi = m.env.data_va(257615);
            // af58b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // af590 mov rdi,rax
            rdi = rax;
            // af593 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf598;
        }
        0xaf598 => {
            // block af598
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af598 mov rax,QWORD PTR [rbp-0x198]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            // af59f add rbx,QWORD PTR [rax-0x18]
            { let (r, f) = add::<64>(rbx, m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64);
              rbx = r;
              fl = f; }
            // af5a3 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // af5a7 mov rsi,rbx
            rsi = rbx;
            // af5aa call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5af mov rsi,QWORD PTR [rip+0x90bda] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af5b6 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // af5ba call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5bf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af5c2 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af5c6 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af5cb mov rdi,rax
            rdi = rax;
            // af5ce call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5d0 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af5d2 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // af5d6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5db movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // af5de lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af5e5 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5ea lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af5f1 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5f6 lea rsi,[rip+0x857cd] # 0x134dca
            rsi = m.env.data_va(300439);
            // af5fd lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af604 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // af609 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af60e mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // af611 and edx,0x100
            { let r = (rdx as u32 as u64) & 0x100u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af617 lea rcx,[rip+0x7b5fb] # 0x12ac19
            rcx = m.env.data_va(259046);
            // af61e lea rsi,[rip+0x7b5e4] # 0x12ac09
            rsi = m.env.data_va(259030);
            // af625 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af629 shr rdx,0x7
            { let (r, f) = shr::<64>(rdx, 0x7u64, fl); rdx = r; fl = f; }
            // af62d xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // af631 add rdx,0x3
            { let (r, f) = add::<64>(rdx, 0x3u64);
              rdx = r;
              fl = f; }
            // af635 mov rdi,rax
            rdi = rax;
            // af638 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af63d mov rbx,rax
            rbx = rax;
            // af640 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af643 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af647 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af64a lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // af64e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af653 mov rsi,QWORD PTR [rip+0x90b36] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af65a lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // af65e call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af663 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af666 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af66a mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af66f mov rdi,rax
            rdi = rax;
            // af672 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af674 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af677 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // af67b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af680 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af684 mov rdi,rbx
            rdi = rbx;
            // af687 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af68c mov rdi,rbx
            rdi = rbx;
            // af68f call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af694 lea rsi,[rip+0x8573c] # 0x134dd7
            rsi = m.env.data_va(300452);
            // af69b mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // af6a0 mov rdi,rbx
            rdi = rbx;
            // af6a3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6a8 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // af6ab shr ecx,0x9
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x9u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // af6ae and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // af6b1 lea rcx,[rcx+rcx*2]
            rcx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // af6b5 lea rdx,[rip+0x98a54] # 0x148110
            rdx = m.env.data_va(379101);
            // af6bc lea rsi,[rdx+rcx*8]
            rsi = rdx.wrapping_add(rcx.wrapping_mul(8));
            // af6c0 mov rdi,rax
            rdi = rax;
            // af6c3 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6c8 mov rbx,rax
            rbx = rax;
            // af6cb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af6ce mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af6d2 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af6d5 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // af6d9 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6de mov rsi,QWORD PTR [rip+0x90aab] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af6e5 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // af6e9 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6ee mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af6f1 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af6f5 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af6fa mov rdi,rax
            rdi = rax;
            // af6fd call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6ff mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af702 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // af706 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af70b movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af70f mov rdi,rbx
            rdi = rbx;
            // af712 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af717 mov rdi,rbx
            rdi = rbx;
            // af71a call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af71f lea rsi,[rip+0x856be] # 0x134de4
            rsi = m.env.data_va(300465);
            // af726 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // af72b mov rdi,rbx
            rdi = rbx;
            // af72e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af733 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // af736 and edx,0x2000
            { let r = (rdx as u32 as u64) & 0x2000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af73c lea rcx,[rip+0x84b82] # 0x1342c5
            rcx = m.env.data_va(297618);
            // af743 lea rsi,[rip+0x84b87] # 0x1342d1
            rsi = m.env.data_va(297630);
            // af74a cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af74e shr rdx,0xd
            { let (r, f) = shr::<64>(rdx, 0xdu64, fl); rdx = r; fl = f; }
            // af752 or rdx,0xa
            { let r = rdx | 0xau64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // af756 mov rdi,rax
            rdi = rax;
            // af759 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af75e mov rbx,rax
            rbx = rax;
            // af761 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af764 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af768 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af76b lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // af76f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af774 mov rsi,QWORD PTR [rip+0x90a15] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af77b lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // af77f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af784 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af787 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af78b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af790 mov rdi,rax
            rdi = rax;
            // af793 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af795 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af798 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // af79c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7a1 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af7a5 mov rdi,rbx
            rdi = rbx;
            // af7a8 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7ad mov rdi,rbx
            rdi = rbx;
            // af7b0 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7b5 lea rsi,[rip+0x85634] # 0x134df0
            rsi = m.env.data_va(300477);
            // af7bc mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // af7c1 mov rdi,rbx
            rdi = rbx;
            // af7c4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7c9 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // af7cc and edx,0x4000
            { let r = (rdx as u32 as u64) & 0x4000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af7d2 lea rcx,[rip+0x85623] # 0x134dfc
            rcx = m.env.data_va(300489);
            // af7d9 lea rsi,[rip+0x85623] # 0x134e03
            rsi = m.env.data_va(300496);
            // af7e0 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af7e4 shr rdx,0xe
            { let (r, f) = shr::<64>(rdx, 0xeu64, fl); rdx = r; fl = f; }
            // af7e8 add rdx,0x5
            { let (r, f) = add::<64>(rdx, 0x5u64);
              rdx = r;
              fl = f; }
            // af7ec mov rdi,rax
            rdi = rax;
            // af7ef call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7f4 mov rbx,rax
            rbx = rax;
            // af7f7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af7fa mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af7fe add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af801 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // af805 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af80a mov rsi,QWORD PTR [rip+0x9097f] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af811 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // af815 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af81a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af81d mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af821 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af826 mov rdi,rax
            rdi = rax;
            // af829 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af82b mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af82e lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // af832 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af837 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af83b mov rdi,rbx
            rdi = rbx;
            // af83e call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af843 mov rdi,rbx
            rdi = rbx;
            // af846 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af84b lea rsi,[rip+0x855b7] # 0x134e09
            rsi = m.env.data_va(300502);
            // af852 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // af857 mov rdi,rbx
            rdi = rbx;
            // af85a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af85f lea rcx,[rip+0x855b4] # 0x134e1a
            rcx = m.env.data_va(300519);
            // af866 lea rsi,[rip+0x855b3] # 0x134e20
            rsi = m.env.data_va(300525);
            // af86d mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // af870 test dh,0x80
            fl = logic::<8>(((rdx >> 8) as u8 as u64) & 0x80u64);
            // af873 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af877 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af87c mov rdi,rax
            rdi = rax;
            // af87f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af884 mov rbx,rax
            rbx = rax;
            // af887 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af88a mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af88e add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af891 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // af895 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af89a mov rsi,QWORD PTR [rip+0x908ef] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af8a1 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // af8a5 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8aa mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af8ad mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af8b1 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af8b6 mov rdi,rax
            rdi = rax;
            // af8b9 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8bb mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af8be lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // af8c2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8c7 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af8cb mov rdi,rbx
            rdi = rbx;
            // af8ce call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8d3 mov rdi,rbx
            rdi = rbx;
            // af8d6 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8db lea rsi,[rip+0x85544] # 0x134e26
            rsi = m.env.data_va(300531);
            // af8e2 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // af8e7 mov rdi,rbx
            rdi = rbx;
            // af8ea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8ef mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // af8f2 and ecx,0x10000000
            { let r = (rcx as u32 as u64) & 0x10000000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // af8f8 lea rdx,[rip+0x85536] # 0x134e35
            rdx = m.env.data_va(300546);
            // af8ff lea rsi,[rip+0x85534] # 0x134e3a
            rsi = m.env.data_va(300551);
            // af906 cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // af90a shr rcx,0x1c
            { let (r, f) = shr::<64>(rcx, 0x1cu64, fl); rcx = r; fl = f; }
            // af90e mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af913 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // af916 mov rdi,rax
            rdi = rax;
            // af919 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af91e mov rbx,rax
            rbx = rax;
            // af921 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af924 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af928 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af92b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // af92f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af934 mov rsi,QWORD PTR [rip+0x90855] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af93b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // af93f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af944 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af947 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af94b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af950 mov rdi,rax
            rdi = rax;
            // af953 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af955 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af958 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // af95c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af961 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af965 mov rdi,rbx
            rdi = rbx;
            // af968 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af96d mov rdi,rbx
            rdi = rbx;
            // af970 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af975 lea rsi,[rip+0x854c4] # 0x134e40
            rsi = m.env.data_va(300557);
            // af97c mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // af981 mov rdi,rbx
            rdi = rbx;
            // af984 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af989 lea rcx,[rip+0x854c1] # 0x134e51
            rcx = m.env.data_va(300574);
            // af990 lea rsi,[rip+0x7b311] # 0x12aca8
            rsi = m.env.data_va(259189);
            // af997 test r12d,0x20000000
            fl = logic::<32>((r12 as u32 as u64) & 0x20000000u64);
            // af99e cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af9a2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // af9a7 mov rdi,rax
            rdi = rax;
            // af9aa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9af mov rbx,rax
            rbx = rax;
            // af9b2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af9b5 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af9b9 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af9bc lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // af9c0 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9c5 mov rsi,QWORD PTR [rip+0x907c4] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af9cc lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // af9d0 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9d5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af9d8 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af9dc mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af9e1 mov rdi,rax
            rdi = rax;
            // af9e4 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9e6 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af9e9 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // af9ed call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9f2 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af9f6 mov rdi,rbx
            rdi = rbx;
            // af9f9 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9fe mov rdi,rbx
            rdi = rbx;
            // afa01 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa06 lea rsi,[rip+0x85446] # 0x134e53
            rsi = m.env.data_va(300576);
            // afa0d mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // afa12 mov rdi,rbx
            rdi = rbx;
            // afa15 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa1a mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // afa1d and ecx,0x40000000
            { let r = (rcx as u32 as u64) & 0x40000000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // afa23 lea rdx,[rip+0x85432] # 0x134e5c
            rdx = m.env.data_va(300585);
            // afa2a lea rsi,[rip+0x7fca4] # 0x12f6d5
            rsi = m.env.data_va(278178);
            // afa31 cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // afa35 shr rcx,0x1e
            { let (r, f) = shr::<64>(rcx, 0x1eu64, fl); rcx = r; fl = f; }
            // afa39 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // afa3e sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // afa41 mov rdi,rax
            rdi = rax;
            // afa44 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa49 mov rbx,rax
            rbx = rax;
            // afa4c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // afa4f mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // afa53 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // afa56 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // afa5a call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa5f mov rsi,QWORD PTR [rip+0x9072a] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // afa66 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // afa6a call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa6f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // afa72 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // afa76 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // afa7b mov rdi,rax
            rdi = rax;
            // afa7e call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa80 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // afa83 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // afa87 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa8c movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // afa90 mov rdi,rbx
            rdi = rbx;
            // afa93 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa98 mov rdi,rbx
            rdi = rbx;
            // afa9b call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afaa0 lea rsi,[rip+0x849f2] # 0x134499
            rsi = m.env.data_va(298086);
            // afaa7 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // afaac mov rdi,rbx
            rdi = rbx;
            // afaaf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afab4 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // afab7 lea rcx,[rip+0x7c2b2] # 0x12bd70
            rcx = m.env.data_va(263485);
            // afabe lea rsi,[rip+0x7c28f] # 0x12bd54
            rsi = m.env.data_va(263457);
            // afac5 cmovs rsi,rcx
            if fl.s() { rsi = rcx; }
            // afac9 setns cl
            rcx = (rcx & !0xff) | (((fl.ns() as u64)) as u8 as u64);
            // afacc movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // afacf or rdx,0x2
            { let r = rdx | 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // afad3 mov rdi,rax
            rdi = rax;
            // afad6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afadb mov rdi,r13
            rdi = r13;
            // afade lea rsi,[rbp-0x190]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // afae5 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afaea lea rdi,[rbp-0x1c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // afaf1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afaf6 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // afafd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb02 mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // afb09 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // afb10 mov rax,QWORD PTR [rbp-0x1e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // afb17 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // afb1e mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // afb25 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // afb2c lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // afb33 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb38 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // afb3f call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb44 mov rsi,QWORD PTR [rip+0x906bd] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // afb4b add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // afb4f lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // afb56 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb5b lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // afb62 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb67 mov rax,r13
            rax = r13;
            // afb6a add rsp,0x1b8
            { let (r, f) = add::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // afb71 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // afb72 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // afb74 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // afb76 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // afb78 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // afb7a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // afb7b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xafb7c => {
            // block afb7c
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afb7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afb7c mov r15,rax
            r15 = rax;
            pc = 0xafb7f;
        }
        0xafb7f => {
            // block afb7f
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afb7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afb7f lea rdi,[rbp-0x1c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // afb86 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xafb8b;
        }
        0xafb8b => {
            // block afb8b
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afb8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afb8b lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // afb92 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xafb97;
        }
        0xafb97 => {
            // block afb97
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afb97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afb97 mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // afb9e mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // afba5 mov rax,QWORD PTR [rbp-0x1e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // afbac mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // afbb3 mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // afbba mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            pc = 0xafbc1;
        }
        0xafbc1 => {
            // block afbc1
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afbc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afbc1 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // afbc8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afbcd lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // afbd4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xafbd9;
        }
        0xafbd9 => {
            // block afbd9
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afbd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afbd9 mov rsi,QWORD PTR [rip+0x90628] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // afbe0 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // afbe4 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // afbeb call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xafbf0;
        }
        0xafbf0 => {
            // block afbf0
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afbf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afbf0 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // afbf7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afbfc mov rdi,r15
            rdi = r15;
            // afbff call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc04 mov r15,rax
            r15 = rax;
            pc = 0xafc07;
        }
        0xafc07 => {
            // block afc07
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc07 add r14,0xffffffffffffffe8
            { let (r, f) = add::<64>(r14, 0xffff_ffff_ffff_ffe8u64);
              r14 = r;
              fl = f; }
            // afc0b mov rdi,r14
            rdi = r14;
            // afc0e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc13 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // afc16 jne 0xafc07
            if fl.ne() { pc = 0xafc07; continue; }
            pc = 0xafc18;
        }
        0xafc18 => {
            // block afc18
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc18 jmp 0xafd30
            pc = 0xafd30; continue;
        }
        0xafc1d => {
            // block afc1d
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc1d mov r15,rax
            r15 = rax;
            pc = 0xafc20;
        }
        0xafc20 => {
            // block afc20
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc20 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // afc24 mov rdi,rbx
            rdi = rbx;
            // afc27 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc2c cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // afc2f jne 0xafc20
            if fl.ne() { pc = 0xafc20; continue; }
            pc = 0xafc31;
        }
        0xafc31 => {
            // block afc31
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc31 jmp 0xafd44
            pc = 0xafd44; continue;
        }
        0xafc36 => {
            // block afc36
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc36 mov r15,rax
            r15 = rax;
            // afc39 jmp 0xafb97
            pc = 0xafb97; continue;
        }
        0xafc3e => {
            // block afc3e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc3e mov r15,rax
            r15 = rax;
            // afc41 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // afc45 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc4a jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc4f => {
            // block afc4f
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc4f mov r15,rax
            r15 = rax;
            // afc52 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // afc56 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc5b jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc60 => {
            // block afc60
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc60 mov r15,rax
            r15 = rax;
            // afc63 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // afc67 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc6c jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc71 => {
            // block afc71
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc71 mov r15,rax
            r15 = rax;
            // afc74 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // afc78 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc7d jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc82 => {
            // block afc82
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc82 mov r15,rax
            r15 = rax;
            // afc85 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // afc89 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc8e jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc93 => {
            // block afc93
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc93 mov r15,rax
            r15 = rax;
            // afc96 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // afc9a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc9f jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafca4 => {
            // block afca4
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afca4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afca4 mov r15,rax
            r15 = rax;
            // afca7 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // afcab call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afcb0 jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafcb5 => {
            // block afcb5
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcb5 mov r15,rax
            r15 = rax;
            // afcb8 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // afcbc call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afcc1 jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafcc6 => {
            // block afcc6
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcc6 mov r15,rax
            r15 = rax;
            // afcc9 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // afccd call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afcd2 jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafcd7 => {
            // block afcd7
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcd7 mov r15,rax
            r15 = rax;
            pc = 0xafcda;
        }
        0xafcda => {
            // block afcda
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcda add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // afcde mov rdi,rbx
            rdi = rbx;
            // afce1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afce6 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // afce9 jne 0xafcda
            if fl.ne() { pc = 0xafcda; continue; }
            pc = 0xafceb;
        }
        0xafceb => {
            // block afceb
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afceb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afceb jmp 0xafd1c
            pc = 0xafd1c; continue;
        }
        0xafced => {
            // block afced
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afced rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afced mov r15,rax
            r15 = rax;
            // afcf0 jmp 0xafbf0
            pc = 0xafbf0; continue;
        }
        0xafcf5 => {
            // block afcf5
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcf5 mov r15,rax
            r15 = rax;
            // afcf8 jmp 0xafbd9
            pc = 0xafbd9; continue;
        }
        0xafcfd => {
            // block afcfd
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcfd mov r15,rax
            r15 = rax;
            // afd00 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // afd07 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afd0c jmp 0xafbc1
            pc = 0xafbc1; continue;
        }
        0xafd11 => {
            // block afd11
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd11 mov r15,rax
            r15 = rax;
            // afd14 jmp 0xafb8b
            pc = 0xafb8b; continue;
        }
        0xafd19 => {
            // block afd19
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd19 mov r15,rax
            r15 = rax;
            pc = 0xafd1c;
        }
        0xafd1c => {
            // block afd1c
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd1c lea rdi,[rip+0x982d5] # 0x147ff8
            rdi = m.env.data_va(378821);
            // afd23 call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afd28 jmp 0xafb97
            pc = 0xafb97; continue;
        }
        0xafd2d => {
            // block afd2d
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd2d mov r15,rax
            r15 = rax;
            pc = 0xafd30;
        }
        0xafd30 => {
            // block afd30
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd30 lea rdi,[rip+0x983d1] # 0x148108
            rdi = m.env.data_va(379093);
            // afd37 call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afd3c jmp 0xafb97
            pc = 0xafb97; continue;
        }
        0xafd41 => {
            // block afd41
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd41 mov r15,rax
            r15 = rax;
            pc = 0xafd44;
        }
        0xafd44 => {
            // block afd44
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd44 lea rdi,[rip+0x984cd] # 0x148218
            rdi = m.env.data_va(379365);
            // afd4b call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afd50 jmp 0xafb97
            pc = 0xafb97; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x408df size=4927
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000408df<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x408df;
    loop {
        match pc {
        0x408df => {
            // block 408df
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 408df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 408df push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 408e0 mov rbp,rsp
            rbp = rsp;
            // 408e3 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 408e5 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 408e7 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 408e9 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 408eb push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 408ec push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 408ed lea r14,[rip+0x18117c] # 0x1c1a70
            r14 = m.env.data_va(877117);
            // 408f4 lea rsi,[rip+0xea30e] # 0x12ac09
            rsi = m.env.data_va(259030);
            // 408fb mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 40900 mov rdi,r14
            rdi = r14;
            // 40903 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40908 lea r12,[rip+0x181179] # 0x1c1a88
            r12 = m.env.data_va(877141);
            // 4090f lea rsi,[rip+0xf83b2] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40916 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40918 mov rdi,r12
            rdi = r12;
            // 4091b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40920 lea r12,[rip+0x181179] # 0x1c1aa0
            r12 = m.env.data_va(877165);
            // 40927 lea rsi,[rip+0xf839a] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 4092e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40930 mov rdi,r12
            rdi = r12;
            // 40933 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40938 lea r12,[rip+0x181179] # 0x1c1ab8
            r12 = m.env.data_va(877189);
            // 4093f lea rsi,[rip+0xf8382] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40946 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40948 mov rdi,r12
            rdi = r12;
            // 4094b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40950 lea r15,[rip+0x181179] # 0x1c1ad0
            r15 = m.env.data_va(877213);
            // 40957 lea rsi,[rip+0xea2ab] # 0x12ac09
            rsi = m.env.data_va(259030);
            // 4095e mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 40963 mov rdi,r15
            rdi = r15;
            // 40966 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4096b lea r12,[rip+0x181176] # 0x1c1ae8
            r12 = m.env.data_va(877237);
            // 40972 lea rsi,[rip+0xf834f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40979 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4097b mov rdi,r12
            rdi = r12;
            // 4097e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40983 lea r12,[rip+0x181176] # 0x1c1b00
            r12 = m.env.data_va(877261);
            // 4098a lea rsi,[rip+0xf8337] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40991 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40993 mov rdi,r12
            rdi = r12;
            // 40996 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4099b lea r12,[rip+0x181176] # 0x1c1b18
            r12 = m.env.data_va(877285);
            // 409a2 lea rsi,[rip+0xf831f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 409a9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 409ab mov rdi,r12
            rdi = r12;
            // 409ae call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 409b3 lea r15,[rip+0x181176] # 0x1c1b30
            r15 = m.env.data_va(877309);
            // 409ba lea rsi,[rip+0xea24e] # 0x12ac0f
            rsi = m.env.data_va(259036);
            // 409c1 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 409c6 mov rdi,r15
            rdi = r15;
            // 409c9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 409ce lea r12,[rip+0x181173] # 0x1c1b48
            r12 = m.env.data_va(877333);
            // 409d5 lea rsi,[rip+0xf82ec] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 409dc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 409de mov rdi,r12
            rdi = r12;
            // 409e1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 409e6 lea r12,[rip+0x181173] # 0x1c1b60
            r12 = m.env.data_va(877357);
            // 409ed lea rsi,[rip+0xf82d4] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 409f4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 409f6 mov rdi,r12
            rdi = r12;
            // 409f9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 409fe lea r12,[rip+0x181173] # 0x1c1b78
            r12 = m.env.data_va(877381);
            // 40a05 lea rsi,[rip+0xf82bc] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40a0c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40a0e mov rdi,r12
            rdi = r12;
            // 40a11 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40a16 lea r15,[rip+0x181173] # 0x1c1b90
            r15 = m.env.data_va(877405);
            // 40a1d lea rsi,[rip+0xea1f0] # 0x12ac14
            rsi = m.env.data_va(259041);
            // 40a24 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 40a29 mov rdi,r15
            rdi = r15;
            // 40a2c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40a31 lea r12,[rip+0x181170] # 0x1c1ba8
            r12 = m.env.data_va(877429);
            // 40a38 lea rsi,[rip+0xf8289] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40a3f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40a41 mov rdi,r12
            rdi = r12;
            // 40a44 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40a49 lea r12,[rip+0x181170] # 0x1c1bc0
            r12 = m.env.data_va(877453);
            // 40a50 lea rsi,[rip+0xf8271] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40a57 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40a59 mov rdi,r12
            rdi = r12;
            // 40a5c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40a61 lea r12,[rip+0x181170] # 0x1c1bd8
            r12 = m.env.data_va(877477);
            // 40a68 lea rsi,[rip+0xf8259] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40a6f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40a71 mov rdi,r12
            rdi = r12;
            // 40a74 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40a79 lea r15,[rip+0x181170] # 0x1c1bf0
            r15 = m.env.data_va(877501);
            // 40a80 lea rsi,[rip+0xea192] # 0x12ac19
            rsi = m.env.data_va(259046);
            // 40a87 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 40a8c mov rdi,r15
            rdi = r15;
            // 40a8f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40a94 lea r12,[rip+0x18116d] # 0x1c1c08
            r12 = m.env.data_va(877525);
            // 40a9b lea rsi,[rip+0xf8226] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40aa2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40aa4 mov rdi,r12
            rdi = r12;
            // 40aa7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40aac lea r12,[rip+0x18116d] # 0x1c1c20
            r12 = m.env.data_va(877549);
            // 40ab3 lea rsi,[rip+0xf820e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40aba xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40abc mov rdi,r12
            rdi = r12;
            // 40abf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40ac4 lea r12,[rip+0x18116d] # 0x1c1c38
            r12 = m.env.data_va(877573);
            // 40acb lea rsi,[rip+0xf81f6] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40ad2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40ad4 mov rdi,r12
            rdi = r12;
            // 40ad7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40adc lea r15,[rip+0x18116d] # 0x1c1c50
            r15 = m.env.data_va(877597);
            // 40ae3 lea rsi,[rip+0xea11f] # 0x12ac09
            rsi = m.env.data_va(259030);
            // 40aea mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 40aef mov rdi,r15
            rdi = r15;
            // 40af2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40af7 lea r12,[rip+0x18116a] # 0x1c1c68
            r12 = m.env.data_va(877621);
            // 40afe lea rsi,[rip+0xf81c3] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40b05 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40b07 mov rdi,r12
            rdi = r12;
            // 40b0a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40b0f lea r12,[rip+0x18116a] # 0x1c1c80
            r12 = m.env.data_va(877645);
            // 40b16 lea rsi,[rip+0xf81ab] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40b1d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40b1f mov rdi,r12
            rdi = r12;
            // 40b22 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40b27 lea r12,[rip+0x18116a] # 0x1c1c98
            r12 = m.env.data_va(877669);
            // 40b2e lea rsi,[rip+0xf8193] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40b35 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40b37 mov rdi,r12
            rdi = r12;
            // 40b3a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40b3f lea r15,[rip+0x18116a] # 0x1c1cb0
            r15 = m.env.data_va(877693);
            // 40b46 lea rsi,[rip+0xea0d0] # 0x12ac1d
            rsi = m.env.data_va(259050);
            // 40b4d mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 40b52 mov rdi,r15
            rdi = r15;
            // 40b55 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40b5a lea r12,[rip+0x181167] # 0x1c1cc8
            r12 = m.env.data_va(877717);
            // 40b61 lea rsi,[rip+0xf8160] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40b68 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40b6a mov rdi,r12
            rdi = r12;
            // 40b6d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40b72 lea r12,[rip+0x181167] # 0x1c1ce0
            r12 = m.env.data_va(877741);
            // 40b79 lea rsi,[rip+0xf8148] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40b80 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40b82 mov rdi,r12
            rdi = r12;
            // 40b85 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40b8a lea r12,[rip+0x181167] # 0x1c1cf8
            r12 = m.env.data_va(877765);
            // 40b91 lea rsi,[rip+0xf8130] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40b98 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40b9a mov rdi,r12
            rdi = r12;
            // 40b9d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40ba2 lea r15,[rip+0x181167] # 0x1c1d10
            r15 = m.env.data_va(877789);
            // 40ba9 lea rsi,[rip+0xf8118] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40bb0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40bb2 mov rdi,r15
            rdi = r15;
            // 40bb5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40bba lea r12,[rip+0x181167] # 0x1c1d28
            r12 = m.env.data_va(877813);
            // 40bc1 lea rsi,[rip+0xf8100] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40bc8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40bca mov rdi,r12
            rdi = r12;
            // 40bcd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40bd2 lea r12,[rip+0x181167] # 0x1c1d40
            r12 = m.env.data_va(877837);
            // 40bd9 lea rsi,[rip+0xf80e8] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40be0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40be2 mov rdi,r12
            rdi = r12;
            // 40be5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40bea lea r12,[rip+0x181167] # 0x1c1d58
            r12 = m.env.data_va(877861);
            // 40bf1 lea rsi,[rip+0xf80d0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40bf8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40bfa mov rdi,r12
            rdi = r12;
            // 40bfd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40c02 lea r15,[rip+0x181167] # 0x1c1d70
            r15 = m.env.data_va(877885);
            // 40c09 lea rsi,[rip+0xe9ff9] # 0x12ac09
            rsi = m.env.data_va(259030);
            // 40c10 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 40c15 mov rdi,r15
            rdi = r15;
            // 40c18 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40c1d lea r12,[rip+0x181164] # 0x1c1d88
            r12 = m.env.data_va(877909);
            // 40c24 lea rsi,[rip+0xf809d] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40c2b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40c2d mov rdi,r12
            rdi = r12;
            // 40c30 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40c35 lea r12,[rip+0x181164] # 0x1c1da0
            r12 = m.env.data_va(877933);
            // 40c3c lea rsi,[rip+0xf8085] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40c43 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40c45 mov rdi,r12
            rdi = r12;
            // 40c48 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40c4d lea r12,[rip+0x181164] # 0x1c1db8
            r12 = m.env.data_va(877957);
            // 40c54 lea rsi,[rip+0xf806d] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40c5b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40c5d mov rdi,r12
            rdi = r12;
            // 40c60 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40c65 lea r15,[rip+0x181164] # 0x1c1dd0
            r15 = m.env.data_va(877981);
            // 40c6c lea rsi,[rip+0xf8055] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40c73 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40c75 mov rdi,r15
            rdi = r15;
            // 40c78 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40c7d lea r12,[rip+0x181164] # 0x1c1de8
            r12 = m.env.data_va(878005);
            // 40c84 lea rsi,[rip+0xf803d] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40c8b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40c8d mov rdi,r12
            rdi = r12;
            // 40c90 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40c95 lea r12,[rip+0x181164] # 0x1c1e00
            r12 = m.env.data_va(878029);
            // 40c9c lea rsi,[rip+0xf8025] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40ca3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40ca5 mov rdi,r12
            rdi = r12;
            // 40ca8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40cad lea r12,[rip+0x181164] # 0x1c1e18
            r12 = m.env.data_va(878053);
            // 40cb4 lea rsi,[rip+0xf800d] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40cbb xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40cbd mov rdi,r12
            rdi = r12;
            // 40cc0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40cc5 lea r15,[rip+0x181164] # 0x1c1e30
            r15 = m.env.data_va(878077);
            // 40ccc lea rsi,[rip+0xe9f4f] # 0x12ac22
            rsi = m.env.data_va(259055);
            // 40cd3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 40cd8 mov rdi,r15
            rdi = r15;
            // 40cdb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40ce0 lea r12,[rip+0x181161] # 0x1c1e48
            r12 = m.env.data_va(878101);
            // 40ce7 lea rsi,[rip+0xe9f36] # 0x12ac24
            rsi = m.env.data_va(259057);
            // 40cee mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 40cf3 mov rdi,r12
            rdi = r12;
            // 40cf6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40cfb lea r12,[rip+0x18115e] # 0x1c1e60
            r12 = m.env.data_va(878125);
            // 40d02 lea rsi,[rip+0xe9f1e] # 0x12ac27
            rsi = m.env.data_va(259060);
            // 40d09 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 40d0e mov rdi,r12
            rdi = r12;
            // 40d11 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40d16 lea r12,[rip+0x18115b] # 0x1c1e78
            r12 = m.env.data_va(878149);
            // 40d1d lea rsi,[rip+0xf7fa4] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40d24 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40d26 mov rdi,r12
            rdi = r12;
            // 40d29 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40d2e lea r15,[rip+0x18115b] # 0x1c1e90
            r15 = m.env.data_va(878173);
            // 40d35 lea rsi,[rip+0xf7f8c] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40d3c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40d3e mov rdi,r15
            rdi = r15;
            // 40d41 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40d46 lea r12,[rip+0x18115b] # 0x1c1ea8
            r12 = m.env.data_va(878197);
            // 40d4d lea rsi,[rip+0xf7f74] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40d54 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40d56 mov rdi,r12
            rdi = r12;
            // 40d59 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40d5e lea r12,[rip+0x18115b] # 0x1c1ec0
            r12 = m.env.data_va(878221);
            // 40d65 lea rsi,[rip+0xf7f5c] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40d6c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40d6e mov rdi,r12
            rdi = r12;
            // 40d71 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40d76 lea r12,[rip+0x18115b] # 0x1c1ed8
            r12 = m.env.data_va(878245);
            // 40d7d lea rsi,[rip+0xf7f44] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40d84 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40d86 mov rdi,r12
            rdi = r12;
            // 40d89 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40d8e lea r15,[rip+0x18115b] # 0x1c1ef0
            r15 = m.env.data_va(878269);
            // 40d95 lea rsi,[rip+0xe9e7d] # 0x12ac19
            rsi = m.env.data_va(259046);
            // 40d9c mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 40da1 mov rdi,r15
            rdi = r15;
            // 40da4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40da9 lea r12,[rip+0x181158] # 0x1c1f08
            r12 = m.env.data_va(878293);
            // 40db0 lea rsi,[rip+0xf7f11] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40db7 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40db9 mov rdi,r12
            rdi = r12;
            // 40dbc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40dc1 lea r12,[rip+0x181158] # 0x1c1f20
            r12 = m.env.data_va(878317);
            // 40dc8 lea rsi,[rip+0xf7ef9] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40dcf xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40dd1 mov rdi,r12
            rdi = r12;
            // 40dd4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40dd9 lea r12,[rip+0x181158] # 0x1c1f38
            r12 = m.env.data_va(878341);
            // 40de0 lea rsi,[rip+0xf7ee1] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40de7 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40de9 mov rdi,r12
            rdi = r12;
            // 40dec call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40df1 lea r15,[rip+0x181158] # 0x1c1f50
            r15 = m.env.data_va(878365);
            // 40df8 lea rsi,[rip+0xe9e2b] # 0x12ac2a
            rsi = m.env.data_va(259063);
            // 40dff mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 40e04 mov rdi,r15
            rdi = r15;
            // 40e07 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40e0c lea r12,[rip+0x181155] # 0x1c1f68
            r12 = m.env.data_va(878389);
            // 40e13 lea rsi,[rip+0xf7eae] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40e1a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40e1c mov rdi,r12
            rdi = r12;
            // 40e1f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40e24 lea r12,[rip+0x181155] # 0x1c1f80
            r12 = m.env.data_va(878413);
            // 40e2b lea rsi,[rip+0xf7e96] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40e32 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40e34 mov rdi,r12
            rdi = r12;
            // 40e37 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40e3c lea r12,[rip+0x181155] # 0x1c1f98
            r12 = m.env.data_va(878437);
            // 40e43 lea rsi,[rip+0xf7e7e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40e4a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40e4c mov rdi,r12
            rdi = r12;
            // 40e4f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40e54 lea r15,[rip+0x181155] # 0x1c1fb0
            r15 = m.env.data_va(878461);
            // 40e5b lea rsi,[rip+0xe9dcc] # 0x12ac2e
            rsi = m.env.data_va(259067);
            // 40e62 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 40e67 mov rdi,r15
            rdi = r15;
            // 40e6a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40e6f lea r12,[rip+0x181152] # 0x1c1fc8
            r12 = m.env.data_va(878485);
            // 40e76 lea rsi,[rip+0xf7e4b] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40e7d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40e7f mov rdi,r12
            rdi = r12;
            // 40e82 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40e87 lea r12,[rip+0x181152] # 0x1c1fe0
            r12 = m.env.data_va(878509);
            // 40e8e lea rsi,[rip+0xf7e33] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40e95 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40e97 mov rdi,r12
            rdi = r12;
            // 40e9a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40e9f lea r12,[rip+0x181152] # 0x1c1ff8
            r12 = m.env.data_va(878533);
            // 40ea6 lea rsi,[rip+0xf7e1b] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40ead xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40eaf mov rdi,r12
            rdi = r12;
            // 40eb2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40eb7 lea r15,[rip+0x181152] # 0x1c2010
            r15 = m.env.data_va(878557);
            // 40ebe lea rsi,[rip+0xf7e03] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40ec5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40ec7 mov rdi,r15
            rdi = r15;
            // 40eca call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40ecf lea r12,[rip+0x181152] # 0x1c2028
            r12 = m.env.data_va(878581);
            // 40ed6 lea rsi,[rip+0xf7deb] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40edd xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40edf mov rdi,r12
            rdi = r12;
            // 40ee2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40ee7 lea r12,[rip+0x181152] # 0x1c2040
            r12 = m.env.data_va(878605);
            // 40eee lea rsi,[rip+0xf7dd3] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40ef5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40ef7 mov rdi,r12
            rdi = r12;
            // 40efa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40eff lea r12,[rip+0x181152] # 0x1c2058
            r12 = m.env.data_va(878629);
            // 40f06 lea rsi,[rip+0xf7dbb] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40f0d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40f0f mov rdi,r12
            rdi = r12;
            // 40f12 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40f17 lea r15,[rip+0x181152] # 0x1c2070
            r15 = m.env.data_va(878653);
            // 40f1e lea rsi,[rip+0xe9cf4] # 0x12ac19
            rsi = m.env.data_va(259046);
            // 40f25 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 40f2a mov rdi,r15
            rdi = r15;
            // 40f2d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40f32 lea r12,[rip+0x18114f] # 0x1c2088
            r12 = m.env.data_va(878677);
            // 40f39 lea rsi,[rip+0xf7d88] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40f40 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40f42 mov rdi,r12
            rdi = r12;
            // 40f45 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40f4a lea r12,[rip+0x18114f] # 0x1c20a0
            r12 = m.env.data_va(878701);
            // 40f51 lea rsi,[rip+0xf7d70] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40f58 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40f5a mov rdi,r12
            rdi = r12;
            // 40f5d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40f62 lea r12,[rip+0x18114f] # 0x1c20b8
            r12 = m.env.data_va(878725);
            // 40f69 lea rsi,[rip+0xf7d58] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40f70 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40f72 mov rdi,r12
            rdi = r12;
            // 40f75 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40f7a lea r15,[rip+0x18114f] # 0x1c20d0
            r15 = m.env.data_va(878749);
            // 40f81 lea rsi,[rip+0xf7d40] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40f88 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40f8a mov rdi,r15
            rdi = r15;
            // 40f8d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40f92 lea r12,[rip+0x18114f] # 0x1c20e8
            r12 = m.env.data_va(878773);
            // 40f99 lea rsi,[rip+0xf7d28] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40fa0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40fa2 mov rdi,r12
            rdi = r12;
            // 40fa5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40faa lea r12,[rip+0x18114f] # 0x1c2100
            r12 = m.env.data_va(878797);
            // 40fb1 lea rsi,[rip+0xf7d10] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40fb8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40fba mov rdi,r12
            rdi = r12;
            // 40fbd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40fc2 lea r12,[rip+0x18114f] # 0x1c2118
            r12 = m.env.data_va(878821);
            // 40fc9 lea rsi,[rip+0xf7cf8] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40fd0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40fd2 mov rdi,r12
            rdi = r12;
            // 40fd5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40fda lea r15,[rip+0x18114f] # 0x1c2130
            r15 = m.env.data_va(878845);
            // 40fe1 lea rsi,[rip+0xf7ce0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 40fe8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 40fea mov rdi,r15
            rdi = r15;
            // 40fed call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 40ff2 lea r12,[rip+0x18114f] # 0x1c2148
            r12 = m.env.data_va(878869);
            // 40ff9 lea rsi,[rip+0xf7cc8] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41000 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41002 mov rdi,r12
            rdi = r12;
            // 41005 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4100a lea r12,[rip+0x18114f] # 0x1c2160
            r12 = m.env.data_va(878893);
            // 41011 lea rsi,[rip+0xf7cb0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41018 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4101a mov rdi,r12
            rdi = r12;
            // 4101d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41022 lea r12,[rip+0x18114f] # 0x1c2178
            r12 = m.env.data_va(878917);
            // 41029 lea rsi,[rip+0xf7c98] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41030 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41032 mov rdi,r12
            rdi = r12;
            // 41035 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4103a lea r15,[rip+0x18114f] # 0x1c2190
            r15 = m.env.data_va(878941);
            // 41041 lea rsi,[rip+0xf7c80] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41048 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4104a mov rdi,r15
            rdi = r15;
            // 4104d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41052 lea r12,[rip+0x18114f] # 0x1c21a8
            r12 = m.env.data_va(878965);
            // 41059 lea rsi,[rip+0xf7c68] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41060 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41062 mov rdi,r12
            rdi = r12;
            // 41065 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4106a lea r12,[rip+0x18114f] # 0x1c21c0
            r12 = m.env.data_va(878989);
            // 41071 lea rsi,[rip+0xf7c50] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41078 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4107a mov rdi,r12
            rdi = r12;
            // 4107d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41082 lea r12,[rip+0x18114f] # 0x1c21d8
            r12 = m.env.data_va(879013);
            // 41089 lea rsi,[rip+0xf7c38] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41090 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41092 mov rdi,r12
            rdi = r12;
            // 41095 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4109a lea r15,[rip+0x18114f] # 0x1c21f0
            r15 = m.env.data_va(879037);
            // 410a1 lea rsi,[rip+0xe9b71] # 0x12ac19
            rsi = m.env.data_va(259046);
            // 410a8 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 410ad mov rdi,r15
            rdi = r15;
            // 410b0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 410b5 lea r12,[rip+0x18114c] # 0x1c2208
            r12 = m.env.data_va(879061);
            // 410bc lea rsi,[rip+0xf7c05] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 410c3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 410c5 mov rdi,r12
            rdi = r12;
            // 410c8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 410cd lea r12,[rip+0x18114c] # 0x1c2220
            r12 = m.env.data_va(879085);
            // 410d4 lea rsi,[rip+0xf7bed] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 410db xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 410dd mov rdi,r12
            rdi = r12;
            // 410e0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 410e5 lea r12,[rip+0x18114c] # 0x1c2238
            r12 = m.env.data_va(879109);
            // 410ec lea rsi,[rip+0xf7bd5] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 410f3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 410f5 mov rdi,r12
            rdi = r12;
            // 410f8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 410fd lea r15,[rip+0x18114c] # 0x1c2250
            r15 = m.env.data_va(879133);
            // 41104 lea rsi,[rip+0xe9b04] # 0x12ac0f
            rsi = m.env.data_va(259036);
            // 4110b mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 41110 mov rdi,r15
            rdi = r15;
            // 41113 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41118 lea r12,[rip+0x181149] # 0x1c2268
            r12 = m.env.data_va(879157);
            // 4111f lea rsi,[rip+0xf7ba2] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41126 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41128 mov rdi,r12
            rdi = r12;
            // 4112b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41130 lea r12,[rip+0x181149] # 0x1c2280
            r12 = m.env.data_va(879181);
            // 41137 lea rsi,[rip+0xf7b8a] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 4113e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41140 mov rdi,r12
            rdi = r12;
            // 41143 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41148 lea r12,[rip+0x181149] # 0x1c2298
            r12 = m.env.data_va(879205);
            // 4114f lea rsi,[rip+0xf7b72] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41156 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41158 mov rdi,r12
            rdi = r12;
            // 4115b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41160 lea r15,[rip+0x181149] # 0x1c22b0
            r15 = m.env.data_va(879229);
            // 41167 lea rsi,[rip+0xe9aa1] # 0x12ac0f
            rsi = m.env.data_va(259036);
            // 4116e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 41173 mov rdi,r15
            rdi = r15;
            // 41176 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4117b lea r12,[rip+0x181146] # 0x1c22c8
            r12 = m.env.data_va(879253);
            // 41182 lea rsi,[rip+0xf7b3f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41189 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4118b mov rdi,r12
            rdi = r12;
            // 4118e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41193 lea r12,[rip+0x181146] # 0x1c22e0
            r12 = m.env.data_va(879277);
            // 4119a lea rsi,[rip+0xf7b27] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 411a1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 411a3 mov rdi,r12
            rdi = r12;
            // 411a6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 411ab lea r12,[rip+0x181146] # 0x1c22f8
            r12 = m.env.data_va(879301);
            // 411b2 lea rsi,[rip+0xf7b0f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 411b9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 411bb mov rdi,r12
            rdi = r12;
            // 411be call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 411c3 lea r15,[rip+0x181146] # 0x1c2310
            r15 = m.env.data_va(879325);
            // 411ca lea rsi,[rip+0xe9a51] # 0x12ac22
            rsi = m.env.data_va(259055);
            // 411d1 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 411d6 mov rdi,r15
            rdi = r15;
            // 411d9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 411de lea r12,[rip+0x181143] # 0x1c2328
            r12 = m.env.data_va(879349);
            // 411e5 lea rsi,[rip+0xe9a38] # 0x12ac24
            rsi = m.env.data_va(259057);
            // 411ec mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 411f1 mov rdi,r12
            rdi = r12;
            // 411f4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 411f9 lea r12,[rip+0x181140] # 0x1c2340
            r12 = m.env.data_va(879373);
            // 41200 lea rsi,[rip+0xe9a20] # 0x12ac27
            rsi = m.env.data_va(259060);
            // 41207 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4120c mov rdi,r12
            rdi = r12;
            // 4120f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41214 lea r12,[rip+0x18113d] # 0x1c2358
            r12 = m.env.data_va(879397);
            // 4121b lea rsi,[rip+0xf7aa6] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41222 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41224 mov rdi,r12
            rdi = r12;
            // 41227 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4122c lea r15,[rip+0x18113d] # 0x1c2370
            r15 = m.env.data_va(879421);
            // 41233 lea rsi,[rip+0xe99df] # 0x12ac19
            rsi = m.env.data_va(259046);
            // 4123a mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4123f mov rdi,r15
            rdi = r15;
            // 41242 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41247 lea r12,[rip+0x18113a] # 0x1c2388
            r12 = m.env.data_va(879445);
            // 4124e lea rsi,[rip+0xf7a73] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41255 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41257 mov rdi,r12
            rdi = r12;
            // 4125a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4125f lea r12,[rip+0x18113a] # 0x1c23a0
            r12 = m.env.data_va(879469);
            // 41266 lea rsi,[rip+0xf7a5b] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 4126d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4126f mov rdi,r12
            rdi = r12;
            // 41272 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41277 lea r12,[rip+0x18113a] # 0x1c23b8
            r12 = m.env.data_va(879493);
            // 4127e lea rsi,[rip+0xf7a43] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41285 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41287 mov rdi,r12
            rdi = r12;
            // 4128a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4128f lea r15,[rip+0x18113a] # 0x1c23d0
            r15 = m.env.data_va(879517);
            // 41296 lea rsi,[rip+0xe996c] # 0x12ac09
            rsi = m.env.data_va(259030);
            // 4129d mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 412a2 mov rdi,r15
            rdi = r15;
            // 412a5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 412aa lea r12,[rip+0x181137] # 0x1c23e8
            r12 = m.env.data_va(879541);
            // 412b1 lea rsi,[rip+0xf7a10] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 412b8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 412ba mov rdi,r12
            rdi = r12;
            // 412bd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 412c2 lea r12,[rip+0x181137] # 0x1c2400
            r12 = m.env.data_va(879565);
            // 412c9 lea rsi,[rip+0xf79f8] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 412d0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 412d2 mov rdi,r12
            rdi = r12;
            // 412d5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 412da lea r12,[rip+0x181137] # 0x1c2418
            r12 = m.env.data_va(879589);
            // 412e1 lea rsi,[rip+0xf79e0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 412e8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 412ea mov rdi,r12
            rdi = r12;
            // 412ed call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 412f2 lea r15,[rip+0x181137] # 0x1c2430
            r15 = m.env.data_va(879613);
            // 412f9 lea rsi,[rip+0xe9922] # 0x12ac22
            rsi = m.env.data_va(259055);
            // 41300 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 41305 mov rdi,r15
            rdi = r15;
            // 41308 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4130d lea r12,[rip+0x181134] # 0x1c2448
            r12 = m.env.data_va(879637);
            // 41314 lea rsi,[rip+0xe9909] # 0x12ac24
            rsi = m.env.data_va(259057);
            // 4131b mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 41320 mov rdi,r12
            rdi = r12;
            // 41323 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41328 lea r12,[rip+0x181131] # 0x1c2460
            r12 = m.env.data_va(879661);
            // 4132f lea rsi,[rip+0xe98f1] # 0x12ac27
            rsi = m.env.data_va(259060);
            // 41336 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4133b mov rdi,r12
            rdi = r12;
            // 4133e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41343 lea r12,[rip+0x18112e] # 0x1c2478
            r12 = m.env.data_va(879685);
            // 4134a lea rsi,[rip+0xf7977] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41351 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41353 mov rdi,r12
            rdi = r12;
            // 41356 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4135b lea r15,[rip+0x18112e] # 0x1c2490
            r15 = m.env.data_va(879709);
            // 41362 lea rsi,[rip+0xe98b9] # 0x12ac22
            rsi = m.env.data_va(259055);
            // 41369 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4136e mov rdi,r15
            rdi = r15;
            // 41371 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41376 lea r12,[rip+0x18112b] # 0x1c24a8
            r12 = m.env.data_va(879733);
            // 4137d lea rsi,[rip+0xe98a0] # 0x12ac24
            rsi = m.env.data_va(259057);
            // 41384 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 41389 mov rdi,r12
            rdi = r12;
            // 4138c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41391 lea r12,[rip+0x181128] # 0x1c24c0
            r12 = m.env.data_va(879757);
            // 41398 lea rsi,[rip+0xe9888] # 0x12ac27
            rsi = m.env.data_va(259060);
            // 4139f mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 413a4 mov rdi,r12
            rdi = r12;
            // 413a7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 413ac lea r12,[rip+0x181125] # 0x1c24d8
            r12 = m.env.data_va(879781);
            // 413b3 lea rsi,[rip+0xf790e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 413ba xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 413bc mov rdi,r12
            rdi = r12;
            // 413bf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 413c4 lea r15,[rip+0x181125] # 0x1c24f0
            r15 = m.env.data_va(879805);
            // 413cb lea rsi,[rip+0xe9850] # 0x12ac22
            rsi = m.env.data_va(259055);
            // 413d2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 413d7 mov rdi,r15
            rdi = r15;
            // 413da call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 413df lea r12,[rip+0x181122] # 0x1c2508
            r12 = m.env.data_va(879829);
            // 413e6 lea rsi,[rip+0xe9848] # 0x12ac35
            rsi = m.env.data_va(259074);
            // 413ed mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 413f2 mov rdi,r12
            rdi = r12;
            // 413f5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 413fa lea r12,[rip+0x18111f] # 0x1c2520
            r12 = m.env.data_va(879853);
            // 41401 lea rsi,[rip+0xf78c0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41408 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4140a mov rdi,r12
            rdi = r12;
            // 4140d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41412 lea r12,[rip+0x18111f] # 0x1c2538
            r12 = m.env.data_va(879877);
            // 41419 lea rsi,[rip+0xf78a8] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41420 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41422 mov rdi,r12
            rdi = r12;
            // 41425 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4142a lea r15,[rip+0x18111f] # 0x1c2550
            r15 = m.env.data_va(879901);
            // 41431 lea rsi,[rip+0xe97ea] # 0x12ac22
            rsi = m.env.data_va(259055);
            // 41438 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4143d mov rdi,r15
            rdi = r15;
            // 41440 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41445 lea r12,[rip+0x18111c] # 0x1c2568
            r12 = m.env.data_va(879925);
            // 4144c lea rsi,[rip+0xe97e2] # 0x12ac35
            rsi = m.env.data_va(259074);
            // 41453 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 41458 mov rdi,r12
            rdi = r12;
            // 4145b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41460 lea r12,[rip+0x181119] # 0x1c2580
            r12 = m.env.data_va(879949);
            // 41467 lea rsi,[rip+0xf785a] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 4146e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41470 mov rdi,r12
            rdi = r12;
            // 41473 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41478 lea r12,[rip+0x181119] # 0x1c2598
            r12 = m.env.data_va(879973);
            // 4147f lea rsi,[rip+0xf7842] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41486 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41488 mov rdi,r12
            rdi = r12;
            // 4148b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41490 lea r15,[rip+0x181119] # 0x1c25b0
            r15 = m.env.data_va(879997);
            // 41497 lea rsi,[rip+0xe9784] # 0x12ac22
            rsi = m.env.data_va(259055);
            // 4149e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 414a3 mov rdi,r15
            rdi = r15;
            // 414a6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 414ab lea r12,[rip+0x181116] # 0x1c25c8
            r12 = m.env.data_va(880021);
            // 414b2 lea rsi,[rip+0xe977c] # 0x12ac35
            rsi = m.env.data_va(259074);
            // 414b9 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 414be mov rdi,r12
            rdi = r12;
            // 414c1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 414c6 lea r12,[rip+0x181113] # 0x1c25e0
            r12 = m.env.data_va(880045);
            // 414cd lea rsi,[rip+0xf77f4] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 414d4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 414d6 mov rdi,r12
            rdi = r12;
            // 414d9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 414de lea r12,[rip+0x181113] # 0x1c25f8
            r12 = m.env.data_va(880069);
            // 414e5 lea rsi,[rip+0xf77dc] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 414ec xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 414ee mov rdi,r12
            rdi = r12;
            // 414f1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 414f6 lea r15,[rip+0x181113] # 0x1c2610
            r15 = m.env.data_va(880093);
            // 414fd lea rsi,[rip+0xe971e] # 0x12ac22
            rsi = m.env.data_va(259055);
            // 41504 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 41509 mov rdi,r15
            rdi = r15;
            // 4150c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41511 lea r12,[rip+0x181110] # 0x1c2628
            r12 = m.env.data_va(880117);
            // 41518 lea rsi,[rip+0xe9716] # 0x12ac35
            rsi = m.env.data_va(259074);
            // 4151f mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 41524 mov rdi,r12
            rdi = r12;
            // 41527 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4152c lea r12,[rip+0x18110d] # 0x1c2640
            r12 = m.env.data_va(880141);
            // 41533 lea rsi,[rip+0xf778e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 4153a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4153c mov rdi,r12
            rdi = r12;
            // 4153f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41544 lea r12,[rip+0x18110d] # 0x1c2658
            r12 = m.env.data_va(880165);
            // 4154b lea rsi,[rip+0xf7776] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 41552 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41554 mov rdi,r12
            rdi = r12;
            // 41557 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4155c lea rdi,[rip+0xffffffffffff290a] # 0x33e6d
            rdi = m.env.code_va(0x33e6d);
            // 41563 lea rdx,[rip+0xfffffffffffbdc06] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // 4156a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4156c call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41571 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 41575 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 41576 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 41578 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4157a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4157c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4157e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4157f jmp 0x33e98
            { let r = { let t = m.env.code_va(0x33e98); m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
        0x41584 => {
            // block 41584
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41584 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41584 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41588 lea r15,[rip+0x1804e1] # 0x1c1a70
            r15 = m.env.data_va(877117);
            pc = 0x4158f;
        }
        0x4158f => {
            // block 4158f
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4158f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4158f add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41593 mov rdi,r12
            rdi = r12;
            // 41596 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4159b cmp r15,r12
            fl = sub::<64>(r15, r12).1;
            // 4159e jne 0x4158f
            if fl.ne() { pc = 0x4158f; continue; }
            pc = 0x415a0;
        }
        0x415a0 => {
            // block 415a0
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 415a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 415a0 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x415a5 => {
            // block 415a5
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 415a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 415a5 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 415a9 add r12,0xffffffffffffffa0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffa0u64);
              r12 = r;
              fl = f; }
            // 415ad lea r13,[rip+0x1804bc] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x415b4;
        }
        0x415b4 => {
            // block 415b4
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 415b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 415b4 lea rdi,[r12+0x48]
            rdi = r12.wrapping_add(0x48);
            // 415b9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 415be add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 415c2 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 415c5 jne 0x415b4
            if fl.ne() { pc = 0x415b4; continue; }
            pc = 0x415c7;
        }
        0x415c7 => {
            // block 415c7
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 415c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 415c7 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x415cc => {
            // block 415cc
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 415cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 415cc mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 415d0 add r12,0xffffffffffffff40
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ff40u64);
              r12 = r;
              fl = f; }
            // 415d7 lea r13,[rip+0x180492] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x415de;
        }
        0x415de => {
            // block 415de
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 415de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 415de lea rdi,[r12+0xa8]
            rdi = r12.wrapping_add(0xa8);
            // 415e6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 415eb add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 415ef cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 415f2 jne 0x415de
            if fl.ne() { pc = 0x415de; continue; }
            pc = 0x415f4;
        }
        0x415f4 => {
            // block 415f4
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 415f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 415f4 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x415f9 => {
            // block 415f9
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 415f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 415f9 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 415fd add r12,0xfffffffffffffee0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fee0u64);
              r12 = r;
              fl = f; }
            // 41604 lea r13,[rip+0x180465] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x4160b;
        }
        0x4160b => {
            // block 4160b
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4160b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4160b lea rdi,[r12+0x108]
            rdi = r12.wrapping_add(0x108);
            // 41613 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41618 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 4161c cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 4161f jne 0x4160b
            if fl.ne() { pc = 0x4160b; continue; }
            pc = 0x41621;
        }
        0x41621 => {
            // block 41621
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41621 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41621 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41626 => {
            // block 41626
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41626 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41626 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 4162a add r12,0xfffffffffffffe80
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fe80u64);
              r12 = r;
              fl = f; }
            // 41631 lea r13,[rip+0x180438] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41638;
        }
        0x41638 => {
            // block 41638
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41638 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41638 lea rdi,[r12+0x168]
            rdi = r12.wrapping_add(0x168);
            // 41640 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41645 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41649 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 4164c jne 0x41638
            if fl.ne() { pc = 0x41638; continue; }
            pc = 0x4164e;
        }
        0x4164e => {
            // block 4164e
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4164e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4164e jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41653 => {
            // block 41653
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41653 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41653 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41657 add r12,0xfffffffffffffe20
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fe20u64);
              r12 = r;
              fl = f; }
            // 4165e lea r13,[rip+0x18040b] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41665;
        }
        0x41665 => {
            // block 41665
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41665 lea rdi,[r12+0x1c8]
            rdi = r12.wrapping_add(0x1c8);
            // 4166d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41672 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41676 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41679 jne 0x41665
            if fl.ne() { pc = 0x41665; continue; }
            pc = 0x4167b;
        }
        0x4167b => {
            // block 4167b
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4167b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4167b jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41680 => {
            // block 41680
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41680 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41680 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41684 add r12,0xfffffffffffffdc0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fdc0u64);
              r12 = r;
              fl = f; }
            // 4168b lea r13,[rip+0x1803de] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41692;
        }
        0x41692 => {
            // block 41692
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41692 lea rdi,[r12+0x228]
            rdi = r12.wrapping_add(0x228);
            // 4169a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4169f add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 416a3 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 416a6 jne 0x41692
            if fl.ne() { pc = 0x41692; continue; }
            pc = 0x416a8;
        }
        0x416a8 => {
            // block 416a8
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 416a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 416a8 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x416ad => {
            // block 416ad
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 416ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 416ad mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 416b1 add r12,0xfffffffffffffd60
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fd60u64);
              r12 = r;
              fl = f; }
            // 416b8 lea r13,[rip+0x1803b1] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x416bf;
        }
        0x416bf => {
            // block 416bf
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 416bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 416bf lea rdi,[r12+0x288]
            rdi = r12.wrapping_add(0x288);
            // 416c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 416cc add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 416d0 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 416d3 jne 0x416bf
            if fl.ne() { pc = 0x416bf; continue; }
            pc = 0x416d5;
        }
        0x416d5 => {
            // block 416d5
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 416d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 416d5 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x416da => {
            // block 416da
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 416da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 416da mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 416de add r12,0xfffffffffffffd00
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fd00u64);
              r12 = r;
              fl = f; }
            // 416e5 lea r13,[rip+0x180384] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x416ec;
        }
        0x416ec => {
            // block 416ec
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 416ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 416ec lea rdi,[r12+0x2e8]
            rdi = r12.wrapping_add(0x2e8);
            // 416f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 416f9 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 416fd cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41700 jne 0x416ec
            if fl.ne() { pc = 0x416ec; continue; }
            pc = 0x41702;
        }
        0x41702 => {
            // block 41702
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41702 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41702 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41707 => {
            // block 41707
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41707 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41707 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 4170b add r12,0xfffffffffffffca0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fca0u64);
              r12 = r;
              fl = f; }
            // 41712 lea r13,[rip+0x180357] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41719;
        }
        0x41719 => {
            // block 41719
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41719 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41719 lea rdi,[r12+0x348]
            rdi = r12.wrapping_add(0x348);
            // 41721 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41726 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 4172a cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 4172d jne 0x41719
            if fl.ne() { pc = 0x41719; continue; }
            pc = 0x4172f;
        }
        0x4172f => {
            // block 4172f
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4172f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4172f jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41734 => {
            // block 41734
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41734 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41734 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41738 add r12,0xfffffffffffffc40
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fc40u64);
              r12 = r;
              fl = f; }
            // 4173f lea r13,[rip+0x18032a] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41746;
        }
        0x41746 => {
            // block 41746
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41746 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41746 lea rdi,[r12+0x3a8]
            rdi = r12.wrapping_add(0x3a8);
            // 4174e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41753 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41757 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 4175a jne 0x41746
            if fl.ne() { pc = 0x41746; continue; }
            pc = 0x4175c;
        }
        0x4175c => {
            // block 4175c
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4175c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4175c jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41761 => {
            // block 41761
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41761 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41761 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41765 add r12,0xfffffffffffffbe0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fbe0u64);
              r12 = r;
              fl = f; }
            // 4176c lea r13,[rip+0x1802fd] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41773;
        }
        0x41773 => {
            // block 41773
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41773 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41773 lea rdi,[r12+0x408]
            rdi = r12.wrapping_add(0x408);
            // 4177b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41780 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41784 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41787 jne 0x41773
            if fl.ne() { pc = 0x41773; continue; }
            pc = 0x41789;
        }
        0x41789 => {
            // block 41789
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41789 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41789 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x4178e => {
            // block 4178e
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4178e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4178e mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41792 add r12,0xfffffffffffffb80
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fb80u64);
              r12 = r;
              fl = f; }
            // 41799 lea r13,[rip+0x1802d0] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x417a0;
        }
        0x417a0 => {
            // block 417a0
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 417a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 417a0 lea rdi,[r12+0x468]
            rdi = r12.wrapping_add(0x468);
            // 417a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 417ad add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 417b1 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 417b4 jne 0x417a0
            if fl.ne() { pc = 0x417a0; continue; }
            pc = 0x417b6;
        }
        0x417b6 => {
            // block 417b6
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 417b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 417b6 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x417bb => {
            // block 417bb
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 417bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 417bb mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 417bf add r12,0xfffffffffffffb20
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fb20u64);
              r12 = r;
              fl = f; }
            // 417c6 lea r13,[rip+0x1802a3] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x417cd;
        }
        0x417cd => {
            // block 417cd
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 417cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 417cd lea rdi,[r12+0x4c8]
            rdi = r12.wrapping_add(0x4c8);
            // 417d5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 417da add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 417de cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 417e1 jne 0x417cd
            if fl.ne() { pc = 0x417cd; continue; }
            pc = 0x417e3;
        }
        0x417e3 => {
            // block 417e3
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 417e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 417e3 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x417e8 => {
            // block 417e8
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 417e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 417e8 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 417ec add r12,0xfffffffffffffac0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fac0u64);
              r12 = r;
              fl = f; }
            // 417f3 lea r13,[rip+0x180276] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x417fa;
        }
        0x417fa => {
            // block 417fa
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 417fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 417fa lea rdi,[r12+0x528]
            rdi = r12.wrapping_add(0x528);
            // 41802 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41807 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 4180b cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 4180e jne 0x417fa
            if fl.ne() { pc = 0x417fa; continue; }
            pc = 0x41810;
        }
        0x41810 => {
            // block 41810
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41810 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41810 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41815 => {
            // block 41815
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41815 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41815 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41819 add r12,0xfffffffffffffa60
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fa60u64);
              r12 = r;
              fl = f; }
            // 41820 lea r13,[rip+0x180249] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41827;
        }
        0x41827 => {
            // block 41827
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41827 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41827 lea rdi,[r12+0x588]
            rdi = r12.wrapping_add(0x588);
            // 4182f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41834 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41838 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 4183b jne 0x41827
            if fl.ne() { pc = 0x41827; continue; }
            pc = 0x4183d;
        }
        0x4183d => {
            // block 4183d
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4183d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4183d jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41842 => {
            // block 41842
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41842 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41842 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41846 add r12,0xfffffffffffffa00
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_fa00u64);
              r12 = r;
              fl = f; }
            // 4184d lea r13,[rip+0x18021c] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41854;
        }
        0x41854 => {
            // block 41854
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41854 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41854 lea rdi,[r12+0x5e8]
            rdi = r12.wrapping_add(0x5e8);
            // 4185c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41861 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41865 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41868 jne 0x41854
            if fl.ne() { pc = 0x41854; continue; }
            pc = 0x4186a;
        }
        0x4186a => {
            // block 4186a
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4186a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4186a jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x4186f => {
            // block 4186f
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4186f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4186f mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41873 add r12,0xfffffffffffff9a0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f9a0u64);
              r12 = r;
              fl = f; }
            // 4187a lea r13,[rip+0x1801ef] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41881;
        }
        0x41881 => {
            // block 41881
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41881 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41881 lea rdi,[r12+0x648]
            rdi = r12.wrapping_add(0x648);
            // 41889 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4188e add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41892 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41895 jne 0x41881
            if fl.ne() { pc = 0x41881; continue; }
            pc = 0x41897;
        }
        0x41897 => {
            // block 41897
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41897 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41897 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x4189c => {
            // block 4189c
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4189c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4189c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 418a0 add r12,0xfffffffffffff940
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f940u64);
              r12 = r;
              fl = f; }
            // 418a7 lea r13,[rip+0x1801c2] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x418ae;
        }
        0x418ae => {
            // block 418ae
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 418ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 418ae lea rdi,[r12+0x6a8]
            rdi = r12.wrapping_add(0x6a8);
            // 418b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 418bb add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 418bf cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 418c2 jne 0x418ae
            if fl.ne() { pc = 0x418ae; continue; }
            pc = 0x418c4;
        }
        0x418c4 => {
            // block 418c4
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 418c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 418c4 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x418c9 => {
            // block 418c9
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 418c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 418c9 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 418cd add r12,0xfffffffffffff8e0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f8e0u64);
              r12 = r;
              fl = f; }
            // 418d4 lea r13,[rip+0x180195] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x418db;
        }
        0x418db => {
            // block 418db
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 418db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 418db lea rdi,[r12+0x708]
            rdi = r12.wrapping_add(0x708);
            // 418e3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 418e8 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 418ec cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 418ef jne 0x418db
            if fl.ne() { pc = 0x418db; continue; }
            pc = 0x418f1;
        }
        0x418f1 => {
            // block 418f1
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 418f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 418f1 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x418f6 => {
            // block 418f6
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 418f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 418f6 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 418fa add r12,0xfffffffffffff880
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f880u64);
              r12 = r;
              fl = f; }
            // 41901 lea r13,[rip+0x180168] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41908;
        }
        0x41908 => {
            // block 41908
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41908 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41908 lea rdi,[r12+0x768]
            rdi = r12.wrapping_add(0x768);
            // 41910 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41915 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41919 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 4191c jne 0x41908
            if fl.ne() { pc = 0x41908; continue; }
            pc = 0x4191e;
        }
        0x4191e => {
            // block 4191e
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4191e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4191e jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41923 => {
            // block 41923
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41923 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41923 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41927 add r12,0xfffffffffffff820
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f820u64);
              r12 = r;
              fl = f; }
            // 4192e lea r13,[rip+0x18013b] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41935;
        }
        0x41935 => {
            // block 41935
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41935 lea rdi,[r12+0x7c8]
            rdi = r12.wrapping_add(0x7c8);
            // 4193d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41942 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41946 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41949 jne 0x41935
            if fl.ne() { pc = 0x41935; continue; }
            pc = 0x4194b;
        }
        0x4194b => {
            // block 4194b
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4194b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4194b jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41950 => {
            // block 41950
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41950 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41950 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41954 add r12,0xfffffffffffff7c0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f7c0u64);
              r12 = r;
              fl = f; }
            // 4195b lea r13,[rip+0x18010e] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41962;
        }
        0x41962 => {
            // block 41962
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41962 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41962 lea rdi,[r12+0x828]
            rdi = r12.wrapping_add(0x828);
            // 4196a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4196f add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41973 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41976 jne 0x41962
            if fl.ne() { pc = 0x41962; continue; }
            pc = 0x41978;
        }
        0x41978 => {
            // block 41978
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41978 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41978 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x4197d => {
            // block 4197d
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4197d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4197d mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41981 add r12,0xfffffffffffff760
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f760u64);
              r12 = r;
              fl = f; }
            // 41988 lea r13,[rip+0x1800e1] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x4198f;
        }
        0x4198f => {
            // block 4198f
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 4198f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4198f lea rdi,[r12+0x888]
            rdi = r12.wrapping_add(0x888);
            // 41997 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4199c add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 419a0 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 419a3 jne 0x4198f
            if fl.ne() { pc = 0x4198f; continue; }
            pc = 0x419a5;
        }
        0x419a5 => {
            // block 419a5
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 419a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 419a5 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x419aa => {
            // block 419aa
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 419aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 419aa mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 419ae add r12,0xfffffffffffff700
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f700u64);
              r12 = r;
              fl = f; }
            // 419b5 lea r13,[rip+0x1800b4] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x419bc;
        }
        0x419bc => {
            // block 419bc
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 419bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 419bc lea rdi,[r12+0x8e8]
            rdi = r12.wrapping_add(0x8e8);
            // 419c4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 419c9 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 419cd cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 419d0 jne 0x419bc
            if fl.ne() { pc = 0x419bc; continue; }
            pc = 0x419d2;
        }
        0x419d2 => {
            // block 419d2
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 419d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 419d2 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x419d7 => {
            // block 419d7
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 419d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 419d7 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 419db add r12,0xfffffffffffff6a0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f6a0u64);
              r12 = r;
              fl = f; }
            // 419e2 lea r13,[rip+0x180087] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x419e9;
        }
        0x419e9 => {
            // block 419e9
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 419e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 419e9 lea rdi,[r12+0x948]
            rdi = r12.wrapping_add(0x948);
            // 419f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 419f6 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 419fa cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 419fd jne 0x419e9
            if fl.ne() { pc = 0x419e9; continue; }
            pc = 0x419ff;
        }
        0x419ff => {
            // block 419ff
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 419ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 419ff jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41a04 => {
            // block 41a04
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a04 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41a08 add r12,0xfffffffffffff640
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f640u64);
              r12 = r;
              fl = f; }
            // 41a0f lea r13,[rip+0x18005a] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41a16;
        }
        0x41a16 => {
            // block 41a16
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a16 lea rdi,[r12+0x9a8]
            rdi = r12.wrapping_add(0x9a8);
            // 41a1e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41a23 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41a27 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41a2a jne 0x41a16
            if fl.ne() { pc = 0x41a16; continue; }
            pc = 0x41a2c;
        }
        0x41a2c => {
            // block 41a2c
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a2c jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41a31 => {
            // block 41a31
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a31 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41a35 add r12,0xfffffffffffff5e0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f5e0u64);
              r12 = r;
              fl = f; }
            // 41a3c lea r13,[rip+0x18002d] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41a43;
        }
        0x41a43 => {
            // block 41a43
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a43 lea rdi,[r12+0xa08]
            rdi = r12.wrapping_add(0xa08);
            // 41a4b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41a50 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41a54 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41a57 jne 0x41a43
            if fl.ne() { pc = 0x41a43; continue; }
            pc = 0x41a59;
        }
        0x41a59 => {
            // block 41a59
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a59 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41a5e => {
            // block 41a5e
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a5e mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41a62 add r12,0xfffffffffffff580
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f580u64);
              r12 = r;
              fl = f; }
            // 41a69 lea r13,[rip+0x180000] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41a70;
        }
        0x41a70 => {
            // block 41a70
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a70 lea rdi,[r12+0xa68]
            rdi = r12.wrapping_add(0xa68);
            // 41a78 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41a7d add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41a81 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41a84 jne 0x41a70
            if fl.ne() { pc = 0x41a70; continue; }
            pc = 0x41a86;
        }
        0x41a86 => {
            // block 41a86
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a86 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41a8b => {
            // block 41a8b
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a8b mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41a8f add r12,0xfffffffffffff520
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f520u64);
              r12 = r;
              fl = f; }
            // 41a96 lea r13,[rip+0x17ffd3] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41a9d;
        }
        0x41a9d => {
            // block 41a9d
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41a9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41a9d lea rdi,[r12+0xac8]
            rdi = r12.wrapping_add(0xac8);
            // 41aa5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41aaa add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41aae cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41ab1 jne 0x41a9d
            if fl.ne() { pc = 0x41a9d; continue; }
            pc = 0x41ab3;
        }
        0x41ab3 => {
            // block 41ab3
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41ab3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41ab3 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41ab8 => {
            // block 41ab8
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41ab8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41ab8 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41abc add r12,0xfffffffffffff4c0
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f4c0u64);
              r12 = r;
              fl = f; }
            // 41ac3 lea r13,[rip+0x17ffa6] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41aca;
        }
        0x41aca => {
            // block 41aca
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41aca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41aca lea rdi,[r12+0xb28]
            rdi = r12.wrapping_add(0xb28);
            // 41ad2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41ad7 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41adb cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41ade jne 0x41aca
            if fl.ne() { pc = 0x41aca; continue; }
            pc = 0x41ae0;
        }
        0x41ae0 => {
            // block 41ae0
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41ae0 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41ae5 => {
            // block 41ae5
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41ae5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41ae5 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41ae9 add r12,0xfffffffffffff460
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_f460u64);
              r12 = r;
              fl = f; }
            // 41af0 lea r13,[rip+0x17ff79] # 0x1c1a70
            r13 = m.env.data_va(877117);
            pc = 0x41af7;
        }
        0x41af7 => {
            // block 41af7
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41af7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41af7 lea rdi,[r12+0xb88]
            rdi = r12.wrapping_add(0xb88);
            // 41aff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41b04 add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // 41b08 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 41b0b jne 0x41af7
            if fl.ne() { pc = 0x41af7; continue; }
            pc = 0x41b0d;
        }
        0x41b0d => {
            // block 41b0d
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b0d jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b12 => {
            // block 41b12
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b12 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b16 mov r15,r14
            r15 = r14;
            // 41b19 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b1e => {
            // block 41b1e
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b1e mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b22 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b27 => {
            // block 41b27
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b27 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b2b jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b30 => {
            // block 41b30
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b30 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b34 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b39 => {
            // block 41b39
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b39 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b3d jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b42 => {
            // block 41b42
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b42 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b46 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b4b => {
            // block 41b4b
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b4b mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b4f jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b54 => {
            // block 41b54
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b54 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b58 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b5d => {
            // block 41b5d
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b5d mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b61 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b66 => {
            // block 41b66
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b66 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b6a jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b6f => {
            // block 41b6f
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b6f mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b73 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b75 => {
            // block 41b75
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b75 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b79 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b7b => {
            // block 41b7b
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b7b mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b7f jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b81 => {
            // block 41b81
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b81 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b85 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b87 => {
            // block 41b87
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b87 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b8b jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b8d => {
            // block 41b8d
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b8d mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b91 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b93 => {
            // block 41b93
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b93 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b97 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b99 => {
            // block 41b99
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b99 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41b9d jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41b9f => {
            // block 41b9f
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41b9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41b9f mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41ba3 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41ba5 => {
            // block 41ba5
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41ba5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41ba5 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41ba9 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bab => {
            // block 41bab
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bab mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41baf jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bb1 => {
            // block 41bb1
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bb1 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41bb5 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bb7 => {
            // block 41bb7
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bb7 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41bbb jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bbd => {
            // block 41bbd
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bbd mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41bc1 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bc3 => {
            // block 41bc3
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bc3 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41bc7 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bc9 => {
            // block 41bc9
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bc9 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41bcd jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bcf => {
            // block 41bcf
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bcf mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41bd3 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bd5 => {
            // block 41bd5
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bd5 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41bd9 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bdb => {
            // block 41bdb
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bdb mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41bdf jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41be1 => {
            // block 41be1
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41be1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41be1 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41be5 jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41be7 => {
            // block 41be7
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41be7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41be7 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41beb jmp 0x41bf1
            pc = 0x41bf1; continue;
        }
        0x41bed => {
            // block 41bed
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bed mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            pc = 0x41bf1;
        }
        0x41bf1 => {
            // block 41bf1
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bf1 cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // 41bf4 je 0x41c0f
            if fl.e() { pc = 0x41c0f; continue; }
            pc = 0x41bf6;
        }
        0x41bf6 => {
            // block 41bf6
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bf6 lea rbx,[rip+0x17fe73] # 0x1c1a70
            rbx = m.env.data_va(877117);
            pc = 0x41bfd;
        }
        0x41bfd => {
            // block 41bfd
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41bfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41bfd lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 41c01 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41c06 add r15,0xffffffffffffffa0
            { let (r, f) = add::<64>(r15, 0xffff_ffff_ffff_ffa0u64);
              r15 = r;
              fl = f; }
            // 41c0a cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 41c0d jne 0x41bfd
            if fl.ne() { pc = 0x41bfd; continue; }
            pc = 0x41c0f;
        }
        0x41c0f => {
            // block 41c0f
            #[cfg(feature = "trace")] eprintln!("f_000408df pc 41c0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41c0f mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 41c13 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x99644 size=5262
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00099644<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x99644;
    loop {
        match pc {
        0x99644 => {
            // block 99644
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99644 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 99645 mov rbp,rsp
            rbp = rsp;
            // 99648 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9964a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9964c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 9964e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 99650 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 99651 sub rsp,0x7f8
            { let (r, f) = sub::<64>(rsp, 0x7f8u64);
              rsp = r;
              fl = f; }
            // 99658 mov r14,rdi
            r14 = rdi;
            // 9965b mov al,BYTE PTR [rip+0xafd4f] # 0x1493b0
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(383869)) as u64) as u8 as u64);
            // 99661 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 99663 jne 0x99740
            if fl.ne() { pc = 0x99740; continue; }
            pc = 0x99669;
        }
        0x99669 => {
            // block 99669
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99669 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99669 lea rdi,[rip+0xafd40] # 0x1493b0
            rdi = m.env.data_va(383869);
            // 99670 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99675 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 99677 je 0x99740
            if fl.e() { pc = 0x99740; continue; }
            pc = 0x9967d;
        }
        0x9967d => {
            // block 9967d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9967d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9967d lea r15,[rip+0xafc9c] # 0x149320
            r15 = m.env.data_va(383725);
            // 99684 lea rsi,[rip+0x9c182] # 0x13580d
            rsi = m.env.data_va(303066);
            // 9968b mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 99690 mov rdi,r15
            rdi = r15;
            // 99693 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99698 lea rbx,[rip+0xafc99] # 0x149338
            rbx = m.env.data_va(383749);
            // 9969f lea rsi,[rip+0x9c16e] # 0x135814
            rsi = m.env.data_va(303073);
            // 996a6 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 996ab mov rdi,rbx
            rdi = rbx;
            // 996ae call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 996b3 lea rbx,[rip+0xafc96] # 0x149350
            rbx = m.env.data_va(383773);
            // 996ba lea rsi,[rip+0x9c161] # 0x135822
            rsi = m.env.data_va(303087);
            // 996c1 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 996c6 mov rdi,rbx
            rdi = rbx;
            // 996c9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 996ce lea rbx,[rip+0xafc93] # 0x149368
            rbx = m.env.data_va(383797);
            // 996d5 lea rsi,[rip+0x9c154] # 0x135830
            rsi = m.env.data_va(303101);
            // 996dc mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 996e1 mov rdi,rbx
            rdi = rbx;
            // 996e4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 996e9 lea rbx,[rip+0xafc90] # 0x149380
            rbx = m.env.data_va(383821);
            // 996f0 lea rsi,[rip+0x9c148] # 0x13583f
            rsi = m.env.data_va(303116);
            // 996f7 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 996fc mov rdi,rbx
            rdi = rbx;
            // 996ff call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99704 lea rbx,[rip+0xafc8d] # 0x149398
            rbx = m.env.data_va(383845);
            // 9970b lea rsi,[rip+0x9c13e] # 0x135850
            rsi = m.env.data_va(303133);
            // 99712 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 99717 mov rdi,rbx
            rdi = rbx;
            // 9971a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9971f lea rdi,[rip+0xfffffffffffcf16d] # 0x68893
            rdi = m.env.code_va(0x68893);
            // 99726 lea rdx,[rip+0xfffffffffff65a43] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // 9972d xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 9972f call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99734 lea rdi,[rip+0xafc75] # 0x1493b0
            rdi = m.env.data_va(383869);
            // 9973b call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99740;
        }
        0x99740 => {
            // block 99740
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99740 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99740 lea rbx,[rbp-0x138]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 99747 lea r13,[rbp-0x1a0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 9974e lea rax,[r14+0x180]
            rax = r14.wrapping_add(0x180);
            // 99755 mov QWORD PTR [rbp-0x788],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f878), (rax) as u64);
            // 9975c lea rax,[r14+0x198]
            rax = r14.wrapping_add(0x198);
            // 99763 mov QWORD PTR [rbp-0x748],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8), (rax) as u64);
            // 9976a lea rax,[r14+0x158]
            rax = r14.wrapping_add(0x158);
            // 99771 mov QWORD PTR [rbp-0x7a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f858), (rax) as u64);
            // 99778 lea rax,[r14+0x160]
            rax = r14.wrapping_add(0x160);
            // 9977f mov QWORD PTR [rbp-0x7a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f860), (rax) as u64);
            // 99786 mov QWORD PTR [rbp-0x770],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f890), (r14) as u64);
            // 9978d xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 9978f mov rax,QWORD PTR [rip+0xa6c12] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 99796 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 9979a mov QWORD PTR [rbp-0x750],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0), (rax) as u64);
            // 997a1 mov rax,QWORD PTR [rip+0xa6a60] # 0x140208
            rax = m.r64(m.env.data_va(346581)) as u64;
            // 997a8 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 997ac mov QWORD PTR [rbp-0x790],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f870), (rax) as u64);
            // 997b3 lea r14,[rbp-0xa0]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            pc = 0x997ba;
        }
        0x997ba => {
            // block 997ba
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 997ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 997ba mov QWORD PTR [rbp-0x780],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f880), (rcx) as u64);
            // 997c1 lea rax,[rcx+0x1]
            rax = rcx.wrapping_add(0x1);
            // 997c5 mov QWORD PTR [rbp-0x778],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f888), (rax) as u64);
            // 997cc lea rax,[rcx+rcx*2]
            rax = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 997d0 lea rcx,[rip+0x12eae9] # 0x1c82c0
            rcx = m.env.data_va(903821);
            // 997d7 lea rdx,[rcx+rax*8]
            rdx = rcx.wrapping_add(rax.wrapping_mul(8));
            // 997db mov QWORD PTR [rbp-0x768],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f898), (rdx) as u64);
            // 997e2 lea rax,[rcx+rax*8+0x8]
            rax = rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8);
            // 997e7 mov QWORD PTR [rbp-0x798],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f868), (rax) as u64);
            // 997ee lea r12,[rip+0xafb2b] # 0x149320
            r12 = m.env.data_va(383725);
            // 997f5 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x997f8;
        }
        0x997f8 => {
            // block 997f8
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 997f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 997f8 mov rax,QWORD PTR [rip+0xa69d1] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 997ff mov rcx,rax
            rcx = rax;
            // 99802 lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 99806 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 9980d lea rax,[rcx+0x40]
            rax = rcx.wrapping_add(0x40);
            // 99811 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 99818 mov rdi,rbx
            rdi = rbx;
            // 9981b mov rsi,r13
            rsi = r13;
            // 9981e call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99823 mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // 9982e mov DWORD PTR [rbp-0xa8],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0xffffffffu64) as u32);
            // 99838 mov rax,QWORD PTR [rip+0xa6b81] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 9983f lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 99843 mov QWORD PTR [rbp-0x760],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0), (rcx) as u64);
            // 9984a mov QWORD PTR [rbp-0x1a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rcx) as u64);
            // 99851 lea rax,[rax+0x40]
            rax = rax.wrapping_add(0x40);
            // 99855 mov QWORD PTR [rbp-0x758],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8a8), (rax) as u64);
            // 9985c mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 99863 mov rdi,r13
            rdi = r13;
            // 99866 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9986b mov rax,QWORD PTR [rbp-0x750]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0)) as u64;
            // 99872 mov QWORD PTR [rbp-0x1a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rax) as u64);
            // 99879 lea rax,[rbp-0x160]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 99880 mov QWORD PTR [rax+0x18],0x0
            m.w64(rax.wrapping_add(0x18), (0x0u64) as u64);
            // 99888 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 99890 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 99898 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 9989f mov DWORD PTR [rbp-0x140],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x10u64) as u32);
            // 998a9 xorps xmm0,xmm0
            x0 = 0;
            // 998ac movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // 998b3 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 998be mov rdi,r13
            rdi = r13;
            // 998c1 mov rsi,r14
            rsi = r14;
            // 998c4 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 998c9 mov rdi,r14
            rdi = r14;
            // 998cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 998d1 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 998d6 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 998dd lea rsi,[rip+0x9bf7e] # 0x135862
            rsi = m.env.data_va(303151);
            // 998e4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 998e9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 998ec mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 998f0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 998f4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 998f7 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 998fa mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 998fe mov rdi,rax
            rdi = rax;
            // 99901 mov rsi,QWORD PTR [rbp-0x778]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f888)) as u64;
            // 99908 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9990d mov rdi,rax
            rdi = rax;
            // 99910 mov rsi,r12
            rsi = r12;
            // 99913 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99918 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9991f mov rsi,r13
            rsi = r13;
            // 99922 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99927 mov rax,QWORD PTR [rbp-0x780]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f880)) as u64;
            // 9992e lea rcx,[rip+0x68ddb] # 0x102710
            rcx = m.env.data_va(93917);
            // 99935 mov r13d,DWORD PTR [rcx+rax*4]
            r13 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 99939 add r13,r15
            { let (r, f) = add::<64>(r13, r15);
              r13 = r;
              fl = f; }
            // 9993c movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // 99940 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 99943 jne 0x999e8
            if fl.ne() { pc = 0x999e8; continue; }
            pc = 0x99949;
        }
        0x99949 => {
            // block 99949
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99949 mov rdi,QWORD PTR [rbp-0x770]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 99950 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 99953 lea rdx,[rbp-0x1c0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9995a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9995f mov DWORD PTR [rbp-0x2f0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd10), ((r13 as u32 as u64)) as u32);
            // 99966 mov rax,QWORD PTR [rbp-0x7a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f860)) as u64;
            // 9996d mov QWORD PTR [rbp-0x2e8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd18), (rax) as u64);
            // 99974 mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 9997b lea rsi,[rbp-0x2f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd10);
            // 99982 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99987 mov rax,r15
            rax = r15;
            // 9998a and rax,0xfffe
            { let r = rax & 0xfffeu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 99990 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // 99994 jne 0x99bda
            if fl.ne() { pc = 0x99bda; continue; }
            pc = 0x9999a;
        }
        0x9999a => {
            // block 9999a
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9999a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9999a mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9999f lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 999a3 lea rsi,[rip+0x9b9b7] # 0x135361
            rsi = m.env.data_va(301870);
            // 999aa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 999af lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 999b3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 999b8 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 999bd lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // 999c4 lea rsi,[rip+0x9a01f] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 999cb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 999d0 movzx ebx,BYTE PTR [rbp-0x308]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fcf8)) as u64) as u32 as u64;
            // 999d7 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 999da jne 0x99b36
            if fl.ne() { pc = 0x99b36; continue; }
            pc = 0x999e0;
        }
        0x999e0 => {
            // block 999e0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 999e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 999e0 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 999e3 jmp 0x99b3d
            pc = 0x99b3d; continue;
        }
        0x999e8 => {
            // block 999e8
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 999e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 999e8 test r15w,r15w
            fl = logic::<16>((r15 as u16 as u64) & (r15 as u16 as u64));
            // 999ec jne 0x99a97
            if fl.ne() { pc = 0x99a97; continue; }
            pc = 0x999f2;
        }
        0x999f2 => {
            // block 999f2
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 999f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 999f2 mov rdi,QWORD PTR [rbp-0x770]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 999f9 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 999fc lea rdx,[rbp-0x1c0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 99a03 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a08 mov DWORD PTR [rbp-0x1d0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe30), ((r13 as u32 as u64)) as u32);
            // 99a0f mov rax,QWORD PTR [rbp-0x7a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f858)) as u64;
            // 99a16 mov QWORD PTR [rbp-0x1c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (rax) as u64);
            // 99a1d mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 99a24 lea rsi,[rbp-0x1d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe30);
            // 99a2b call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a30 mov rax,r15
            rax = r15;
            // 99a33 and rax,0xfffe
            { let r = rax & 0xfffeu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 99a39 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // 99a3d jne 0x99e6e
            if fl.ne() { pc = 0x99e6e; continue; }
            pc = 0x99a43;
        }
        0x99a43 => {
            // block 99a43
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99a43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99a43 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 99a48 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 99a4f lea rsi,[rip+0x9b90b] # 0x135361
            rsi = m.env.data_va(301870);
            // 99a56 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a5b lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 99a62 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a67 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99a6c lea rdi,[rbp-0x1e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe18);
            // 99a73 lea rsi,[rip+0x99f70] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 99a7a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a7f movzx ebx,BYTE PTR [rbp-0x1e8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe18)) as u64) as u32 as u64;
            // 99a86 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99a89 jne 0x99dca
            if fl.ne() { pc = 0x99dca; continue; }
            pc = 0x99a8f;
        }
        0x99a8f => {
            // block 99a8f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99a8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99a8f shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99a92 jmp 0x99dd1
            pc = 0x99dd1; continue;
        }
        0x99a97 => {
            // block 99a97
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99a97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99a97 mov rdi,QWORD PTR [rbp-0x770]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 99a9e mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 99aa1 lea rdx,[rbp-0x1c0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 99aa8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99aad mov DWORD PTR [rbp-0x410],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbf0), ((r13 as u32 as u64)) as u32);
            // 99ab4 mov rax,QWORD PTR [rbp-0x770]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 99abb mov QWORD PTR [rbp-0x408],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbf8), (rax) as u64);
            // 99ac2 mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 99ac9 lea rsi,[rbp-0x410]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbf0);
            // 99ad0 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99ad5 mov rax,r15
            rax = r15;
            // 99ad8 and rax,0xfffe
            { let r = rax & 0xfffeu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 99ade cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // 99ae2 jne 0x9a102
            if fl.ne() { pc = 0x9a102; continue; }
            pc = 0x99ae8;
        }
        0x99ae8 => {
            // block 99ae8
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99ae8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99ae8 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 99aed lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 99af1 lea rsi,[rip+0x9b869] # 0x135361
            rsi = m.env.data_va(301870);
            // 99af8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99afd lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 99b01 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99b06 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99b0b lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // 99b12 lea rsi,[rip+0x99ed1] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 99b19 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99b1e movzx ebx,BYTE PTR [rbp-0x428]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fbd8)) as u64) as u32 as u64;
            // 99b25 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99b28 jne 0x9a05e
            if fl.ne() { pc = 0x9a05e; continue; }
            pc = 0x99b2e;
        }
        0x99b2e => {
            // block 99b2e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99b2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99b2e shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99b31 jmp 0x9a065
            pc = 0x9a065; continue;
        }
        0x99b36 => {
            // block 99b36
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99b36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99b36 mov rbx,QWORD PTR [rbp-0x300]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64;
            pc = 0x99b3d;
        }
        0x99b3d => {
            // block 99b3d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99b3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99b3d lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // 99b44 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99b49 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99b4c je 0x99bda
            if fl.e() { pc = 0x99bda; continue; }
            pc = 0x99b52;
        }
        0x99b52 => {
            // block 99b52
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99b52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99b52 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99b57 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 99b5e lea rsi,[rip+0x99e85] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 99b65 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99b6a mov rax,QWORD PTR [rbp-0x330]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // 99b71 mov QWORD PTR [rbp-0x318],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rax) as u64);
            // 99b78 mov rax,QWORD PTR [rbp-0x340]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcc0)) as u64;
            // 99b7f mov rcx,QWORD PTR [rbp-0x338]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcc8)) as u64;
            // 99b86 mov QWORD PTR [rbp-0x320],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce0), (rcx) as u64);
            // 99b8d mov QWORD PTR [rbp-0x328],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8), (rax) as u64);
            // 99b94 xorps xmm0,xmm0
            x0 = 0;
            // 99b97 movaps XMMWORD PTR [rbp-0x340],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fcc0), x0);
            // 99b9e mov QWORD PTR [rbp-0x330],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0), (0x0u64) as u64);
            // 99ba9 mov DWORD PTR [rbp-0x310],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), ((r13 as u32 as u64)) as u32);
            // 99bb0 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99bb7 lea rbx,[rbp-0x328]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 99bbe mov rsi,rbx
            rsi = rbx;
            // 99bc1 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99bc6 mov rdi,rbx
            rdi = rbx;
            // 99bc9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99bce lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 99bd5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99bda;
        }
        0x99bda => {
            // block 99bda
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99bda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99bda mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99bdf lea rdi,[rbp-0x358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // 99be6 lea rsi,[rip+0x9bc7f] # 0x13586c
            rsi = m.env.data_va(303161);
            // 99bed call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99bf2 movzx ebx,BYTE PTR [rbp-0x358]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fca8)) as u64) as u32 as u64;
            // 99bf9 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99bfc jne 0x99c03
            if fl.ne() { pc = 0x99c03; continue; }
            pc = 0x99bfe;
        }
        0x99bfe => {
            // block 99bfe
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99bfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99bfe shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99c01 jmp 0x99c0a
            pc = 0x99c0a; continue;
        }
        0x99c03 => {
            // block 99c03
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99c03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99c03 mov rbx,QWORD PTR [rbp-0x350]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64;
            pc = 0x99c0a;
        }
        0x99c0a => {
            // block 99c0a
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99c0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99c0a lea rdi,[rbp-0x358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // 99c11 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99c16 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99c19 je 0x99ca7
            if fl.e() { pc = 0x99ca7; continue; }
            pc = 0x99c1f;
        }
        0x99c1f => {
            // block 99c1f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99c1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99c1f mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99c24 lea rdi,[rbp-0x390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc70);
            // 99c2b lea rsi,[rip+0x9bc3a] # 0x13586c
            rsi = m.env.data_va(303161);
            // 99c32 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99c37 mov rax,QWORD PTR [rbp-0x380]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc80)) as u64;
            // 99c3e mov QWORD PTR [rbp-0x368],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc98), (rax) as u64);
            // 99c45 mov rax,QWORD PTR [rbp-0x390]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc70)) as u64;
            // 99c4c mov rcx,QWORD PTR [rbp-0x388]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc78)) as u64;
            // 99c53 mov QWORD PTR [rbp-0x370],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc90), (rcx) as u64);
            // 99c5a mov QWORD PTR [rbp-0x378],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc88), (rax) as u64);
            // 99c61 xorps xmm0,xmm0
            x0 = 0;
            // 99c64 movaps XMMWORD PTR [rbp-0x390],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc70), x0);
            // 99c6b mov QWORD PTR [rbp-0x380],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc80), (0x0u64) as u64);
            // 99c76 mov DWORD PTR [rbp-0x360],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fca0), ((r13 as u32 as u64)) as u32);
            // 99c7d mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99c84 lea rbx,[rbp-0x378]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc88);
            // 99c8b mov rsi,rbx
            rsi = rbx;
            // 99c8e call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99c93 mov rdi,rbx
            rdi = rbx;
            // 99c96 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99c9b lea rdi,[rbp-0x390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc70);
            // 99ca2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99ca7;
        }
        0x99ca7 => {
            // block 99ca7
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99ca7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99ca7 mov rax,QWORD PTR [rbp-0x768]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 99cae movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // 99cb1 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 99cb3 jne 0x99cba
            if fl.ne() { pc = 0x99cba; continue; }
            pc = 0x99cb5;
        }
        0x99cb5 => {
            // block 99cb5
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cb5 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 99cb8 jmp 0x99cc4
            pc = 0x99cc4; continue;
        }
        0x99cba => {
            // block 99cba
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cba mov rax,QWORD PTR [rbp-0x798]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f868)) as u64;
            // 99cc1 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            pc = 0x99cc4;
        }
        0x99cc4 => {
            // block 99cc4
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cc4 lea rdi,[rbp-0x3b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc50);
            // 99ccb test rax,rax
            fl = logic::<64>(rax & rax);
            // 99cce je 0x99cfd
            if fl.e() { pc = 0x99cfd; continue; }
            pc = 0x99cd0;
        }
        0x99cd0 => {
            // block 99cd0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cd0 mov rbx,rdi
            rbx = rdi;
            // 99cd3 mov rsi,QWORD PTR [rbp-0x768]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 99cda call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99cdf mov DWORD PTR [rbp-0x398],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc68), ((r13 as u32 as u64)) as u32);
            // 99ce6 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99ced mov rsi,rbx
            rsi = rbx;
            // 99cf0 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99cf5 mov rdi,rbx
            rdi = rbx;
            // 99cf8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99cfd;
        }
        0x99cfd => {
            // block 99cfd
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cfd mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 99d02 lea rdi,[rbp-0x3c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // 99d09 lea rsi,[rip+0x99cfb] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 99d10 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99d15 movzx ebx,BYTE PTR [rbp-0x3c8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fc38)) as u64) as u32 as u64;
            // 99d1c test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99d1f jne 0x99d26
            if fl.ne() { pc = 0x99d26; continue; }
            pc = 0x99d21;
        }
        0x99d21 => {
            // block 99d21
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99d21 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99d24 jmp 0x99d2d
            pc = 0x99d2d; continue;
        }
        0x99d26 => {
            // block 99d26
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99d26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99d26 mov rbx,QWORD PTR [rbp-0x3c0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc40)) as u64;
            pc = 0x99d2d;
        }
        0x99d2d => {
            // block 99d2d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99d2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99d2d lea rdi,[rbp-0x3c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // 99d34 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99d39 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99d3c je 0x9a2f5
            if fl.e() { pc = 0x9a2f5; continue; }
            pc = 0x99d42;
        }
        0x99d42 => {
            // block 99d42
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99d42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99d42 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 99d47 lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 99d4e lea rsi,[rip+0x99cb6] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 99d55 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99d5a mov rax,QWORD PTR [rbp-0x3f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc10)) as u64;
            // 99d61 mov QWORD PTR [rbp-0x3d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc28), (rax) as u64);
            // 99d68 mov rax,QWORD PTR [rbp-0x400]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc00)) as u64;
            // 99d6f mov rcx,QWORD PTR [rbp-0x3f8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc08)) as u64;
            // 99d76 mov QWORD PTR [rbp-0x3e0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc20), (rcx) as u64);
            // 99d7d mov QWORD PTR [rbp-0x3e8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc18), (rax) as u64);
            // 99d84 xorps xmm0,xmm0
            x0 = 0;
            // 99d87 movaps XMMWORD PTR [rbp-0x400],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc00), x0);
            // 99d8e mov QWORD PTR [rbp-0x3f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc10), (0x0u64) as u64);
            // 99d99 mov DWORD PTR [rbp-0x3d0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc30), ((r13 as u32 as u64)) as u32);
            // 99da0 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99da7 lea rbx,[rbp-0x3e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 99dae mov rsi,rbx
            rsi = rbx;
            // 99db1 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99db6 mov rdi,rbx
            rdi = rbx;
            // 99db9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99dbe lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 99dc5 jmp 0x9a2f0
            pc = 0x9a2f0; continue;
        }
        0x99dca => {
            // block 99dca
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99dca mov rbx,QWORD PTR [rbp-0x1e0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            pc = 0x99dd1;
        }
        0x99dd1 => {
            // block 99dd1
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99dd1 lea rdi,[rbp-0x1e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe18);
            // 99dd8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99ddd test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99de0 je 0x99e6e
            if fl.e() { pc = 0x99e6e; continue; }
            pc = 0x99de6;
        }
        0x99de6 => {
            // block 99de6
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99de6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99de6 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99deb lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 99df2 lea rsi,[rip+0x99bf1] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 99df9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99dfe mov rax,QWORD PTR [rbp-0x210]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0)) as u64;
            // 99e05 mov QWORD PTR [rbp-0x1f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe08), (rax) as u64);
            // 99e0c mov rax,QWORD PTR [rbp-0x220]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde0)) as u64;
            // 99e13 mov rcx,QWORD PTR [rbp-0x218]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde8)) as u64;
            // 99e1a mov QWORD PTR [rbp-0x200],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (rcx) as u64);
            // 99e21 mov QWORD PTR [rbp-0x208],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8), (rax) as u64);
            // 99e28 xorps xmm0,xmm0
            x0 = 0;
            // 99e2b movaps XMMWORD PTR [rbp-0x220],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fde0), x0);
            // 99e32 mov QWORD PTR [rbp-0x210],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0), (0x0u64) as u64);
            // 99e3d mov DWORD PTR [rbp-0x1f0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe10), ((r13 as u32 as u64)) as u32);
            // 99e44 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99e4b lea rbx,[rbp-0x208]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // 99e52 mov rsi,rbx
            rsi = rbx;
            // 99e55 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99e5a mov rdi,rbx
            rdi = rbx;
            // 99e5d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99e62 lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 99e69 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99e6e;
        }
        0x99e6e => {
            // block 99e6e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99e6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99e6e mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99e73 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 99e7a lea rsi,[rip+0x9b9eb] # 0x13586c
            rsi = m.env.data_va(303161);
            // 99e81 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99e86 movzx ebx,BYTE PTR [rbp-0x238]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fdc8)) as u64) as u32 as u64;
            // 99e8d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99e90 jne 0x99e97
            if fl.ne() { pc = 0x99e97; continue; }
            pc = 0x99e92;
        }
        0x99e92 => {
            // block 99e92
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99e92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99e92 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99e95 jmp 0x99e9e
            pc = 0x99e9e; continue;
        }
        0x99e97 => {
            // block 99e97
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99e97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99e97 mov rbx,QWORD PTR [rbp-0x230]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdd0)) as u64;
            pc = 0x99e9e;
        }
        0x99e9e => {
            // block 99e9e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99e9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99e9e lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 99ea5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99eaa test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99ead je 0x99f3b
            if fl.e() { pc = 0x99f3b; continue; }
            pc = 0x99eb3;
        }
        0x99eb3 => {
            // block 99eb3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99eb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99eb3 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99eb8 lea rdi,[rbp-0x270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd90);
            // 99ebf lea rsi,[rip+0x9b9a6] # 0x13586c
            rsi = m.env.data_va(303161);
            // 99ec6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99ecb mov rax,QWORD PTR [rbp-0x260]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fda0)) as u64;
            // 99ed2 mov QWORD PTR [rbp-0x248],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb8), (rax) as u64);
            // 99ed9 mov rax,QWORD PTR [rbp-0x270]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd90)) as u64;
            // 99ee0 mov rcx,QWORD PTR [rbp-0x268]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd98)) as u64;
            // 99ee7 mov QWORD PTR [rbp-0x250],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0), (rcx) as u64);
            // 99eee mov QWORD PTR [rbp-0x258],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (rax) as u64);
            // 99ef5 xorps xmm0,xmm0
            x0 = 0;
            // 99ef8 movaps XMMWORD PTR [rbp-0x270],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd90), x0);
            // 99eff mov QWORD PTR [rbp-0x260],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (0x0u64) as u64);
            // 99f0a mov DWORD PTR [rbp-0x240],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fdc0), ((r13 as u32 as u64)) as u32);
            // 99f11 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99f18 lea rbx,[rbp-0x258]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 99f1f mov rsi,rbx
            rsi = rbx;
            // 99f22 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99f27 mov rdi,rbx
            rdi = rbx;
            // 99f2a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99f2f lea rdi,[rbp-0x270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd90);
            // 99f36 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99f3b;
        }
        0x99f3b => {
            // block 99f3b
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f3b mov rax,QWORD PTR [rbp-0x768]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 99f42 movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // 99f45 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 99f47 jne 0x99f4e
            if fl.ne() { pc = 0x99f4e; continue; }
            pc = 0x99f49;
        }
        0x99f49 => {
            // block 99f49
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f49 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 99f4c jmp 0x99f58
            pc = 0x99f58; continue;
        }
        0x99f4e => {
            // block 99f4e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f4e mov rax,QWORD PTR [rbp-0x798]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f868)) as u64;
            // 99f55 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            pc = 0x99f58;
        }
        0x99f58 => {
            // block 99f58
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f58 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 99f5f test rax,rax
            fl = logic::<64>(rax & rax);
            // 99f62 je 0x99f91
            if fl.e() { pc = 0x99f91; continue; }
            pc = 0x99f64;
        }
        0x99f64 => {
            // block 99f64
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f64 mov rbx,rdi
            rbx = rdi;
            // 99f67 mov rsi,QWORD PTR [rbp-0x768]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 99f6e call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99f73 mov DWORD PTR [rbp-0x278],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd88), ((r13 as u32 as u64)) as u32);
            // 99f7a mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99f81 mov rsi,rbx
            rsi = rbx;
            // 99f84 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99f89 mov rdi,rbx
            rdi = rbx;
            // 99f8c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99f91;
        }
        0x99f91 => {
            // block 99f91
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f91 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 99f96 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 99f9d lea rsi,[rip+0x99a67] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 99fa4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99fa9 movzx ebx,BYTE PTR [rbp-0x2a8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd58)) as u64) as u32 as u64;
            // 99fb0 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99fb3 jne 0x99fba
            if fl.ne() { pc = 0x99fba; continue; }
            pc = 0x99fb5;
        }
        0x99fb5 => {
            // block 99fb5
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99fb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99fb5 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99fb8 jmp 0x99fc1
            pc = 0x99fc1; continue;
        }
        0x99fba => {
            // block 99fba
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99fba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99fba mov rbx,QWORD PTR [rbp-0x2a0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd60)) as u64;
            pc = 0x99fc1;
        }
        0x99fc1 => {
            // block 99fc1
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99fc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99fc1 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 99fc8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99fcd test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99fd0 je 0x9a2f5
            if fl.e() { pc = 0x9a2f5; continue; }
            pc = 0x99fd6;
        }
        0x99fd6 => {
            // block 99fd6
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99fd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99fd6 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 99fdb lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 99fe2 lea rsi,[rip+0x99a22] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 99fe9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99fee mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 99ff5 mov QWORD PTR [rbp-0x2b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd48), (rax) as u64);
            // 99ffc mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 9a003 mov rcx,QWORD PTR [rbp-0x2d8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64;
            // 9a00a mov QWORD PTR [rbp-0x2c0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd40), (rcx) as u64);
            // 9a011 mov QWORD PTR [rbp-0x2c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd38), (rax) as u64);
            // 9a018 xorps xmm0,xmm0
            x0 = 0;
            // 9a01b movaps XMMWORD PTR [rbp-0x2e0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd20), x0);
            // 9a022 mov QWORD PTR [rbp-0x2d0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd30), (0x0u64) as u64);
            // 9a02d mov DWORD PTR [rbp-0x2b0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd50), ((r13 as u32 as u64)) as u32);
            // 9a034 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a03b lea rbx,[rbp-0x2c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 9a042 mov rsi,rbx
            rsi = rbx;
            // 9a045 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a04a mov rdi,rbx
            rdi = rbx;
            // 9a04d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a052 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 9a059 jmp 0x9a2f0
            pc = 0x9a2f0; continue;
        }
        0x9a05e => {
            // block 9a05e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a05e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a05e mov rbx,QWORD PTR [rbp-0x420]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbe0)) as u64;
            pc = 0x9a065;
        }
        0x9a065 => {
            // block 9a065
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a065 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a065 lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // 9a06c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a071 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a074 je 0x9a102
            if fl.e() { pc = 0x9a102; continue; }
            pc = 0x9a07a;
        }
        0x9a07a => {
            // block 9a07a
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a07a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a07a mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a07f lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // 9a086 lea rsi,[rip+0x9995d] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a08d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a092 mov rax,QWORD PTR [rbp-0x450]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64;
            // 9a099 mov QWORD PTR [rbp-0x438],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc8), (rax) as u64);
            // 9a0a0 mov rax,QWORD PTR [rbp-0x460]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fba0)) as u64;
            // 9a0a7 mov rcx,QWORD PTR [rbp-0x458]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fba8)) as u64;
            // 9a0ae mov QWORD PTR [rbp-0x440],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0), (rcx) as u64);
            // 9a0b5 mov QWORD PTR [rbp-0x448],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbb8), (rax) as u64);
            // 9a0bc xorps xmm0,xmm0
            x0 = 0;
            // 9a0bf movaps XMMWORD PTR [rbp-0x460],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fba0), x0);
            // 9a0c6 mov QWORD PTR [rbp-0x450],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0), (0x0u64) as u64);
            // 9a0d1 mov DWORD PTR [rbp-0x430],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbd0), ((r13 as u32 as u64)) as u32);
            // 9a0d8 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a0df lea rbx,[rbp-0x448]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // 9a0e6 mov rsi,rbx
            rsi = rbx;
            // 9a0e9 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a0ee mov rdi,rbx
            rdi = rbx;
            // 9a0f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a0f6 lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // 9a0fd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a102;
        }
        0x9a102 => {
            // block 9a102
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a102 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a102 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a107 lea rdi,[rbp-0x478]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // 9a10e lea rsi,[rip+0x9b757] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a115 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a11a movzx ebx,BYTE PTR [rbp-0x478]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb88)) as u64) as u32 as u64;
            // 9a121 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a124 jne 0x9a12b
            if fl.ne() { pc = 0x9a12b; continue; }
            pc = 0x9a126;
        }
        0x9a126 => {
            // block 9a126
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a126 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a126 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a129 jmp 0x9a132
            pc = 0x9a132; continue;
        }
        0x9a12b => {
            // block 9a12b
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a12b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a12b mov rbx,QWORD PTR [rbp-0x470]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb90)) as u64;
            pc = 0x9a132;
        }
        0x9a132 => {
            // block 9a132
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a132 lea rdi,[rbp-0x478]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // 9a139 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a13e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a141 lea rsi,[rbp-0x498]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb68);
            // 9a148 je 0x9a1d2
            if fl.e() { pc = 0x9a1d2; continue; }
            pc = 0x9a14e;
        }
        0x9a14e => {
            // block 9a14e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a14e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a14e mov rbx,rsi
            rbx = rsi;
            // 9a151 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a156 lea rdi,[rbp-0x4b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 9a15d lea rsi,[rip+0x9b708] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a164 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a169 mov rax,QWORD PTR [rbp-0x4a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb60)) as u64;
            // 9a170 mov QWORD PTR [rbp-0x488],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb78), (rax) as u64);
            // 9a177 mov rax,QWORD PTR [rbp-0x4b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb50)) as u64;
            // 9a17e mov rcx,QWORD PTR [rbp-0x4a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb58)) as u64;
            // 9a185 mov QWORD PTR [rbp-0x490],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (rcx) as u64);
            // 9a18c mov QWORD PTR [rbp-0x498],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb68), (rax) as u64);
            // 9a193 xorps xmm0,xmm0
            x0 = 0;
            // 9a196 movaps XMMWORD PTR [rbp-0x4b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb50), x0);
            // 9a19d mov QWORD PTR [rbp-0x4a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb60), (0x0u64) as u64);
            // 9a1a8 mov DWORD PTR [rbp-0x480],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb80), ((r13 as u32 as u64)) as u32);
            // 9a1af mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a1b6 mov rsi,rbx
            rsi = rbx;
            // 9a1b9 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a1be mov rdi,rbx
            rdi = rbx;
            // 9a1c1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a1c6 lea rdi,[rbp-0x4b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 9a1cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a1d2;
        }
        0x9a1d2 => {
            // block 9a1d2
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1d2 mov rax,QWORD PTR [rbp-0x768]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 9a1d9 movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // 9a1dc test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 9a1de jne 0x9a1e5
            if fl.ne() { pc = 0x9a1e5; continue; }
            pc = 0x9a1e0;
        }
        0x9a1e0 => {
            // block 9a1e0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1e0 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 9a1e3 jmp 0x9a1ef
            pc = 0x9a1ef; continue;
        }
        0x9a1e5 => {
            // block 9a1e5
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1e5 mov rax,QWORD PTR [rbp-0x798]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f868)) as u64;
            // 9a1ec mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            pc = 0x9a1ef;
        }
        0x9a1ef => {
            // block 9a1ef
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1ef lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // 9a1f6 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9a1f9 je 0x9a228
            if fl.e() { pc = 0x9a228; continue; }
            pc = 0x9a1fb;
        }
        0x9a1fb => {
            // block 9a1fb
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1fb mov rbx,rdi
            rbx = rdi;
            // 9a1fe mov rsi,QWORD PTR [rbp-0x768]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 9a205 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a20a mov DWORD PTR [rbp-0x4b8],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb48), ((r13 as u32 as u64)) as u32);
            // 9a211 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a218 mov rsi,rbx
            rsi = rbx;
            // 9a21b call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a220 mov rdi,rbx
            rdi = rbx;
            // 9a223 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a228;
        }
        0x9a228 => {
            // block 9a228
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a228 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a228 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9a22d lea rdi,[rbp-0x4e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb18);
            // 9a234 lea rsi,[rip+0x997d0] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 9a23b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a240 movzx ebx,BYTE PTR [rbp-0x4e8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb18)) as u64) as u32 as u64;
            // 9a247 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a24a jne 0x9a251
            if fl.ne() { pc = 0x9a251; continue; }
            pc = 0x9a24c;
        }
        0x9a24c => {
            // block 9a24c
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a24c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a24c shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a24f jmp 0x9a258
            pc = 0x9a258; continue;
        }
        0x9a251 => {
            // block 9a251
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a251 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a251 mov rbx,QWORD PTR [rbp-0x4e0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb20)) as u64;
            pc = 0x9a258;
        }
        0x9a258 => {
            // block 9a258
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a258 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a258 lea rdi,[rbp-0x4e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb18);
            // 9a25f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a264 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a267 je 0x9a2f5
            if fl.e() { pc = 0x9a2f5; continue; }
            pc = 0x9a26d;
        }
        0x9a26d => {
            // block 9a26d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a26d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a26d mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9a272 lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // 9a279 lea rsi,[rip+0x9978b] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 9a280 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a285 mov rax,QWORD PTR [rbp-0x510]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faf0)) as u64;
            // 9a28c mov QWORD PTR [rbp-0x4f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb08), (rax) as u64);
            // 9a293 mov rax,QWORD PTR [rbp-0x520]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae0)) as u64;
            // 9a29a mov rcx,QWORD PTR [rbp-0x518]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae8)) as u64;
            // 9a2a1 mov QWORD PTR [rbp-0x500],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb00), (rcx) as u64);
            // 9a2a8 mov QWORD PTR [rbp-0x508],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faf8), (rax) as u64);
            // 9a2af xorps xmm0,xmm0
            x0 = 0;
            // 9a2b2 movaps XMMWORD PTR [rbp-0x520],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fae0), x0);
            // 9a2b9 mov QWORD PTR [rbp-0x510],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faf0), (0x0u64) as u64);
            // 9a2c4 mov DWORD PTR [rbp-0x4f0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb10), ((r13 as u32 as u64)) as u32);
            // 9a2cb mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a2d2 lea rbx,[rbp-0x508]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_faf8);
            // 9a2d9 mov rsi,rbx
            rsi = rbx;
            // 9a2dc call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a2e1 mov rdi,rbx
            rdi = rbx;
            // 9a2e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a2e9 lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            pc = 0x9a2f0;
        }
        0x9a2f0 => {
            // block 9a2f0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a2f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a2f0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a2f5;
        }
        0x9a2f5 => {
            // block 9a2f5
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a2f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a2f5 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9a2fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a301 mov rax,QWORD PTR [rbp-0x760]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0)) as u64;
            // 9a308 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 9a30f mov rax,QWORD PTR [rbp-0x758]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a8)) as u64;
            // 9a316 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 9a31d mov rax,QWORD PTR [rbp-0x750]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0)) as u64;
            // 9a324 mov QWORD PTR [rbp-0x1a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rax) as u64);
            // 9a32b lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 9a332 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a337 lea r13,[rbp-0x1a0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 9a33e mov rdi,r13
            rdi = r13;
            // 9a341 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a346 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 9a34d mov rsi,QWORD PTR [rbp-0x790]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f870)) as u64;
            // 9a354 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a359 lea rbx,[rbp-0x138]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 9a360 mov rdi,rbx
            rdi = rbx;
            // 9a363 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a368 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 9a36b add r12,0x18
            { let (r, f) = add::<64>(r12, 0x18u64);
              r12 = r;
              fl = f; }
            // 9a36f cmp r15,0x6
            fl = sub::<64>(r15, 0x6u64).1;
            // 9a373 jb 0x997f8
            if fl.b() { pc = 0x997f8; continue; }
            pc = 0x9a379;
        }
        0x9a379 => {
            // block 9a379
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a379 mov rax,QWORD PTR [rbp-0x778]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f888)) as u64;
            // 9a380 cmp rax,0x8
            fl = sub::<64>(rax, 0x8u64).1;
            // 9a384 mov rcx,rax
            rcx = rax;
            // 9a387 jb 0x997ba
            if fl.b() { pc = 0x997ba; continue; }
            pc = 0x9a38d;
        }
        0x9a38d => {
            // block 9a38d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a38d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a38d lea rsi,[rip+0x9b4eb] # 0x13587f
            rsi = m.env.data_va(303180);
            // 9a394 lea rbx,[rbp-0x538]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 9a39b mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9a3a0 mov rdi,rbx
            rdi = rbx;
            // 9a3a3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a3a8 mov esi,0x840
            rsi = (0x840u64) as u32 as u64;
            // 9a3ad mov r14,QWORD PTR [rbp-0x770]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 9a3b4 mov rdi,r14
            rdi = r14;
            // 9a3b7 mov rdx,rbx
            rdx = rbx;
            // 9a3ba call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a3bf lea rdi,[rbp-0x538]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 9a3c6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a3cb mov DWORD PTR [rbp-0x548],0x840
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fab8), (0x840u64) as u32);
            // 9a3d5 mov QWORD PTR [rbp-0x540],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fac0), (r14) as u64);
            // 9a3dc lea rsi,[rbp-0x548]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fab8);
            // 9a3e3 mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 9a3ea call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a3ef lea rsi,[rip+0x9af6b] # 0x135361
            rsi = m.env.data_va(301870);
            // 9a3f6 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9a3fa mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9a3ff mov rdi,rbx
            rdi = rbx;
            // 9a402 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a407 mov rdi,rbx
            rdi = rbx;
            // 9a40a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a40f lea rsi,[rip+0x995d4] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a416 lea rdi,[rbp-0x560]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faa0);
            // 9a41d mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a422 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a427 movzx ebx,BYTE PTR [rbp-0x560]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_faa0)) as u64) as u32 as u64;
            // 9a42e test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a431 jne 0x9a438
            if fl.ne() { pc = 0x9a438; continue; }
            pc = 0x9a433;
        }
        0x9a433 => {
            // block 9a433
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a433 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a433 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a436 jmp 0x9a43f
            pc = 0x9a43f; continue;
        }
        0x9a438 => {
            // block 9a438
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a438 mov rbx,QWORD PTR [rbp-0x558]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faa8)) as u64;
            pc = 0x9a43f;
        }
        0x9a43f => {
            // block 9a43f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a43f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a43f lea rdi,[rbp-0x560]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faa0);
            // 9a446 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a44b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a44e je 0x9a4e0
            if fl.e() { pc = 0x9a4e0; continue; }
            pc = 0x9a454;
        }
        0x9a454 => {
            // block 9a454
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a454 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a454 lea rsi,[rip+0x9958f] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a45b lea rdi,[rbp-0x5a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa60);
            // 9a462 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a467 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a46c mov rax,QWORD PTR [rbp-0x590]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa70)) as u64;
            // 9a473 mov QWORD PTR [rbp-0x570],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa90), (rax) as u64);
            // 9a47a mov rax,QWORD PTR [rbp-0x5a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa60)) as u64;
            // 9a481 mov rcx,QWORD PTR [rbp-0x598]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa68)) as u64;
            // 9a488 mov QWORD PTR [rbp-0x578],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa88), (rcx) as u64);
            // 9a48f mov QWORD PTR [rbp-0x580],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa80), (rax) as u64);
            // 9a496 xorps xmm0,xmm0
            x0 = 0;
            // 9a499 movaps XMMWORD PTR [rbp-0x5a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa60), x0);
            // 9a4a0 mov QWORD PTR [rbp-0x590],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa70), (0x0u64) as u64);
            // 9a4ab mov DWORD PTR [rbp-0x568],0x840
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa98), (0x840u64) as u32);
            // 9a4b5 lea rsi,[rbp-0x580]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // 9a4bc mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a4c3 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a4c8 lea rdi,[rbp-0x580]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // 9a4cf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a4d4 lea rdi,[rbp-0x5a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa60);
            // 9a4db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a4e0;
        }
        0x9a4e0 => {
            // block 9a4e0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a4e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a4e0 lea rsi,[rip+0x9b385] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a4e7 lea rdi,[rbp-0x5b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa48);
            // 9a4ee mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a4f3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a4f8 movzx ebx,BYTE PTR [rbp-0x5b8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fa48)) as u64) as u32 as u64;
            // 9a4ff test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a502 jne 0x9a509
            if fl.ne() { pc = 0x9a509; continue; }
            pc = 0x9a504;
        }
        0x9a504 => {
            // block 9a504
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a504 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a504 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a507 jmp 0x9a510
            pc = 0x9a510; continue;
        }
        0x9a509 => {
            // block 9a509
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a509 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a509 mov rbx,QWORD PTR [rbp-0x5b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa50)) as u64;
            pc = 0x9a510;
        }
        0x9a510 => {
            // block 9a510
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a510 lea rdi,[rbp-0x5b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa48);
            // 9a517 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a51c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a51f je 0x9a5b1
            if fl.e() { pc = 0x9a5b1; continue; }
            pc = 0x9a525;
        }
        0x9a525 => {
            // block 9a525
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a525 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a525 lea rsi,[rip+0x9b340] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a52c lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 9a533 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a538 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a53d mov rax,QWORD PTR [rbp-0x5e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa20)) as u64;
            // 9a544 mov QWORD PTR [rbp-0x5c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa38), (rax) as u64);
            // 9a54b mov rax,QWORD PTR [rbp-0x5f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa10)) as u64;
            // 9a552 mov rcx,QWORD PTR [rbp-0x5e8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa18)) as u64;
            // 9a559 mov QWORD PTR [rbp-0x5d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa30), (rcx) as u64);
            // 9a560 mov QWORD PTR [rbp-0x5d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa28), (rax) as u64);
            // 9a567 xorps xmm0,xmm0
            x0 = 0;
            // 9a56a movaps XMMWORD PTR [rbp-0x5f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa10), x0);
            // 9a571 mov QWORD PTR [rbp-0x5e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa20), (0x0u64) as u64);
            // 9a57c mov DWORD PTR [rbp-0x5c0],0x840
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa40), (0x840u64) as u32);
            // 9a586 lea rsi,[rbp-0x5d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 9a58d mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a594 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a599 lea rdi,[rbp-0x5d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 9a5a0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a5a5 lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 9a5ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a5b1;
        }
        0x9a5b1 => {
            // block 9a5b1
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a5b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a5b1 xorps xmm0,xmm0
            x0 = 0;
            // 9a5b4 movaps XMMWORD PTR [rbp-0x610],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f9f0), x0);
            // 9a5bb mov QWORD PTR [rbp-0x600],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa00), (0x0u64) as u64);
            // 9a5c6 lea rdi,[rbp-0x610]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9f0);
            // 9a5cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a5d2 xorps xmm0,xmm0
            x0 = 0;
            // 9a5d5 movaps XMMWORD PTR [rbp-0x630],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f9d0), x0);
            // 9a5dc mov QWORD PTR [rbp-0x620],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9e0), (0x0u64) as u64);
            // 9a5e7 lea rdi,[rbp-0x630]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9d0);
            // 9a5ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a5f3 lea rsi,[rip+0x9b2a2] # 0x13589c
            rsi = m.env.data_va(303209);
            // 9a5fa lea rbx,[rbp-0x648]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f9b8);
            // 9a601 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9a606 mov rdi,rbx
            rdi = rbx;
            // 9a609 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a60e mov esi,0x841
            rsi = (0x841u64) as u32 as u64;
            // 9a613 mov rdi,r14
            rdi = r14;
            // 9a616 mov rdx,rbx
            rdx = rbx;
            // 9a619 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a61e lea rdi,[rbp-0x648]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9b8);
            // 9a625 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a62a mov DWORD PTR [rbp-0x658],0x841
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9a8), (0x841u64) as u32);
            // 9a634 mov QWORD PTR [rbp-0x650],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9b0), (r14) as u64);
            // 9a63b lea rsi,[rbp-0x658]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9a8);
            // 9a642 mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 9a649 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a64e lea rsi,[rip+0x9ad0c] # 0x135361
            rsi = m.env.data_va(301870);
            // 9a655 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9a659 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9a65e mov rdi,rbx
            rdi = rbx;
            // 9a661 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a666 mov rdi,rbx
            rdi = rbx;
            // 9a669 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a66e lea rsi,[rip+0x99375] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a675 lea rdi,[rbp-0x670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // 9a67c mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a681 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a686 movzx ebx,BYTE PTR [rbp-0x670]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f990)) as u64) as u32 as u64;
            // 9a68d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a690 jne 0x9a697
            if fl.ne() { pc = 0x9a697; continue; }
            pc = 0x9a692;
        }
        0x9a692 => {
            // block 9a692
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a692 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a695 jmp 0x9a69e
            pc = 0x9a69e; continue;
        }
        0x9a697 => {
            // block 9a697
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a697 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a697 mov rbx,QWORD PTR [rbp-0x668]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f998)) as u64;
            pc = 0x9a69e;
        }
        0x9a69e => {
            // block 9a69e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a69e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a69e lea rdi,[rbp-0x670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // 9a6a5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a6aa test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a6ad je 0x9a73f
            if fl.e() { pc = 0x9a73f; continue; }
            pc = 0x9a6b3;
        }
        0x9a6b3 => {
            // block 9a6b3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a6b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a6b3 lea rsi,[rip+0x99330] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a6ba lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 9a6c1 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a6c6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a6cb mov rax,QWORD PTR [rbp-0x6a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f960)) as u64;
            // 9a6d2 mov QWORD PTR [rbp-0x680],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f980), (rax) as u64);
            // 9a6d9 mov rax,QWORD PTR [rbp-0x6b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f950)) as u64;
            // 9a6e0 mov rcx,QWORD PTR [rbp-0x6a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f958)) as u64;
            // 9a6e7 mov QWORD PTR [rbp-0x688],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f978), (rcx) as u64);
            // 9a6ee mov QWORD PTR [rbp-0x690],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f970), (rax) as u64);
            // 9a6f5 xorps xmm0,xmm0
            x0 = 0;
            // 9a6f8 movaps XMMWORD PTR [rbp-0x6b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f950), x0);
            // 9a6ff mov QWORD PTR [rbp-0x6a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f960), (0x0u64) as u64);
            // 9a70a mov DWORD PTR [rbp-0x678],0x841
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f988), (0x841u64) as u32);
            // 9a714 lea rsi,[rbp-0x690]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 9a71b mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a722 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a727 lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 9a72e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a733 lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 9a73a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a73f;
        }
        0x9a73f => {
            // block 9a73f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a73f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a73f lea rsi,[rip+0x9b126] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a746 lea rdi,[rbp-0x6c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f938);
            // 9a74d mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a752 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a757 movzx ebx,BYTE PTR [rbp-0x6c8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f938)) as u64) as u32 as u64;
            // 9a75e test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a761 jne 0x9a768
            if fl.ne() { pc = 0x9a768; continue; }
            pc = 0x9a763;
        }
        0x9a763 => {
            // block 9a763
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a763 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a763 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a766 jmp 0x9a76f
            pc = 0x9a76f; continue;
        }
        0x9a768 => {
            // block 9a768
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a768 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a768 mov rbx,QWORD PTR [rbp-0x6c0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f940)) as u64;
            pc = 0x9a76f;
        }
        0x9a76f => {
            // block 9a76f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a76f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a76f lea rdi,[rbp-0x6c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f938);
            // 9a776 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a77b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a77e je 0x9a810
            if fl.e() { pc = 0x9a810; continue; }
            pc = 0x9a784;
        }
        0x9a784 => {
            // block 9a784
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a784 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a784 lea rsi,[rip+0x9b0e1] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a78b lea rdi,[rbp-0x700]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            // 9a792 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a797 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a79c mov rax,QWORD PTR [rbp-0x6f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f910)) as u64;
            // 9a7a3 mov QWORD PTR [rbp-0x6d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f928), (rax) as u64);
            // 9a7aa mov rax,QWORD PTR [rbp-0x700]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f900)) as u64;
            // 9a7b1 mov rcx,QWORD PTR [rbp-0x6f8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f908)) as u64;
            // 9a7b8 mov QWORD PTR [rbp-0x6e0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f920), (rcx) as u64);
            // 9a7bf mov QWORD PTR [rbp-0x6e8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f918), (rax) as u64);
            // 9a7c6 xorps xmm0,xmm0
            x0 = 0;
            // 9a7c9 movaps XMMWORD PTR [rbp-0x700],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f900), x0);
            // 9a7d0 mov QWORD PTR [rbp-0x6f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f910), (0x0u64) as u64);
            // 9a7db mov DWORD PTR [rbp-0x6d0],0x841
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f930), (0x841u64) as u32);
            // 9a7e5 lea rsi,[rbp-0x6e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 9a7ec mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a7f3 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a7f8 lea rdi,[rbp-0x6e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 9a7ff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a804 lea rdi,[rbp-0x700]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            // 9a80b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a810;
        }
        0x9a810 => {
            // block 9a810
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a810 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a810 xorps xmm0,xmm0
            x0 = 0;
            // 9a813 movaps XMMWORD PTR [rbp-0x720],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f8e0), x0);
            // 9a81a mov QWORD PTR [rbp-0x710],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8f0), (0x0u64) as u64);
            // 9a825 lea rdi,[rbp-0x720]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8e0);
            // 9a82c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a831 xorps xmm0,xmm0
            x0 = 0;
            // 9a834 movaps XMMWORD PTR [rbp-0x740],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f8c0), x0);
            // 9a83b mov QWORD PTR [rbp-0x730],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8d0), (0x0u64) as u64);
            // 9a846 lea rdi,[rbp-0x740]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8c0);
            // 9a84d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a852 add rsp,0x7f8
            { let (r, f) = add::<64>(rsp, 0x7f8u64);
              rsp = r;
              fl = f; }
            // 9a859 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9a85a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 9a85c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 9a85e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9a860 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 9a862 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9a863 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x9a864 => {
            // block 9a864
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a864 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a864 mov r12,rax
            r12 = rax;
            // 9a867 jmp 0x9a9a9
            pc = 0x9a9a9; continue;
        }
        0x9a86c => {
            // block 9a86c
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a86c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a86c mov r12,rax
            r12 = rax;
            // 9a86f jmp 0x9a9c3
            pc = 0x9a9c3; continue;
        }
        0x9a874 => {
            // block 9a874
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a874 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a874 mov r12,rax
            r12 = rax;
            // 9a877 jmp 0x9aa14
            pc = 0x9aa14; continue;
        }
        0x9a87c => {
            // block 9a87c
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a87c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a87c mov r12,rax
            r12 = rax;
            // 9a87f jmp 0x9aa01
            pc = 0x9aa01; continue;
        }
        0x9a884 => {
            // block 9a884
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a884 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a884 mov r12,rax
            r12 = rax;
            // 9a887 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 9a88e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a893 jmp 0x9a9ed
            pc = 0x9a9ed; continue;
        }
        0x9a898 => {
            // block 9a898
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a898 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a898 mov r12,rax
            r12 = rax;
            // 9a89b lea rdi,[rbp-0x378]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc88);
            // 9a8a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a8a7 lea rdi,[rbp-0x390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc70);
            // 9a8ae jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a8b3 => {
            // block 9a8b3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a8b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a8b3 mov r12,rax
            r12 = rax;
            // 9a8b6 lea rdi,[rbp-0x3b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc50);
            // 9a8bd jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a8c2 => {
            // block 9a8c2
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a8c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a8c2 mov r12,rax
            r12 = rax;
            // 9a8c5 lea rdi,[rbp-0x3e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 9a8cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a8d1 lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 9a8d8 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a8dd => {
            // block 9a8dd
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a8dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a8dd mov r12,rax
            r12 = rax;
            // 9a8e0 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 9a8e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a8ec lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 9a8f3 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a8f8 => {
            // block 9a8f8
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a8f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a8f8 mov r12,rax
            r12 = rax;
            // 9a8fb lea rdi,[rbp-0x258]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 9a902 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a907 lea rdi,[rbp-0x270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd90);
            // 9a90e jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a913 => {
            // block 9a913
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a913 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a913 mov r12,rax
            r12 = rax;
            // 9a916 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 9a91d jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a922 => {
            // block 9a922
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a922 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a922 mov r12,rax
            r12 = rax;
            // 9a925 lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 9a92c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a931 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 9a938 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a93a => {
            // block 9a93a
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a93a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a93a mov r12,rax
            r12 = rax;
            // 9a93d lea rdi,[rbp-0x498]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb68);
            // 9a944 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a949 lea rdi,[rbp-0x4b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 9a950 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a952 => {
            // block 9a952
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a952 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a952 mov r12,rax
            r12 = rax;
            // 9a955 lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // 9a95c jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a95e => {
            // block 9a95e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a95e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a95e mov r12,rax
            r12 = rax;
            // 9a961 lea rdi,[rbp-0x508]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faf8);
            // 9a968 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a96d lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // 9a974 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a976 => {
            // block 9a976
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a976 mov r12,rax
            r12 = rax;
            // 9a979 lea rdi,[rbp-0x208]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // 9a980 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a985 lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 9a98c jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a98e => {
            // block 9a98e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a98e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a98e mov r12,rax
            r12 = rax;
            // 9a991 lea rdi,[rbp-0x448]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // 9a998 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a99d lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            pc = 0x9a9a4;
        }
        0x9a9a4 => {
            // block 9a9a4
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a9a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a9a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a9a9;
        }
        0x9a9a9 => {
            // block 9a9a9
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a9a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a9a9 lea rbx,[rbp-0x138]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 9a9b0 lea r13,[rbp-0x1a0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 9a9b7 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9a9be call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a9c3;
        }
        0x9a9c3 => {
            // block 9a9c3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a9c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a9c3 mov rax,QWORD PTR [rbp-0x758]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a8)) as u64;
            // 9a9ca mov rcx,QWORD PTR [rbp-0x760]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0)) as u64;
            // 9a9d1 mov QWORD PTR [rbp-0x1a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rcx) as u64);
            // 9a9d8 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 9a9df mov rax,QWORD PTR [rbp-0x750]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0)) as u64;
            // 9a9e6 mov QWORD PTR [rbp-0x1a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rax) as u64);
            pc = 0x9a9ed;
        }
        0x9a9ed => {
            // block 9a9ed
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a9ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a9ed lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 9a9f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a9f9 mov rdi,r13
            rdi = r13;
            // 9a9fc call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9aa01;
        }
        0x9aa01 => {
            // block 9aa01
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa01 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 9aa08 mov rsi,QWORD PTR [rbp-0x790]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f870)) as u64;
            // 9aa0f call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9aa14;
        }
        0x9aa14 => {
            // block 9aa14
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa14 mov rdi,rbx
            rdi = rbx;
            // 9aa17 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9aa1c;
        }
        0x9aa1c => {
            // block 9aa1c
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa1c mov rdi,r12
            rdi = r12;
            // 9aa1f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa24 mov r12,rax
            r12 = rax;
            pc = 0x9aa27;
        }
        0x9aa27 => {
            // block 9aa27
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa27 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 9aa2b mov rdi,rbx
            rdi = rbx;
            // 9aa2e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa33 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 9aa36 jne 0x9aa27
            if fl.ne() { pc = 0x9aa27; continue; }
            pc = 0x9aa38;
        }
        0x9aa38 => {
            // block 9aa38
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa38 jmp 0x9aac0
            pc = 0x9aac0; continue;
        }
        0x9aa3d => {
            // block 9aa3d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa3d mov r12,rax
            r12 = rax;
            // 9aa40 lea rdi,[rbp-0x538]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 9aa47 jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa49 => {
            // block 9aa49
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa49 mov r12,rax
            r12 = rax;
            // 9aa4c lea rdi,[rbp-0x648]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9b8);
            // 9aa53 jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa55 => {
            // block 9aa55
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa55 mov r12,rax
            r12 = rax;
            // 9aa58 lea rdi,[rbp-0x580]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // 9aa5f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa64 lea rdi,[rbp-0x5a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa60);
            // 9aa6b jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa6d => {
            // block 9aa6d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa6d mov r12,rax
            r12 = rax;
            // 9aa70 lea rdi,[rbp-0x5d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 9aa77 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa7c lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 9aa83 jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa85 => {
            // block 9aa85
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa85 mov r12,rax
            r12 = rax;
            // 9aa88 lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 9aa8f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa94 lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 9aa9b jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa9d => {
            // block 9aa9d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa9d mov r12,rax
            r12 = rax;
            // 9aaa0 lea rdi,[rbp-0x6e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 9aaa7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aaac lea rdi,[rbp-0x700]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            pc = 0x9aab3;
        }
        0x9aab3 => {
            // block 9aab3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aab3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aab3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aab8 jmp 0x9aa1c
            pc = 0x9aa1c; continue;
        }
        0x9aabd => {
            // block 9aabd
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aabd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aabd mov r12,rax
            r12 = rax;
            pc = 0x9aac0;
        }
        0x9aac0 => {
            // block 9aac0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aac0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aac0 lea rdi,[rip+0xae8e9] # 0x1493b0
            rdi = m.env.data_va(383869);
            // 9aac7 call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aacc jmp 0x9aa1c
            pc = 0x9aa1c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
