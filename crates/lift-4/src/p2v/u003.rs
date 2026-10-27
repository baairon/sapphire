//! Mechanically lifted functions (scripts/lift_run.py), module u003. Each one is checked against the original
//! in the world harness (tests/world_u003.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xd5004 size=1844
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5004<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd5004;
    loop {
        match pc {
        0xd5004 => {
            // block d5004
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5004 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5004 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5005 mov rbp,rsp
            rbp = rsp;
            // d5008 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d500a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d500c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d500e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d5010 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5011 sub rsp,0x468
            { let (r, f) = sub::<64>(rsp, 0x468u64);
              rsp = r;
              fl = f; }
            // d5018 mov r13,rsi
            r13 = rsi;
            // d501b mov rbx,rdi
            rbx = rdi;
            // d501e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5021 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d5024 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d5026 call QWORD PTR [rax+0xe8]
            { let r = { let t = m.r64(rax.wrapping_add(0xe8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d502c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d502e je 0xd56b5
            if fl.e() { pc = 0xd56b5; continue; }
            pc = 0xd5034;
        }
        0xd5034 => {
            // block d5034
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5034 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5037 mov rdi,rbx
            rdi = rbx;
            // d503a call QWORD PTR [rax+0x100]
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5040 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5042 je 0xd514e
            if fl.e() { pc = 0xd514e; continue; }
            pc = 0xd5048;
        }
        0xd5048 => {
            // block d5048
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5048 xorps xmm0,xmm0
            x0 = 0;
            // d504b movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d504f mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // d5057 mov eax,DWORD PTR [rbx+0x10]
            rax = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d505a mov DWORD PTR [rbp-0x44],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((rax as u32 as u64)) as u32);
            // d505d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5060 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d5064 lea rdx,[rbp-0x44]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // d5068 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // d506d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5072 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5075 mov rdi,rbx
            rdi = rbx;
            // d5078 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d507a mov edi,DWORD PTR [rbp-0x44]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64) as u32 as u64;
            // d507d call 0x2f83a
            { let r = { let t = m.env.code_va(0x2f83a); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5082 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5084 je 0xd5156
            if fl.e() { pc = 0xd5156; continue; }
            pc = 0xd508a;
        }
        0xd508a => {
            // block d508a
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d508a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d508a movzx esi,WORD PTR [rbx+0x8]
            rsi = (m.r16(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // d508e lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // d5095 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d5097 call 0x2f7d2
            { let r = { let t = m.env.code_va(0x2f7d2); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d509c lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // d50a3 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d50a8 call 0x31108
            { let r = { let t = m.env.code_va(0x31108); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d50ad xorps xmm0,xmm0
            x0 = 0;
            // d50b0 movaps XMMWORD PTR [rbp-0x320],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fce0), x0);
            // d50b7 mov QWORD PTR [rbp-0x310],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), (0x0u64) as u64);
            // d50c2 lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // d50c9 lea rdx,[rbp-0x320]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fce0);
            // d50d0 mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // d50d5 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d50d7 call 0x2f84c
            { let r = { let t = m.env.code_va(0x2f84c); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d50dc mov rsi,QWORD PTR [rbp-0x320]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // d50e3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d50e5 je 0xd54f6
            if fl.e() { pc = 0xd54f6; continue; }
            pc = 0xd50eb;
        }
        0xd50eb => {
            // block d50eb
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d50eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d50eb mov rdx,QWORD PTR [rbp-0x318]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce8)) as u64;
            // d50f2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d50f6 call 0xebeb2
            { let r = { let t = m.env.stub_va(0x67f); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d50fb lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d50ff xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d5101 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d5103 call 0xebfa8
            { let r = { let t = m.env.stub_va(0x775); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5108 cmp rax,0xffffffffffffffff
            fl = sub::<64>(rax, 0xffff_ffff_ffff_ffffu64).1;
            // d510c je 0xd511c
            if fl.e() { pc = 0xd511c; continue; }
            pc = 0xd510e;
        }
        0xd510e => {
            // block d510e
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d510e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d510e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d5112 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d5114 mov rsi,rax
            rsi = rax;
            // d5117 call 0xec008
            { let r = { let t = m.env.stub_va(0x7d5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd511c;
        }
        0xd511c => {
            // block d511c
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d511c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d511c mov rdi,QWORD PTR [rbp-0x320]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // d5123 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d5126 je 0xd513d
            if fl.e() { pc = 0xd513d; continue; }
            pc = 0xd5128;
        }
        0xd5128 => {
            // block d5128
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5128 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5128 cmp QWORD PTR [rbp-0x318],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce8)) as u64, rdi).1;
            // d512f je 0xd5138
            if fl.e() { pc = 0xd5138; continue; }
            pc = 0xd5131;
        }
        0xd5131 => {
            // block d5131
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5131 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5131 mov QWORD PTR [rbp-0x318],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rdi) as u64);
            pc = 0xd5138;
        }
        0xd5138 => {
            // block d5138
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5138 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5138 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd513d;
        }
        0xd513d => {
            // block d513d
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d513d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d513d lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // d5144 call 0x2f7f8
            { let r = { let t = m.env.code_va(0x2f7f8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5149 jmp 0xd52ec
            pc = 0xd52ec; continue;
        }
        0xd514e => {
            // block d514e
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d514e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d514e xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d5151 jmp 0xd56b5
            pc = 0xd56b5; continue;
        }
        0xd5156 => {
            // block d5156
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5156 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5156 mov edi,0x100
            rdi = (0x100u64) as u32 as u64;
            // d515b call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5160 mov QWORD PTR [rbp-0x480],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb80), (rax) as u64);
            // d5167 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d516a mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d516e mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d5173 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d5178 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d517d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5180 mov rdi,rbx
            rdi = rbx;
            // d5183 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5185 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5188 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d518c mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d5191 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // d5196 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d519b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d519e mov rdi,rbx
            rdi = rbx;
            // d51a1 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d51a3 mov r14d,0x3e8
            r14 = (0x3e8u64) as u32 as u64;
            // d51a9 lea r15,[rbp-0x324]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fcdc);
            // d51b0 mov r12d,0x100
            r12 = (0x100u64) as u32 as u64;
            pc = 0xd51b6;
        }
        0xd51b6 => {
            // block d51b6
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d51b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d51b6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d51b9 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d51bd mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d51c2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d51c7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d51ca mov rdi,rbx
            rdi = rbx;
            // d51cd mov rdx,r15
            rdx = r15;
            // d51d0 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d51d2 mov eax,DWORD PTR [rbp-0x324]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc)) as u64) as u32 as u64;
            // d51d8 and eax,r12d
            { let r = (rax as u32 as u64) & (r12 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d51db mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d51dd shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d51e0 sub r14d,ecx
            { let (r, f) = sub::<32>((r14 as u32 as u64), (rcx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // d51e3 je 0xd51e9
            if fl.e() { pc = 0xd51e9; continue; }
            pc = 0xd51e5;
        }
        0xd51e5 => {
            // block d51e5
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d51e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d51e5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d51e7 jne 0xd51b6
            if fl.ne() { pc = 0xd51b6; continue; }
            pc = 0xd51e9;
        }
        0xd51e9 => {
            // block d51e9
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d51e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d51e9 mov QWORD PTR [rbp-0x488],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb78), (r13) as u64);
            // d51f0 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d51f3 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // d51f6 je 0xd56ac
            if fl.e() { pc = 0xd56ac; continue; }
            pc = 0xd51fc;
        }
        0xd51fc => {
            // block d51fc
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d51fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d51fc mov r15d,0xf40000
            r15 = (0xf40000u64) as u32 as u64;
            // d5202 lea r13,[rbp-0x328]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // d5209 mov r14d,0x100
            r14 = (0x100u64) as u32 as u64;
            pc = 0xd520f;
        }
        0xd520f => {
            // block d520f
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d520f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d520f mov QWORD PTR [rbp-0x478],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb88), (r12) as u64);
            // d5216 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5219 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d521d mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d5222 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5227 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d522a mov rdi,rbx
            rdi = rbx;
            // d522d mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // d5230 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5232 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5235 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d5239 mov r12d,0x3e8
            r12 = (0x3e8u64) as u32 as u64;
            // d523f mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d5244 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d5249 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d524e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5251 mov rdi,rbx
            rdi = rbx;
            // d5254 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd5256;
        }
        0xd5256 => {
            // block d5256
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5256 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5256 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5259 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d525d mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d5262 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5267 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d526a mov rdi,rbx
            rdi = rbx;
            // d526d mov rdx,r13
            rdx = r13;
            // d5270 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5272 mov eax,DWORD PTR [rbp-0x328]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcd8)) as u64) as u32 as u64;
            // d5278 and eax,r14d
            { let r = (rax as u32 as u64) & (r14 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d527b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d527d shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d5280 sub r12d,ecx
            { let (r, f) = sub::<32>((r12 as u32 as u64), (rcx as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // d5283 je 0xd5289
            if fl.e() { pc = 0xd5289; continue; }
            pc = 0xd5285;
        }
        0xd5285 => {
            // block d5285
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5285 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5285 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d5287 jne 0xd5256
            if fl.ne() { pc = 0xd5256; continue; }
            pc = 0xd5289;
        }
        0xd5289 => {
            // block d5289
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5289 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5289 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // d528c je 0xd551f
            if fl.e() { pc = 0xd551f; continue; }
            pc = 0xd5292;
        }
        0xd5292 => {
            // block d5292
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5292 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5292 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5295 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d5299 mov rcx,QWORD PTR [rbp-0x478]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb88)) as u64;
            // d52a0 mov rdx,QWORD PTR [rbp-0x480]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb80)) as u64;
            // d52a7 lea rdx,[rdx+rcx*4]
            rdx = rdx.wrapping_add(rcx.wrapping_mul(4));
            // d52ab mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d52b0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d52b5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d52b8 mov rdi,rbx
            rdi = rbx;
            // d52bb call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d52bd mov r12,QWORD PTR [rbp-0x478]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb88)) as u64;
            // d52c4 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // d52c7 add r15d,0x4
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x4u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d52cb cmp r12,0x40
            fl = sub::<64>(r12, 0x40u64).1;
            // d52cf jb 0xd520f
            if fl.b() { pc = 0xd520f; continue; }
            pc = 0xd52d5;
        }
        0xd52d5 => {
            // block d52d5
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d52d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d52d5 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d52d9 mov rsi,QWORD PTR [rbp-0x480]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb80)) as u64;
            // d52e0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d52e5 mov r13,QWORD PTR [rbp-0x488]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb78)) as u64;
            pc = 0xd52ec;
        }
        0xd52ec => {
            // block d52ec
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d52ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d52ec lea rdi,[rbp-0x3c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc40);
            // d52f3 lea rbx,[rbp-0x428]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // d52fa mov rax,QWORD PTR [rip+0x6aec7] # 0x1401c8
            rax = m.r64(m.env.data_va(346517)) as u64;
            // d5301 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // d5305 mov QWORD PTR [rbp-0x438],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc8), (rcx) as u64);
            // d530c add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // d5310 mov QWORD PTR [rbp-0x3c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc40), (rax) as u64);
            // d5317 mov QWORD PTR [rbp-0x430],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbd0), (0x0u64) as u64);
            // d5322 mov rsi,rbx
            rsi = rbx;
            // d5325 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d532a mov QWORD PTR [rbp-0x338],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcc8), (0x0u64) as u64);
            // d5335 mov DWORD PTR [rbp-0x330],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcd0), (0xffffffffu64) as u32);
            // d533f mov r14,QWORD PTR [rip+0x6b072] # 0x1403b8
            r14 = m.r64(m.env.data_va(347013)) as u64;
            // d5346 lea rax,[r14+0x18]
            rax = r14.wrapping_add(0x18);
            // d534a mov QWORD PTR [rbp-0x478],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb88), (rax) as u64);
            // d5351 mov QWORD PTR [rbp-0x438],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc8), (rax) as u64);
            // d5358 add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // d535c mov QWORD PTR [rbp-0x3c0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc40), (r14) as u64);
            // d5363 mov rdi,rbx
            rdi = rbx;
            // d5366 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d536b mov QWORD PTR [rbp-0x488],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb78), (r13) as u64);
            // d5372 mov r15,QWORD PTR [rip+0x6b02f] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // d5379 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // d537d mov QWORD PTR [rbp-0x428],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbd8), (r15) as u64);
            // d5384 mov QWORD PTR [rbp-0x3d0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc30), (0x0u64) as u64);
            // d538f mov QWORD PTR [rbp-0x3d8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc28), (0x0u64) as u64);
            // d539a mov QWORD PTR [rbp-0x3e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc20), (0x0u64) as u64);
            // d53a5 mov QWORD PTR [rbp-0x3e8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc18), (0x0u64) as u64);
            // d53b0 mov DWORD PTR [rbp-0x3c8],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc38), (0x8u64) as u32);
            // d53ba lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d53be mov rdi,rbx
            rdi = rbx;
            // d53c1 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d53c6 xorps xmm0,xmm0
            x0 = 0;
            // d53c9 movaps XMMWORD PTR [rbp-0x450],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fbb0), x0);
            // d53d0 mov QWORD PTR [rbp-0x440],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0), (0x0u64) as u64);
            // d53db movaps XMMWORD PTR [rbp-0x470],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb90), x0);
            // d53e2 mov QWORD PTR [rbp-0x460],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fba0), (0x0u64) as u64);
            // d53ed lea r12,[rbp-0x438]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fbc8);
            // d53f4 lea rbx,[rbp-0x470]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // d53fb lea r13,[rbp-0x450]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // d5402 jmp 0xd540c
            pc = 0xd540c; continue;
        }
        0xd5404 => {
            // block d5404
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5404 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5404 add QWORD PTR [rbp-0x448],0x18
            { let (r, f) = add::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb8)) as u64, 0x18u64);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbb8), (r) as u64);
              fl = f; }
            pc = 0xd540c;
        }
        0xd540c => {
            // block d540c
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d540c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d540c mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // d5411 mov rdi,r12
            rdi = r12;
            // d5414 mov rsi,rbx
            rsi = rbx;
            // d5417 call 0xebf00
            { let r = { let t = m.env.stub_va(0x6cd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d541c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // d541f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // d5423 mov rdi,QWORD PTR [rbp-0x448]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb8)) as u64;
            // d542a test BYTE PTR [rcx+rax*1+0x20],0x5
            fl = logic::<8>(m.r8(rcx.wrapping_add(rax).wrapping_add(0x20)) as u64 & 0x5u64);
            // d542f jne 0xd54cc
            if fl.ne() { pc = 0xd54cc; continue; }
            pc = 0xd5435;
        }
        0xd5435 => {
            // block d5435
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5435 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5435 cmp rdi,QWORD PTR [rbp-0x440]
            fl = sub::<64>(rdi, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0)) as u64).1;
            // d543c je 0xd5448
            if fl.e() { pc = 0xd5448; continue; }
            pc = 0xd543e;
        }
        0xd543e => {
            // block d543e
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d543e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d543e mov rsi,rbx
            rsi = rbx;
            // d5441 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5446 jmp 0xd5404
            pc = 0xd5404; continue;
        }
        0xd5448 => {
            // block d5448
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5448 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5448 mov rdi,r13
            rdi = r13;
            // d544b mov rsi,rbx
            rsi = rbx;
            // d544e call 0xebefa
            { let r = { let t = m.env.stub_va(0x6c7); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5453 jmp 0xd540c
            pc = 0xd540c; continue;
        }
        0xd5455 => {
            // block d5455
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5455 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5455 mov r12,rax
            r12 = rax;
            // d5458 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // d545f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5464 lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // d546b call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5470 mov rax,QWORD PTR [rbp-0x478]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb88)) as u64;
            // d5477 mov QWORD PTR [rbp-0x438],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc8), (rax) as u64);
            // d547e mov QWORD PTR [rbp-0x3c0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc40), (r14) as u64);
            // d5485 mov QWORD PTR [rbp-0x428],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbd8), (r15) as u64);
            pc = 0xd548c;
        }
        0xd548c => {
            // block d548c
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d548c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d548c lea rdi,[rbp-0x3e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // d5493 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5498 lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // d549f call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd54a4;
        }
        0xd54a4 => {
            // block d54a4
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d54a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d54a4 mov rsi,QWORD PTR [rip+0x6ad55] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // d54ab add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // d54af lea rdi,[rbp-0x438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbc8);
            // d54b6 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd54bb;
        }
        0xd54bb => {
            // block d54bb
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d54bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d54bb lea rdi,[rbp-0x3c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc40);
            // d54c2 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d54c7 jmp 0xd56cd
            pc = 0xd56cd; continue;
        }
        0xd54cc => {
            // block d54cc
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d54cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d54cc mov rsi,QWORD PTR [rbp-0x450]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64;
            // d54d3 sub rdi,rsi
            { let (r, f) = sub::<64>(rdi, rsi);
              rdi = r;
              fl = f; }
            // d54d6 sar rdi,0x3
            { let (r, f) = sar::<64>(rdi, 0x3u64, fl); rdi = r; fl = f; }
            // d54da movabs rax,0xaaaaaaaaaaaaaaab
            rax = 0xaaaa_aaaa_aaaa_aaabu64;
            // d54e4 imul rax,rdi
            { let (r, f) = imul::<64>(rax, rdi); rax = r; fl = f; }
            // d54e8 cmp rax,0x8
            fl = sub::<64>(rax, 0x8u64).1;
            // d54ec jae 0xd5527
            if fl.ae() { pc = 0xd5527; continue; }
            pc = 0xd54ee;
        }
        0xd54ee => {
            // block d54ee
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d54ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d54ee xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d54f1 jmp 0xd563d
            pc = 0xd563d; continue;
        }
        0xd54f6 => {
            // block d54f6
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d54f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d54f6 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // d54f9 je 0xd5513
            if fl.e() { pc = 0xd5513; continue; }
            pc = 0xd54fb;
        }
        0xd54fb => {
            // block d54fb
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d54fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d54fb cmp QWORD PTR [rbp-0x318],rsi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce8)) as u64, rsi).1;
            // d5502 je 0xd550b
            if fl.e() { pc = 0xd550b; continue; }
            pc = 0xd5504;
        }
        0xd5504 => {
            // block d5504
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5504 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5504 mov QWORD PTR [rbp-0x318],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rsi) as u64);
            pc = 0xd550b;
        }
        0xd550b => {
            // block d550b
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d550b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d550b mov rdi,rsi
            rdi = rsi;
            // d550e call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd5513;
        }
        0xd5513 => {
            // block d5513
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5513 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5513 lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // d551a call 0x2f7f8
            { let r = { let t = m.env.code_va(0x2f7f8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd551f;
        }
        0xd551f => {
            // block d551f
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d551f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d551f xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d5522 jmp 0xd56ac
            pc = 0xd56ac; continue;
        }
        0xd5527 => {
            // block d5527
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5527 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5527 mov rbx,QWORD PTR [rbp-0x488]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb78)) as u64;
            // d552e lea rdi,[rbx+0x30]
            rdi = rbx.wrapping_add(0x30);
            // d5532 add rsi,0x18
            { let (r, f) = add::<64>(rsi, 0x18u64);
              rsi = r;
              fl = f; }
            // d5536 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d553b mov rsi,QWORD PTR [rbp-0x450]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64;
            // d5542 add rsi,0x30
            { let (r, f) = add::<64>(rsi, 0x30u64);
              rsi = r;
              fl = f; }
            // d5546 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // d554d call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5552 mov al,BYTE PTR [rbp-0x470]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb90)) as u64) as u8 as u64);
            // d5558 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // d555b test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // d555e jne 0xd556c
            if fl.ne() { pc = 0xd556c; continue; }
            pc = 0xd5560;
        }
        0xd5560 => {
            // block d5560
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5560 lea rsi,[rbp-0x46f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb91);
            // d5567 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // d556a jmp 0xd557a
            pc = 0xd557a; continue;
        }
        0xd556c => {
            // block d556c
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d556c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d556c mov rdx,QWORD PTR [rbp-0x468]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb98)) as u64;
            // d5573 mov rsi,QWORD PTR [rbp-0x460]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fba0)) as u64;
            pc = 0xd557a;
        }
        0xd557a => {
            // block d557a
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d557a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d557a mov rcx,rsi
            rcx = rsi;
            // d557d add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // d5580 cmp rsi,rcx
            fl = sub::<64>(rsi, rcx).1;
            // d5583 je 0xd559a
            if fl.e() { pc = 0xd559a; continue; }
            pc = 0xd5585;
        }
        0xd5585 => {
            // block d5585
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5585 movzx edx,BYTE PTR [rsi]
            rdx = (m.r8(rsi) as u64) as u32 as u64;
            // d5588 cmp edx,0xa
            fl = sub::<32>((rdx as u32 as u64), 0xau64).1;
            // d558b je 0xd559a
            if fl.e() { pc = 0xd559a; continue; }
            pc = 0xd558d;
        }
        0xd558d => {
            // block d558d
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d558d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d558d inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // d5590 cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // d5593 jne 0xd5585
            if fl.ne() { pc = 0xd5585; continue; }
            pc = 0xd5595;
        }
        0xd5595 => {
            // block d5595
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5595 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5595 mov rsi,rcx
            rsi = rcx;
            // d5598 jmp 0xd55c1
            pc = 0xd55c1; continue;
        }
        0xd559a => {
            // block d559a
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d559a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d559a cmp rsi,rcx
            fl = sub::<64>(rsi, rcx).1;
            // d559d je 0xd55c1
            if fl.e() { pc = 0xd55c1; continue; }
            pc = 0xd559f;
        }
        0xd559f => {
            // block d559f
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d559f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d559f mov rax,rsi
            rax = rsi;
            // d55a2 jmp 0xd55a9
            pc = 0xd55a9; continue;
        }
        0xd55a4 => {
            // block d55a4
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d55a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55a4 mov BYTE PTR [rsi],dl
            m.w8(rsi, ((rdx as u8 as u64)) as u8);
            // d55a6 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            pc = 0xd55a9;
        }
        0xd55a9 => {
            // block d55a9
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d55a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55a9 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // d55ac cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // d55af je 0xd55bb
            if fl.e() { pc = 0xd55bb; continue; }
            pc = 0xd55b1;
        }
        0xd55b1 => {
            // block d55b1
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d55b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55b1 movzx edx,BYTE PTR [rax]
            rdx = (m.r8(rax) as u64) as u32 as u64;
            // d55b4 cmp edx,0xa
            fl = sub::<32>((rdx as u32 as u64), 0xau64).1;
            // d55b7 je 0xd55a9
            if fl.e() { pc = 0xd55a9; continue; }
            pc = 0xd55b9;
        }
        0xd55b9 => {
            // block d55b9
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d55b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55b9 jmp 0xd55a4
            pc = 0xd55a4; continue;
        }
        0xd55bb => {
            // block d55bb
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d55bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55bb mov al,BYTE PTR [rbp-0x470]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb90)) as u64) as u8 as u64);
            pc = 0xd55c1;
        }
        0xd55c1 => {
            // block d55c1
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d55c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55c1 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d55c3 jne 0xd55dc
            if fl.ne() { pc = 0xd55dc; continue; }
            pc = 0xd55c5;
        }
        0xd55c5 => {
            // block d55c5
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d55c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55c5 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // d55c8 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // d55cb lea rdx,[rbp+rax*1-0x46f]
            rdx = rbp.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fb91);
            // d55d3 lea rax,[rbp-0x46f]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fb91);
            // d55da jmp 0xd55ed
            pc = 0xd55ed; continue;
        }
        0xd55dc => {
            // block d55dc
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d55dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55dc mov rax,QWORD PTR [rbp-0x460]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fba0)) as u64;
            // d55e3 mov rdx,QWORD PTR [rbp-0x468]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb98)) as u64;
            // d55ea add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            pc = 0xd55ed;
        }
        0xd55ed => {
            // block d55ed
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d55ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55ed sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // d55f0 sub rsi,rax
            { let (r, f) = sub::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // d55f3 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // d55fa call 0xebfde
            { let r = { let t = m.env.stub_va(0x7ab); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d55ff lea rdi,[rbx+0x48]
            rdi = rbx.wrapping_add(0x48);
            // d5603 lea rsi,[rbp-0x470]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // d560a call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d560f mov rsi,QWORD PTR [rbp-0x450]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64;
            // d5616 add rsi,0x60
            { let (r, f) = add::<64>(rsi, 0x60u64);
              rsi = r;
              fl = f; }
            // d561a mov rdi,rbx
            rdi = rbx;
            // d561d call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5622 add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // d5626 mov esi,0xa8
            rsi = (0xa8u64) as u32 as u64;
            // d562b add rsi,QWORD PTR [rbp-0x450]
            { let (r, f) = add::<64>(rsi, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64);
              rsi = r;
              fl = f; }
            // d5632 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // d5635 mov rdi,rbx
            rdi = rbx;
            // d5638 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd563d;
        }
        0xd563d => {
            // block d563d
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d563d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d563d lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // d5644 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5649 lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // d5650 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5655 mov rax,QWORD PTR [rbp-0x478]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb88)) as u64;
            // d565c mov QWORD PTR [rbp-0x438],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc8), (rax) as u64);
            // d5663 mov QWORD PTR [rbp-0x3c0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc40), (r14) as u64);
            // d566a mov QWORD PTR [rbp-0x428],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbd8), (r15) as u64);
            // d5671 lea rdi,[rbp-0x3e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // d5678 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d567d lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // d5684 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5689 mov rsi,QWORD PTR [rip+0x6ab70] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // d5690 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // d5694 lea rdi,[rbp-0x438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbc8);
            // d569b call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d56a0 lea rdi,[rbp-0x3c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc40);
            // d56a7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd56ac;
        }
        0xd56ac => {
            // block d56ac
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d56ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d56ac lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d56b0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd56b5;
        }
        0xd56b5 => {
            // block d56b5
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d56b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d56b5 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // d56b8 add rsp,0x468
            { let (r, f) = add::<64>(rsp, 0x468u64);
              rsp = r;
              fl = f; }
            // d56bf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d56c0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d56c2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d56c4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d56c6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d56c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d56c9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd56ca => {
            // block d56ca
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d56ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d56ca mov r12,rax
            r12 = rax;
            pc = 0xd56cd;
        }
        0xd56cd => {
            // block d56cd
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d56cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d56cd lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d56d1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d56d6 mov rdi,r12
            rdi = r12;
            // d56d9 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d56de jmp 0xd56ca
            pc = 0xd56ca; continue;
        }
        0xd56e0 => {
            // block d56e0
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d56e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d56e0 jmp 0xd56ca
            pc = 0xd56ca; continue;
        }
        0xd56e2 => {
            // block d56e2
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d56e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d56e2 jmp 0xd56ca
            pc = 0xd56ca; continue;
        }
        0xd56e4 => {
            // block d56e4
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d56e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d56e4 jmp 0xd5455
            pc = 0xd5455; continue;
        }
        0xd56e9 => {
            // block d56e9
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d56e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d56e9 mov r12,rax
            r12 = rax;
            // d56ec mov rdi,QWORD PTR [rbp-0x320]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // d56f3 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d56f6 je 0xd5712
            if fl.e() { pc = 0xd5712; continue; }
            pc = 0xd56f8;
        }
        0xd56f8 => {
            // block d56f8
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d56f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d56f8 cmp QWORD PTR [rbp-0x318],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce8)) as u64, rdi).1;
            // d56ff je 0xd5708
            if fl.e() { pc = 0xd5708; continue; }
            pc = 0xd5701;
        }
        0xd5701 => {
            // block d5701
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5701 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5701 mov QWORD PTR [rbp-0x318],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rdi) as u64);
            pc = 0xd5708;
        }
        0xd5708 => {
            // block d5708
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5708 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5708 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d570d jmp 0xd5712
            pc = 0xd5712; continue;
        }
        0xd570f => {
            // block d570f
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d570f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d570f mov r12,rax
            r12 = rax;
            pc = 0xd5712;
        }
        0xd5712 => {
            // block d5712
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5712 lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // d5719 call 0x2f7f8
            { let r = { let t = m.env.code_va(0x2f7f8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d571e jmp 0xd56cd
            pc = 0xd56cd; continue;
        }
        0xd5720 => {
            // block d5720
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5720 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5720 mov r12,rax
            r12 = rax;
            // d5723 jmp 0xd54bb
            pc = 0xd54bb; continue;
        }
        0xd5728 => {
            // block d5728
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5728 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5728 mov r12,rax
            r12 = rax;
            // d572b jmp 0xd54a4
            pc = 0xd54a4; continue;
        }
        0xd5730 => {
            // block d5730
            #[cfg(feature = "trace")] eprintln!("f_000d5004 pc d5730 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5730 mov r12,rax
            r12 = rax;
            // d5733 jmp 0xd548c
            pc = 0xd548c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18098 size=2576
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018098<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x18098;
    loop {
        match pc {
        0x18098 => {
            // block 18098
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18098 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18099 mov rbp,rsp
            rbp = rsp;
            // 1809c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1809e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 180a0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 180a2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 180a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 180a5 sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 180ac mov QWORD PTR [rbp-0x140],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rcx) as u64);
            // 180b3 mov r15,rdx
            r15 = rdx;
            // 180b6 mov QWORD PTR [rbp-0x130],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (r15) as u64);
            // 180bd mov QWORD PTR [rbp-0x160],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rsi) as u64);
            // 180c4 mov QWORD PTR [rbp-0x138],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rdi) as u64);
            // 180cb mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 180ce mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // 180d5 mov r13,QWORD PTR [rsi]
            r13 = m.r64(rsi) as u64;
            // 180d8 mov r14,QWORD PTR [r15]
            r14 = m.r64(r15) as u64;
            // 180db jmp 0x18127
            pc = 0x18127; continue;
        }
        0x180dd => {
            // block 180dd
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 180dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 180dd lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 180e4 mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            // 180e8 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 180ec call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 180f1 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 180f5 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 180fa lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 180fe call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18103 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 18107 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1810c lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 18113 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18118 add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 1811f mov rdi,rbx
            rdi = rbx;
            // 18122 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18127;
        }
        0x18127 => {
            // block 18127
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18127 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18127 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1812b cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1812e jne 0x180dd
            if fl.ne() { pc = 0x180dd; continue; }
            pc = 0x18130;
        }
        0x18130 => {
            // block 18130
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18130 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18130 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 18137 mov r14,QWORD PTR [rdx]
            r14 = m.r64(rdx) as u64;
            // 1813a jmp 0x1818c
            pc = 0x1818c; continue;
        }
        0x1813c => {
            // block 1813c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1813c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1813c lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 18143 mov QWORD PTR [rdx+0x8],rax
            m.w64(rdx.wrapping_add(0x8), (rax) as u64);
            // 18147 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 1814b mov r12,rdx
            r12 = rdx;
            // 1814e call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18153 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 18157 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1815c lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 18160 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18165 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 18169 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1816e lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 18175 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1817a add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 18181 mov rdi,rbx
            rdi = rbx;
            // 18184 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18189 mov rdx,r12
            rdx = r12;
            pc = 0x1818c;
        }
        0x1818c => {
            // block 1818c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1818c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1818c mov rbx,QWORD PTR [rdx+0x8]
            rbx = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 18190 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 18193 jne 0x1813c
            if fl.ne() { pc = 0x1813c; continue; }
            pc = 0x18195;
        }
        0x18195 => {
            // block 18195
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18195 mov r14,QWORD PTR [rbp-0x160]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 1819c mov rbx,QWORD PTR [rbp-0x128]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 181a3 mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 181aa jmp 0x181b3
            pc = 0x181b3; continue;
        }
        0x181ac => {
            // block 181ac
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181ac add rbx,0xf8
            { let (r, f) = add::<64>(rbx, 0xf8u64);
              rbx = r;
              fl = f; }
            pc = 0x181b3;
        }
        0x181b3 => {
            // block 181b3
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181b3 mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // 181ba cmp r13,QWORD PTR [r14+0x8]
            fl = sub::<64>(r13, m.r64(r14.wrapping_add(0x8)) as u64).1;
            // 181be sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 181c1 cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 181c6 jne 0x1835d
            if fl.ne() { pc = 0x1835d; continue; }
            pc = 0x181cc;
        }
        0x181cc => {
            // block 181cc
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181cc add r13,0x90
            { let (r, f) = add::<64>(r13, 0x90u64);
              r13 = r;
              fl = f; }
            pc = 0x181d3;
        }
        0x181d3 => {
            // block 181d3
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181d3 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 181d5 jne 0x18900
            if fl.ne() { pc = 0x18900; continue; }
            pc = 0x181db;
        }
        0x181db => {
            // block 181db
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181db mov eax,DWORD PTR [r13-0x90]
            rax = (m.r32(r13.wrapping_add(0xffff_ffff_ffff_ff70)) as u64) as u32 as u64;
            // 181e2 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 181e4 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 181e7 jb 0x18339
            if fl.b() { pc = 0x18339; continue; }
            pc = 0x181ed;
        }
        0x181ed => {
            // block 181ed
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181ed mov rbx,r15
            rbx = r15;
            // 181f0 mov r15,QWORD PTR [rbx+0x8]
            r15 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 181f4 cmp r15,QWORD PTR [rbx+0x10]
            fl = sub::<64>(r15, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // 181f8 je 0x1831a
            if fl.e() { pc = 0x1831a; continue; }
            pc = 0x181fe;
        }
        0x181fe => {
            // block 181fe
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181fe mov rax,QWORD PTR [r13-0x80]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 18202 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 18206 mov rax,QWORD PTR [r13-0x90]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 1820d mov rcx,QWORD PTR [r13-0x88]
            rcx = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 18214 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 18218 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 1821b lea rbx,[r15+0x18]
            rbx = r15.wrapping_add(0x18);
            // 1821f lea rsi,[r13-0x78]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18223 mov rdi,rbx
            rdi = rbx;
            // 18226 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1822b mov ax,WORD PTR [r13-0x34]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u16 as u64);
            // 18230 mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 18235 mov eax,DWORD PTR [r13-0x38]
            rax = (m.r32(r13.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 18239 mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1823d mov rax,QWORD PTR [r13-0x40]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 18241 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 18245 mov rax,QWORD PTR [r13-0x48]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 18249 mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 1824d mov rax,QWORD PTR [r13-0x50]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 18251 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 18255 mov rax,QWORD PTR [r13-0x60]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 18259 mov rcx,QWORD PTR [r13-0x58]
            rcx = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1825d mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 18261 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 18265 lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 18269 mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 18270 lea rsi,[r13-0x30]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18274 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18279 mov r12,r14
            r12 = r14;
            // 1827c lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 18280 mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 18287 lea rsi,[r13-0x18]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 1828b call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18290 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 18297 lea r14,[r15+0x90]
            r14 = r15.wrapping_add(0x90);
            // 1829e mov rdi,r14
            rdi = r14;
            // 182a1 mov rsi,r13
            rsi = r13;
            // 182a4 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182a9 lea rbx,[r15+0xa8]
            rbx = r15.wrapping_add(0xa8);
            // 182b0 lea rsi,[r13+0x18]
            rsi = r13.wrapping_add(0x18);
            // 182b4 mov rdi,rbx
            rdi = rbx;
            // 182b7 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182bc lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 182c3 lea rsi,[r13+0x30]
            rsi = r13.wrapping_add(0x30);
            // 182c7 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182cc mov eax,DWORD PTR [r13+0x60]
            rax = (m.r32(r13.wrapping_add(0x60)) as u64) as u32 as u64;
            // 182d0 mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 182d7 mov rax,QWORD PTR [r13+0x58]
            rax = m.r64(r13.wrapping_add(0x58)) as u64;
            // 182db mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 182e2 mov rax,QWORD PTR [r13+0x48]
            rax = m.r64(r13.wrapping_add(0x48)) as u64;
            // 182e6 mov rcx,QWORD PTR [r13+0x50]
            rcx = m.r64(r13.wrapping_add(0x50)) as u64;
            // 182ea mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 182f1 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 182f8 mov r15,QWORD PTR [rbp-0x130]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 182ff add QWORD PTR [r15+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 18307 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 1830e mov r14,r12
            r14 = r12;
            // 18311 mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 18318 jmp 0x18332
            pc = 0x18332; continue;
        }
        0x1831a => {
            // block 1831a
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1831a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1831a lea rsi,[r13-0x90]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18321 mov rdi,rbx
            rdi = rbx;
            // 18324 mov r15,rdx
            r15 = rdx;
            // 18327 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1832c mov rdx,r15
            rdx = r15;
            // 1832f mov r15,rbx
            r15 = rbx;
            pc = 0x18332;
        }
        0x18332 => {
            // block 18332
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18332 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18332 mov rbx,QWORD PTR [rbp-0x128]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            pc = 0x18339;
        }
        0x18339 => {
            // block 18339
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18339 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18339 lea rax,[r13+0x68]
            rax = r13.wrapping_add(0x68);
            // 1833d cmp rax,QWORD PTR [r14+0x8]
            fl = sub::<64>(rax, m.r64(r14.wrapping_add(0x8)) as u64).1;
            // 18341 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 18344 add r13,0xf8
            { let (r, f) = add::<64>(r13, 0xf8u64);
              r13 = r;
              fl = f; }
            // 1834b cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 18350 je 0x181d3
            if fl.e() { pc = 0x181d3; continue; }
            pc = 0x18356;
        }
        0x18356 => {
            // block 18356
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18356 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18356 add r13,0xffffffffffffff70
            { let (r, f) = add::<64>(r13, 0xffff_ffff_ffff_ff70u64);
              r13 = r;
              fl = f; }
            pc = 0x1835d;
        }
        0x1835d => {
            // block 1835d
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1835d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1835d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1835f je 0x1848d
            if fl.e() { pc = 0x1848d; continue; }
            pc = 0x18365;
        }
        0x18365 => {
            // block 18365
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18365 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18365 mov r15,QWORD PTR [rdx+0x8]
            r15 = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 18369 cmp r15,QWORD PTR [rdx+0x10]
            fl = sub::<64>(r15, m.r64(rdx.wrapping_add(0x10)) as u64).1;
            // 1836d je 0x186fa
            if fl.e() { pc = 0x186fa; continue; }
            pc = 0x18373;
        }
        0x18373 => {
            // block 18373
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18373 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18373 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 18377 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 1837b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1837e mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 18382 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 18386 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 18389 lea rdi,[r15+0x18]
            rdi = r15.wrapping_add(0x18);
            // 1838d mov QWORD PTR [rbp-0x148],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rdi) as u64);
            // 18394 lea rsi,[rbx+0x18]
            rsi = rbx.wrapping_add(0x18);
            // 18398 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1839d mov ax,WORD PTR [rbx+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 183a1 mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 183a6 mov eax,DWORD PTR [rbx+0x58]
            rax = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 183a9 mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 183ad mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 183b1 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 183b5 mov rax,QWORD PTR [rbx+0x48]
            rax = m.r64(rbx.wrapping_add(0x48)) as u64;
            // 183b9 mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 183bd mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 183c1 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 183c5 mov rax,QWORD PTR [rbx+0x30]
            rax = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 183c9 mov rcx,QWORD PTR [rbx+0x38]
            rcx = m.r64(rbx.wrapping_add(0x38)) as u64;
            // 183cd mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 183d1 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 183d5 lea r14,[r15+0x60]
            r14 = r15.wrapping_add(0x60);
            // 183d9 lea rsi,[rbx+0x60]
            rsi = rbx.wrapping_add(0x60);
            // 183dd mov rdi,r14
            rdi = r14;
            // 183e0 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 183e5 mov QWORD PTR [rbp-0x150],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r14) as u64);
            // 183ec lea r14,[r15+0x78]
            r14 = r15.wrapping_add(0x78);
            // 183f0 lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 183f4 mov rdi,r14
            rdi = r14;
            // 183f7 call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 183fc mov QWORD PTR [rbp-0x158],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r14) as u64);
            // 18403 lea r14,[r15+0x90]
            r14 = r15.wrapping_add(0x90);
            // 1840a lea rsi,[rbx+0x90]
            rsi = rbx.wrapping_add(0x90);
            // 18411 mov rdi,r14
            rdi = r14;
            // 18414 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18419 lea r12,[r15+0xa8]
            r12 = r15.wrapping_add(0xa8);
            // 18420 lea rsi,[rbx+0xa8]
            rsi = rbx.wrapping_add(0xa8);
            // 18427 mov rdi,r12
            rdi = r12;
            // 1842a call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1842f lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 18436 lea rsi,[rbx+0xc0]
            rsi = rbx.wrapping_add(0xc0);
            // 1843d call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18442 mov eax,DWORD PTR [rbx+0xf0]
            rax = (m.r32(rbx.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 18448 mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 1844f mov rax,QWORD PTR [rbx+0xe8]
            rax = m.r64(rbx.wrapping_add(0xe8)) as u64;
            // 18456 mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 1845d mov rax,QWORD PTR [rbx+0xd8]
            rax = m.r64(rbx.wrapping_add(0xd8)) as u64;
            // 18464 mov rcx,QWORD PTR [rbx+0xe0]
            rcx = m.r64(rbx.wrapping_add(0xe0)) as u64;
            // 1846b mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 18472 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 18479 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 18480 add QWORD PTR [rdx+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(rdx.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(rdx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 18488 jmp 0x188e6
            pc = 0x188e6; continue;
        }
        0x1848d => {
            // block 1848d
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1848d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1848d mov r14,rdx
            r14 = rdx;
            // 18490 mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 18494 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 1849b mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1849f mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 184a3 mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 184aa mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 184b1 lea rsi,[r13+0x18]
            rsi = r13.wrapping_add(0x18);
            // 184b5 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 184bc call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 184c1 mov ax,WORD PTR [r13+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 184c6 lea rcx,[rbp-0xf0]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 184cd mov rdx,rcx
            rdx = rcx;
            // 184d0 mov WORD PTR [rdx+0x2c],ax
            m.w16(rdx.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 184d4 mov eax,DWORD PTR [r13+0x58]
            rax = (m.r32(r13.wrapping_add(0x58)) as u64) as u32 as u64;
            // 184d8 mov DWORD PTR [rdx+0x28],eax
            m.w32(rdx.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 184db mov rax,QWORD PTR [r13+0x50]
            rax = m.r64(r13.wrapping_add(0x50)) as u64;
            // 184df mov QWORD PTR [rdx+0x20],rax
            m.w64(rdx.wrapping_add(0x20), (rax) as u64);
            // 184e3 mov rax,QWORD PTR [r13+0x48]
            rax = m.r64(r13.wrapping_add(0x48)) as u64;
            // 184e7 mov QWORD PTR [rdx+0x18],rax
            m.w64(rdx.wrapping_add(0x18), (rax) as u64);
            // 184eb mov rax,QWORD PTR [r13+0x40]
            rax = m.r64(r13.wrapping_add(0x40)) as u64;
            // 184ef mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 184f3 mov rax,QWORD PTR [r13+0x30]
            rax = m.r64(r13.wrapping_add(0x30)) as u64;
            // 184f7 mov rcx,QWORD PTR [r13+0x38]
            rcx = m.r64(r13.wrapping_add(0x38)) as u64;
            // 184fb mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 184ff mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 18502 lea rsi,[r13+0x60]
            rsi = r13.wrapping_add(0x60);
            // 18506 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 1850d call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18512 lea rsi,[r13+0x78]
            rsi = r13.wrapping_add(0x78);
            // 18516 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1851d call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18522 lea rsi,[r13+0x90]
            rsi = r13.wrapping_add(0x90);
            // 18529 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18530 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18535 lea rsi,[r13+0xa8]
            rsi = r13.wrapping_add(0xa8);
            // 1853c lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18540 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18545 lea rsi,[r13+0xc0]
            rsi = r13.wrapping_add(0xc0);
            // 1854c lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 18550 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18555 mov eax,DWORD PTR [r13+0xf0]
            rax = (m.r32(r13.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1855c lea rcx,[rbp-0x48]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18560 mov rdx,rcx
            rdx = rcx;
            // 18563 mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 18566 mov rax,QWORD PTR [r13+0xe8]
            rax = m.r64(r13.wrapping_add(0xe8)) as u64;
            // 1856d mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 18571 mov rax,QWORD PTR [r13+0xd8]
            rax = m.r64(r13.wrapping_add(0xd8)) as u64;
            // 18578 mov rcx,QWORD PTR [r13+0xe0]
            rcx = m.r64(r13.wrapping_add(0xe0)) as u64;
            // 1857f mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 18583 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 18586 mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // 18588 mov ecx,DWORD PTR [rbx+0x4]
            rcx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 1858b cmp eax,DWORD PTR [rbp-0x120]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64).1;
            // 18591 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 18594 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 18597 lea rdx,[rax+0x1]
            rdx = rax.wrapping_add(0x1);
            // 1859b cmp ecx,DWORD PTR [rbp-0x11c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee4)) as u64).1;
            // 185a1 cmove rdx,rax
            if fl.e() { rdx = rax; }
            // 185a5 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 185a9 cmp rax,QWORD PTR [rbp-0x110]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64).1;
            // 185b0 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 185b3 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 185b6 add rax,rdx
            { let (r, f) = add::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 185b9 mov ecx,DWORD PTR [rbx+0x8]
            rcx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 185bc cmp ecx,DWORD PTR [rbp-0x118]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64).1;
            // 185c2 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 185c7 mov rdx,0xffffffffffffffff
            rdx = 0xffff_ffff_ffff_ffffu64;
            // 185ce cmovne rcx,rdx
            if fl.ne() { rcx = rdx; }
            // 185d2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 185d5 je 0x1889a
            if fl.e() { pc = 0x1889a; continue; }
            pc = 0x185db;
        }
        0x185db => {
            // block 185db
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 185db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 185db mov r15,QWORD PTR [r14+0x8]
            r15 = m.r64(r14.wrapping_add(0x8)) as u64;
            // 185df cmp r15,QWORD PTR [r14+0x10]
            fl = sub::<64>(r15, m.r64(r14.wrapping_add(0x10)) as u64).1;
            // 185e3 je 0x18717
            if fl.e() { pc = 0x18717; continue; }
            pc = 0x185e9;
        }
        0x185e9 => {
            // block 185e9
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 185e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 185e9 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 185ed mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 185f1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 185f4 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 185f8 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 185fc mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 185ff lea r12,[r15+0x18]
            r12 = r15.wrapping_add(0x18);
            // 18603 lea rsi,[rbx+0x18]
            rsi = rbx.wrapping_add(0x18);
            // 18607 mov rdi,r12
            rdi = r12;
            // 1860a call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1860f mov ax,WORD PTR [rbx+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 18613 mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 18618 mov eax,DWORD PTR [rbx+0x58]
            rax = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1861b mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1861f mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 18623 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 18627 mov rax,QWORD PTR [rbx+0x48]
            rax = m.r64(rbx.wrapping_add(0x48)) as u64;
            // 1862b mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 1862f mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 18633 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 18637 mov rax,QWORD PTR [rbx+0x30]
            rax = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 1863b mov rcx,QWORD PTR [rbx+0x38]
            rcx = m.r64(rbx.wrapping_add(0x38)) as u64;
            // 1863f mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 18643 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 18647 lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 1864b mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 18652 lea rsi,[rbx+0x60]
            rsi = rbx.wrapping_add(0x60);
            // 18656 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1865b lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 1865f mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 18666 lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 1866a call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1866f mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 18676 lea rdi,[r15+0x90]
            rdi = r15.wrapping_add(0x90);
            // 1867d mov QWORD PTR [rbp-0x168],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rdi) as u64);
            // 18684 lea rsi,[rbx+0x90]
            rsi = rbx.wrapping_add(0x90);
            // 1868b call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18690 lea r12,[r15+0xa8]
            r12 = r15.wrapping_add(0xa8);
            // 18697 lea rsi,[rbx+0xa8]
            rsi = rbx.wrapping_add(0xa8);
            // 1869e mov rdi,r12
            rdi = r12;
            // 186a1 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 186a6 lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 186ad lea rsi,[rbx+0xc0]
            rsi = rbx.wrapping_add(0xc0);
            // 186b4 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 186b9 mov eax,DWORD PTR [rbx+0xf0]
            rax = (m.r32(rbx.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 186bf mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 186c6 mov rax,QWORD PTR [rbx+0xe8]
            rax = m.r64(rbx.wrapping_add(0xe8)) as u64;
            // 186cd mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 186d4 mov rax,QWORD PTR [rbx+0xd8]
            rax = m.r64(rbx.wrapping_add(0xd8)) as u64;
            // 186db mov rcx,QWORD PTR [rbx+0xe0]
            rcx = m.r64(rbx.wrapping_add(0xe0)) as u64;
            // 186e2 mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 186e9 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 186f0 add QWORD PTR [r14+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(r14.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 186f8 jmp 0x18722
            pc = 0x18722; continue;
        }
        0x186fa => {
            // block 186fa
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 186fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 186fa mov rdi,rdx
            rdi = rdx;
            // 186fd mov rsi,rbx
            rsi = rbx;
            // 18700 mov r15,rdx
            r15 = rdx;
            // 18703 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18708 mov rdx,r15
            rdx = r15;
            // 1870b mov r15,QWORD PTR [rbp-0x130]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 18712 jmp 0x181ac
            pc = 0x181ac; continue;
        }
        0x18717 => {
            // block 18717
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18717 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18717 mov rdi,r14
            rdi = r14;
            // 1871a mov rsi,rbx
            rsi = rbx;
            // 1871d call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18722;
        }
        0x18722 => {
            // block 18722
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18722 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18722 mov eax,DWORD PTR [rbp-0x120]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64) as u32 as u64;
            // 18728 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 1872a cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 1872d jb 0x1889a
            if fl.b() { pc = 0x1889a; continue; }
            pc = 0x18733;
        }
        0x18733 => {
            // block 18733
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18733 mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1873a mov r12,QWORD PTR [rdi+0x8]
            r12 = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 1873e cmp r12,QWORD PTR [rdi+0x10]
            fl = sub::<64>(r12, m.r64(rdi.wrapping_add(0x10)) as u64).1;
            // 18742 je 0x1888e
            if fl.e() { pc = 0x1888e; continue; }
            pc = 0x18748;
        }
        0x18748 => {
            // block 18748
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18748 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18748 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 1874f mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 18754 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 1875b mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 18762 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 18767 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 1876b lea r15,[r12+0x18]
            r15 = r12.wrapping_add(0x18);
            // 18770 mov rdi,r15
            rdi = r15;
            // 18773 lea rsi,[rbp-0x108]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1877a call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1877f lea rax,[rbp-0xf0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 18786 mov rcx,rax
            rcx = rax;
            // 18789 mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 1878d mov WORD PTR [r12+0x5c],ax
            m.w16(r12.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 18793 mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 18796 mov DWORD PTR [r12+0x58],eax
            m.w32(r12.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1879b mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 1879f mov QWORD PTR [r12+0x50],rax
            m.w64(r12.wrapping_add(0x50), (rax) as u64);
            // 187a4 mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 187a8 mov QWORD PTR [r12+0x48],rax
            m.w64(r12.wrapping_add(0x48), (rax) as u64);
            // 187ad mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 187b1 mov QWORD PTR [r12+0x40],rax
            m.w64(r12.wrapping_add(0x40), (rax) as u64);
            // 187b6 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 187b9 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 187bd mov QWORD PTR [r12+0x38],rcx
            m.w64(r12.wrapping_add(0x38), (rcx) as u64);
            // 187c2 mov QWORD PTR [r12+0x30],rax
            m.w64(r12.wrapping_add(0x30), (rax) as u64);
            // 187c7 lea rdi,[r12+0x60]
            rdi = r12.wrapping_add(0x60);
            // 187cc mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 187d3 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 187da call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 187df mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // 187e6 lea rdi,[r12+0x78]
            rdi = r12.wrapping_add(0x78);
            // 187eb mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 187f2 lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 187f9 call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 187fe mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18805 lea rbx,[r12+0x90]
            rbx = r12.wrapping_add(0x90);
            // 1880d mov rdi,rbx
            rdi = rbx;
            // 18810 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18817 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1881c lea r15,[r12+0xa8]
            r15 = r12.wrapping_add(0xa8);
            // 18824 mov rdi,r15
            rdi = r15;
            // 18827 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1882b call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18830 lea rdi,[r12+0xc0]
            rdi = r12.wrapping_add(0xc0);
            // 18838 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1883c call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18841 lea rax,[rbp-0x48]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18845 mov rcx,rax
            rcx = rax;
            // 18848 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 1884b mov DWORD PTR [r12+0xf0],eax
            m.w32(r12.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 18853 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 18857 mov QWORD PTR [r12+0xe8],rax
            m.w64(r12.wrapping_add(0xe8), (rax) as u64);
            // 1885f mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 18862 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 18866 mov QWORD PTR [r12+0xe0],rcx
            m.w64(r12.wrapping_add(0xe0), (rcx) as u64);
            // 1886e mov QWORD PTR [r12+0xd8],rax
            m.w64(r12.wrapping_add(0xd8), (rax) as u64);
            // 18876 mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1887d add QWORD PTR [rax+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(rax.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(rax.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 18885 mov rbx,QWORD PTR [rbp-0x128]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 1888c jmp 0x1889a
            pc = 0x1889a; continue;
        }
        0x1888e => {
            // block 1888e
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1888e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1888e lea rsi,[rbp-0x120]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 18895 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1889a;
        }
        0x1889a => {
            // block 1889a
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1889a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1889a add r13,0xf8
            { let (r, f) = add::<64>(r13, 0xf8u64);
              r13 = r;
              fl = f; }
            // 188a1 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 188a5 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188aa lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 188ae call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188b3 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 188ba call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188bf lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 188c6 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188cb lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 188d2 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188d7 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 188de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188e3 mov rdx,r14
            rdx = r14;
            pc = 0x188e6;
        }
        0x188e6 => {
            // block 188e6
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 188e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 188e6 mov r15,QWORD PTR [rbp-0x130]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 188ed mov r14,QWORD PTR [rbp-0x160]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 188f4 mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 188fb jmp 0x181ac
            pc = 0x181ac; continue;
        }
        0x18900 => {
            // block 18900
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18900 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18900 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 18903 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 18905 cmp rcx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rcx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 18909 jne 0x18915
            if fl.ne() { pc = 0x18915; continue; }
            pc = 0x1890b;
        }
        0x1890b => {
            // block 1890b
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1890b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1890b mov rax,QWORD PTR [rdx]
            rax = m.r64(rdx) as u64;
            // 1890e cmp rax,QWORD PTR [rdx+0x8]
            fl = sub::<64>(rax, m.r64(rdx.wrapping_add(0x8)) as u64).1;
            // 18912 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x18915;
        }
        0x18915 => {
            // block 18915
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18915 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18915 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 1891c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1891d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1891f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 18921 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18923 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18925 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18926 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x18927 => {
            // block 18927
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18927 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18927 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 1892e mov r13,rax
            r13 = rax;
            // 18931 jmp 0x189c3
            pc = 0x189c3; continue;
        }
        0x18936 => {
            // block 18936
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18936 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18936 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 1893d mov r13,rax
            r13 = rax;
            // 18940 jmp 0x189b7
            pc = 0x189b7; continue;
        }
        0x18942 => {
            // block 18942
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18942 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18942 mov r13,rax
            r13 = rax;
            // 18945 jmp 0x189ab
            pc = 0x189ab; continue;
        }
        0x18947 => {
            // block 18947
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18947 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18947 mov r13,rax
            r13 = rax;
            // 1894a jmp 0x189a3
            pc = 0x189a3; continue;
        }
        0x1894c => {
            // block 1894c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1894c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1894c mov r13,rax
            r13 = rax;
            // 1894f mov rdi,rbx
            rdi = rbx;
            // 18952 jmp 0x1899e
            pc = 0x1899e; continue;
        }
        0x18954 => {
            // block 18954
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18954 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18954 mov r13,rax
            r13 = rax;
            // 18957 jmp 0x18a94
            pc = 0x18a94; continue;
        }
        0x1895c => {
            // block 1895c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1895c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1895c mov r13,rax
            r13 = rax;
            // 1895f jmp 0x18a88
            pc = 0x18a88; continue;
        }
        0x18964 => {
            // block 18964
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18964 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18964 mov r13,rax
            r13 = rax;
            // 18967 jmp 0x18a7c
            pc = 0x18a7c; continue;
        }
        0x1896c => {
            // block 1896c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1896c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1896c mov r13,rax
            r13 = rax;
            // 1896f jmp 0x18a70
            pc = 0x18a70; continue;
        }
        0x18974 => {
            // block 18974
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18974 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18974 mov r13,rax
            r13 = rax;
            // 18977 jmp 0x18a67
            pc = 0x18a67; continue;
        }
        0x1897c => {
            // block 1897c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1897c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1897c mov r13,rax
            r13 = rax;
            // 1897f jmp 0x18a5e
            pc = 0x18a5e; continue;
        }
        0x18984 => {
            // block 18984
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18984 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18984 mov r13,rax
            r13 = rax;
            // 18987 jmp 0x189c3
            pc = 0x189c3; continue;
        }
        0x18989 => {
            // block 18989
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18989 mov r13,rax
            r13 = rax;
            // 1898c jmp 0x189b7
            pc = 0x189b7; continue;
        }
        0x1898e => {
            // block 1898e
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1898e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1898e mov r13,rax
            r13 = rax;
            // 18991 jmp 0x189ab
            pc = 0x189ab; continue;
        }
        0x18993 => {
            // block 18993
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18993 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18993 mov r13,rax
            r13 = rax;
            // 18996 jmp 0x189a3
            pc = 0x189a3; continue;
        }
        0x18998 => {
            // block 18998
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18998 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18998 mov r13,rax
            r13 = rax;
            // 1899b mov rdi,r12
            rdi = r12;
            pc = 0x1899e;
        }
        0x1899e => {
            // block 1899e
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1899e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1899e call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189a3;
        }
        0x189a3 => {
            // block 189a3
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189a3 mov rdi,r14
            rdi = r14;
            // 189a6 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189ab;
        }
        0x189ab => {
            // block 189ab
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189ab mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 189b2 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189b7;
        }
        0x189b7 => {
            // block 189b7
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189b7 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 189be call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189c3;
        }
        0x189c3 => {
            // block 189c3
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189c3 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 189ca jmp 0x18a9b
            pc = 0x18a9b; continue;
        }
        0x189cf => {
            // block 189cf
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189cf mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 189d6 mov r13,rax
            r13 = rax;
            // 189d9 jmp 0x18a52
            pc = 0x18a52; continue;
        }
        0x189db => {
            // block 189db
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189db mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 189e2 mov r13,rax
            r13 = rax;
            // 189e5 jmp 0x18a46
            pc = 0x18a46; continue;
        }
        0x189e7 => {
            // block 189e7
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189e7 mov r13,rax
            r13 = rax;
            // 189ea jmp 0x18a3a
            pc = 0x18a3a; continue;
        }
        0x189ec => {
            // block 189ec
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189ec mov r13,rax
            r13 = rax;
            // 189ef jmp 0x189fc
            pc = 0x189fc; continue;
        }
        0x189f1 => {
            // block 189f1
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189f1 mov r13,rax
            r13 = rax;
            // 189f4 mov rdi,r12
            rdi = r12;
            // 189f7 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189fc;
        }
        0x189fc => {
            // block 189fc
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189fc mov rdi,QWORD PTR [rbp-0x168]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 18a03 jmp 0x18a35
            pc = 0x18a35; continue;
        }
        0x18a05 => {
            // block 18a05
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a05 mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18a0c mov r13,rax
            r13 = rax;
            // 18a0f jmp 0x18a52
            pc = 0x18a52; continue;
        }
        0x18a11 => {
            // block 18a11
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a11 mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18a18 mov r13,rax
            r13 = rax;
            // 18a1b jmp 0x18a46
            pc = 0x18a46; continue;
        }
        0x18a1d => {
            // block 18a1d
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a1d mov r13,rax
            r13 = rax;
            // 18a20 jmp 0x18a3a
            pc = 0x18a3a; continue;
        }
        0x18a22 => {
            // block 18a22
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a22 mov r13,rax
            r13 = rax;
            // 18a25 jmp 0x18a32
            pc = 0x18a32; continue;
        }
        0x18a27 => {
            // block 18a27
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a27 mov r13,rax
            r13 = rax;
            // 18a2a mov rdi,r15
            rdi = r15;
            // 18a2d call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a32;
        }
        0x18a32 => {
            // block 18a32
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a32 mov rdi,rbx
            rdi = rbx;
            pc = 0x18a35;
        }
        0x18a35 => {
            // block 18a35
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a35 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a3a;
        }
        0x18a3a => {
            // block 18a3a
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a3a mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 18a41 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a46;
        }
        0x18a46 => {
            // block 18a46
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a46 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 18a4d call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a52;
        }
        0x18a52 => {
            // block 18a52
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a52 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 18a59 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a5e;
        }
        0x18a5e => {
            // block 18a5e
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a5e lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 18a62 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a67;
        }
        0x18a67 => {
            // block 18a67
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a67 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18a6b call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a70;
        }
        0x18a70 => {
            // block 18a70
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a70 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18a77 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a7c;
        }
        0x18a7c => {
            // block 18a7c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a7c lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 18a83 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a88;
        }
        0x18a88 => {
            // block 18a88
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a88 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 18a8f call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a94;
        }
        0x18a94 => {
            // block 18a94
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a94 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            pc = 0x18a9b;
        }
        0x18a9b => {
            // block 18a9b
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a9b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18aa0 mov rdi,r13
            rdi = r13;
            // 18aa3 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
