//! Mechanically lifted functions (scripts/lift_run.py), module t000. Each one is checked against the original
//! in the world harness (tests/world_t000.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x46c98 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00046c98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x46c98;
    loop {
        match pc {
        0x46c98 => {
            // block 46c98
            #[cfg(feature = "trace")] eprintln!("f_00046c98 pc 46c98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46c98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 46c99 mov rbp,rsp
            rbp = rsp;
            // 46c9c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 46c9d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 46c9e mov rbx,rdi
            rbx = rdi;
            // 46ca1 lea rdx,[rbp-0x10]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // 46ca5 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 46ca7 call 0x46bf4
            { let r = { let t = m.env.code_va(0x46bf4); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46cac test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 46cae je 0x46cbf
            if fl.e() { pc = 0x46cbf; continue; }
            pc = 0x46cb0;
        }
        0x46cb0 => {
            // block 46cb0
            #[cfg(feature = "trace")] eprintln!("f_00046c98 pc 46cb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46cb0 mov rax,QWORD PTR [rbp-0x10]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64;
            // 46cb4 mov QWORD PTR [rbx+0x228],rax
            m.w64(rbx.wrapping_add(0x228), (rax) as u64);
            // 46cbb mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 46cbd jmp 0x46ccc
            pc = 0x46ccc; continue;
        }
        0x46cbf => {
            // block 46cbf
            #[cfg(feature = "trace")] eprintln!("f_00046c98 pc 46cbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46cbf mov QWORD PTR [rbx+0x228],0x0
            m.w64(rbx.wrapping_add(0x228), (0x0u64) as u64);
            // 46cca xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x46ccc;
        }
        0x46ccc => {
            // block 46ccc
            #[cfg(feature = "trace")] eprintln!("f_00046c98 pc 46ccc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46ccc add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 46cd0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 46cd1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 46cd2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
