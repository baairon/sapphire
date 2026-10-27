//! Mechanically lifted functions (scripts/lift_run.py), module h005. Each one is checked against the original
//! in the world harness (tests/world_h005.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x5d10 size=236
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005d10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5d10;
    loop {
        match pc {
        0x5d10 => {
            // block 5d10
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5d11 mov rbp,rsp
            rbp = rsp;
            // 5d14 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5d16 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5d18 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5d1a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5d1b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5d1f mov r14,rdx
            r14 = rdx;
            // 5d22 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5d25 mov rbx,rdi
            rbx = rdi;
            // 5d28 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d2b call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d31 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d33 jne 0x5ddf
            if fl.ne() { pc = 0x5ddf; continue; }
            pc = 0x5d39;
        }
        0x5d39 => {
            // block 5d39
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d39 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d3c lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 5d40 mov rdi,rbx
            rdi = rbx;
            // 5d43 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 5d46 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d4c mov r12d,DWORD PTR [rbp-0x24]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 5d50 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d53 mov rdi,rbx
            rdi = rbx;
            // 5d56 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d5c cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // 5d5f jbe 0x5d68
            if fl.be() { pc = 0x5d68; continue; }
            pc = 0x5d61;
        }
        0x5d61 => {
            // block 5d61
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d61 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            pc = 0x5d68;
        }
        0x5d68 => {
            // block 5d68
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d68 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d6b mov rdi,rbx
            rdi = rbx;
            // 5d6e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d74 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5d76 call 0xcdd26
            { let r = { let t = m.env.code_va(0xcdd26); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d7b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d7d je 0x5db7
            if fl.e() { pc = 0x5db7; continue; }
            pc = 0x5d7f;
        }
        0x5d7f => {
            // block 5d7f
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d7f mov rax,QWORD PTR [rbx+0x210]
            rax = m.r64(rbx.wrapping_add(0x210)) as u64;
            // 5d86 test rax,rax
            fl = logic::<64>(rax & rax);
            // 5d89 jne 0x5da2
            if fl.ne() { pc = 0x5da2; continue; }
            pc = 0x5d8b;
        }
        0x5d8b => {
            // block 5d8b
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d8b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d8e mov rdi,rbx
            rdi = rbx;
            // 5d91 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d97 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d99 je 0x5ddf
            if fl.e() { pc = 0x5ddf; continue; }
            pc = 0x5d9b;
        }
        0x5d9b => {
            // block 5d9b
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d9b mov rax,QWORD PTR [rbx+0x210]
            rax = m.r64(rbx.wrapping_add(0x210)) as u64;
            pc = 0x5da2;
        }
        0x5da2 => {
            // block 5da2
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5da2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5da2 mov ecx,DWORD PTR [rbx+0x24c]
            rcx = (m.r32(rbx.wrapping_add(0x24c)) as u64) as u32 as u64;
            // 5da8 imul ecx,DWORD PTR [rbp-0x24]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // 5dac and ecx,0xfffffffc
            { let r = (rcx as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5daf add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 5db2 mov QWORD PTR [r14],rcx
            m.w64(r14, (rcx) as u64);
            // 5db5 jmp 0x5ded
            pc = 0x5ded; continue;
        }
        0x5db7 => {
            // block 5db7
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5db7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5db7 cmp QWORD PTR [rbx+0x218],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x218)) as u64, 0x0u64).1;
            // 5dbf jne 0x5dd1
            if fl.ne() { pc = 0x5dd1; continue; }
            pc = 0x5dc1;
        }
        0x5dc1 => {
            // block 5dc1
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dc1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5dc4 mov rdi,rbx
            rdi = rbx;
            // 5dc7 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5dcd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5dcf je 0x5ddf
            if fl.e() { pc = 0x5ddf; continue; }
            pc = 0x5dd1;
        }
        0x5dd1 => {
            // block 5dd1
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dd1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 5dd4 je 0x5de3
            if fl.e() { pc = 0x5de3; continue; }
            pc = 0x5dd6;
        }
        0x5dd6 => {
            // block 5dd6
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dd6 mov rax,QWORD PTR [rbx+0x220]
            rax = m.r64(rbx.wrapping_add(0x220)) as u64;
            // 5ddd jmp 0x5dea
            pc = 0x5dea; continue;
        }
        0x5ddf => {
            // block 5ddf
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5ddf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ddf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5de1 jmp 0x5def
            pc = 0x5def; continue;
        }
        0x5de3 => {
            // block 5de3
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5de3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5de3 mov rax,QWORD PTR [rbx+0x218]
            rax = m.r64(rbx.wrapping_add(0x218)) as u64;
            pc = 0x5dea;
        }
        0x5dea => {
            // block 5dea
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dea mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            pc = 0x5ded;
        }
        0x5ded => {
            // block 5ded
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5ded rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ded mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5def;
        }
        0x5def => {
            // block 5def
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5def add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5df3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5df4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5df6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5df8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5dfa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5dfb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3192a size=930
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f, stub:0x961
pub fn f_0003192a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3192a;
    loop {
        match pc {
        0x3192a => {
            // block 3192a
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 3192a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3192a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3192b mov rbp,rsp
            rbp = rsp;
            // 3192e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 31930 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31932 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 31934 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 31936 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31937 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 3193e mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 31941 mov rbx,rsi
            rbx = rsi;
            // 31944 mov r15,rdi
            r15 = rdi;
            // 31947 mov r12,QWORD PTR [rip+0x10ea8a] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // 3194e mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // 31952 mov QWORD PTR [rbp-0x30],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r12) as u64);
            // 31956 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31959 mov rdi,rbx
            rdi = rbx;
            // 3195c call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31962 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31964 je 0x31c8f
            if fl.e() { pc = 0x31c8f; continue; }
            pc = 0x3196a;
        }
        0x3196a => {
            // block 3196a
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 3196a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3196a mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 3196d lea rcx,[rip+0xce4ec] # 0xffe60
            rcx = m.env.data_va(83501);
            // 31974 mov r13d,DWORD PTR [rcx+rax*4]
            r13 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 31978 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3197b lea rdx,[rbp-0x80]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 3197f xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 31982 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 31987 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3198a mov rdi,rbx
            rdi = rbx;
            // 3198d mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 31990 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31993 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31995 je 0x31c92
            if fl.e() { pc = 0x31c92; continue; }
            pc = 0x3199b;
        }
        0x3199b => {
            // block 3199b
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 3199b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3199b mov QWORD PTR [rbp-0xb8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rbx) as u64);
            // 319a2 mov eax,0xffffcccc
            rax = (0xffffccccu64) as u32 as u64;
            // 319a7 and eax,DWORD PTR [rbp-0x80]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 319aa mov ecx,0xffffcccf
            rcx = (0xffffcccfu64) as u32 as u64;
            // 319af and ecx,DWORD PTR [r15+0x8]
            { let r = (rcx as u32 as u64) & m.r32(r15.wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 319b3 or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 319b5 mov eax,DWORD PTR [r15+0xc]
            rax = (m.r32(r15.wrapping_add(0xc)) as u64) as u32 as u64;
            // 319b9 mov edx,DWORD PTR [r15+0x10]
            rdx = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 319bd shl eax,0x4
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 319c0 and eax,0xffffccf0
            { let r = (rax as u32 as u64) & 0xffffccf0u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 319c5 or eax,ecx
            { let r = (rax as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 319c7 shl edx,0xc
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xcu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 319ca or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 319cc mov eax,DWORD PTR [r15+0x14]
            rax = (m.r32(r15.wrapping_add(0x14)) as u64) as u32 as u64;
            // 319d0 shl eax,0x8
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 319d3 or eax,edx
            { let r = (rax as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 319d5 mov DWORD PTR [rbp-0x80],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), ((rax as u32 as u64)) as u32);
            // 319d8 lea r12,[r15+0x30]
            r12 = r15.wrapping_add(0x30);
            // 319dc mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 319e0 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 319e5 mov rdi,r12
            rdi = r12;
            // 319e8 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 319eb mov bx,ax
            rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // 319ee shl ebx,0x10
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 319f1 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 319f5 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 319f8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 319fa mov rdi,r12
            rdi = r12;
            // 319fd call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a00 cwde
            rax = (rax as u16 as i16 as i32 as u32) as u64;
            // 31a01 or eax,ebx
            { let r = (rax as u32 as u64) | (rbx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31a03 mov DWORD PTR [rbp-0x7c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff84), ((rax as u32 as u64)) as u32);
            // 31a06 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a0a mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 31a0f mov rdi,r12
            rdi = r12;
            // 31a12 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a15 cwde
            rax = (rax as u16 as i16 as i32 as u32) as u64;
            // 31a16 mov DWORD PTR [rbp-0x78],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), ((rax as u32 as u64)) as u32);
            // 31a19 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a1d xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31a1f mov rdi,r12
            rdi = r12;
            // 31a22 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a25 mulsd xmm0,QWORD PTR [rip+0xce403] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31a2d cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31a32 mov DWORD PTR [rbp-0x74],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), ((rax as u32 as u64)) as u32);
            // 31a35 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a39 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31a3e mov rdi,r12
            rdi = r12;
            // 31a41 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a44 mulsd xmm0,QWORD PTR [rip+0xce3e4] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31a4c cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31a51 mov DWORD PTR [rbp-0x70],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((rax as u32 as u64)) as u32);
            // 31a54 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a58 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 31a5d mov rdi,r12
            rdi = r12;
            // 31a60 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a63 mulsd xmm0,QWORD PTR [rip+0xce3c5] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31a6b cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31a70 mov DWORD PTR [rbp-0x6c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rax as u32 as u64)) as u32);
            // 31a73 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a77 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31a7c mov rdi,r12
            rdi = r12;
            // 31a7f call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a82 mulsd xmm0,QWORD PTR [rip+0xce3a6] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31a8a cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31a8f mov DWORD PTR [rbp-0x68],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), ((rax as u32 as u64)) as u32);
            // 31a92 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a96 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 31a9b mov rdi,r12
            rdi = r12;
            // 31a9e call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31aa1 mulsd xmm0,QWORD PTR [rip+0xce387] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31aa9 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31aae mov DWORD PTR [rbp-0x64],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), ((rax as u32 as u64)) as u32);
            // 31ab1 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31ab5 mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 31aba mov rdi,r12
            rdi = r12;
            // 31abd call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31ac0 mulsd xmm0,QWORD PTR [rip+0xce368] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31ac8 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31acd mov DWORD PTR [rbp-0x60],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), ((rax as u32 as u64)) as u32);
            // 31ad0 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31ad4 mov esi,0x6
            rsi = (0x6u64) as u32 as u64;
            // 31ad9 mov rdi,r12
            rdi = r12;
            // 31adc call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31adf mulsd xmm0,QWORD PTR [rip+0xce349] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31ae7 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31aec mov DWORD PTR [rbp-0x5c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rax as u32 as u64)) as u32);
            // 31aef mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31af3 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 31af8 mov rdi,r12
            rdi = r12;
            // 31afb call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31afe mulsd xmm0,QWORD PTR [rip+0xce32a] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31b06 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31b0b mov DWORD PTR [rbp-0x58],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rax as u32 as u64)) as u32);
            // 31b0e mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31b12 mov esi,0x8
            rsi = (0x8u64) as u32 as u64;
            // 31b17 mov rdi,r12
            rdi = r12;
            // 31b1a call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31b1d mulsd xmm0,QWORD PTR [rip+0xce30b] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31b25 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31b2a mov DWORD PTR [rbp-0x54],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rax as u32 as u64)) as u32);
            // 31b2d mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31b31 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 31b36 mov rdi,r12
            rdi = r12;
            // 31b39 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31b3c mov bx,ax
            rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // 31b3f shl ebx,0x10
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 31b42 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31b46 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31b4b mov rdi,r12
            rdi = r12;
            // 31b4e call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31b51 cwde
            rax = (rax as u16 as i16 as i32 as u32) as u64;
            // 31b52 or eax,ebx
            { let r = (rax as u32 as u64) | (rbx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31b54 mov DWORD PTR [rbp-0x50],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), ((rax as u32 as u64)) as u32);
            // 31b57 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31b5b mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 31b60 mov rdi,r12
            rdi = r12;
            // 31b63 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31b66 cwde
            rax = (rax as u16 as i16 as i32 as u32) as u64;
            // 31b67 mov DWORD PTR [rbp-0x4c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((rax as u32 as u64)) as u32);
            // 31b6a mov eax,DWORD PTR [r15+0x18]
            rax = (m.r32(r15.wrapping_add(0x18)) as u64) as u32 as u64;
            // 31b6e mov ecx,DWORD PTR [r15+0x1c]
            rcx = (m.r32(r15.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 31b72 and ecx,0x10
            { let r = (rcx as u32 as u64) & 0x10u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 31b75 shl ecx,0x4
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 31b78 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31b7b or eax,ecx
            { let r = (rax as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31b7d mov DWORD PTR [rbp-0x48],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((rax as u32 as u64)) as u32);
            // 31b80 movzx eax,WORD PTR [r15+0x22]
            rax = (m.r16(r15.wrapping_add(0x22)) as u64) as u32 as u64;
            // 31b85 shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 31b88 movsx ecx,WORD PTR [r15+0x20]
            rcx = (sx::<16>(m.r16(r15.wrapping_add(0x20)) as u64) as u64) as u32 as u64;
            // 31b8d or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 31b8f mov DWORD PTR [rbp-0x44],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((rcx as u32 as u64)) as u32);
            // 31b92 movsd xmm0,QWORD PTR [r15+0x28]
            x0 = m.r64(r15.wrapping_add(0x28)) as u128;
            // 31b98 mulsd xmm0,QWORD PTR [rip+0xce298] # 0xffe38
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83461)) as u128))).to_bits());
            // 31ba0 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31ba5 mov DWORD PTR [rbp-0x40],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rax as u32 as u64)) as u32);
            // 31ba8 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 31bb3 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 31bb8 mov QWORD PTR [rbp-0x88],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rax) as u64);
            // 31bbf xorps xmm0,xmm0
            x0 = 0;
            // 31bc2 movaps XMMWORD PTR [rbp-0xb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff50), x0);
            // 31bc9 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // 31bd4 lea r15,[rbp-0xb0]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31bdb lea r12,[rbp-0x90]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 31be2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 31be4 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31be6 jmp 0x31bf6
            pc = 0x31bf6; continue;
        }
        0x31be8 => {
            // block 31be8
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31be8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31be8 mov rcx,QWORD PTR [rbp-0xa8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // 31bef mov r14,QWORD PTR [rbp-0xa0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            pc = 0x31bf6;
        }
        0x31bf6 => {
            // block 31bf6
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31bf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31bf6 lea eax,[r13+rbx*1+0x0]
            rax = (r13.wrapping_add(rbx)) as u32 as u64;
            // 31bfb mov DWORD PTR [rbp-0x90],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff70), ((rax as u32 as u64)) as u32);
            // 31c01 mov eax,DWORD PTR [rbp+rbx*4-0x80]
            rax = (m.r32(rbp.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            // 31c05 mov DWORD PTR [rbp-0x8c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff74), ((rax as u32 as u64)) as u32);
            // 31c0b mov DWORD PTR [rbp-0x88],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0xffffffffu64) as u32);
            // 31c15 mov DWORD PTR [rbp-0x84],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), (0x0u64) as u32);
            // 31c1f cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // 31c22 je 0x31c43
            if fl.e() { pc = 0x31c43; continue; }
            pc = 0x31c24;
        }
        0x31c24 => {
            // block 31c24
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31c24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31c24 mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 31c2b mov rdx,QWORD PTR [rbp-0x88]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 31c32 mov QWORD PTR [rcx+0x8],rdx
            m.w64(rcx.wrapping_add(0x8), (rdx) as u64);
            // 31c36 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 31c39 add QWORD PTR [rbp-0xa8],0x10
            { let (r, f) = add::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, 0x10u64);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (r) as u64);
              fl = f; }
            // 31c41 jmp 0x31c4e
            pc = 0x31c4e; continue;
        }
        0x31c43 => {
            // block 31c43
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31c43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31c43 mov rdi,r15
            rdi = r15;
            // 31c46 mov rsi,r12
            rsi = r12;
            // 31c49 call 0xebb2e
            { let r = { let t = m.env.stub_va(0x2fb); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31c4e;
        }
        0x31c4e => {
            // block 31c4e
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31c4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31c4e inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 31c51 cmp rbx,0x10
            fl = sub::<64>(rbx, 0x10u64).1;
            // 31c55 jle 0x31be8
            if fl.le() { pc = 0x31be8; continue; }
            pc = 0x31c57;
        }
        0x31c57 => {
            // block 31c57
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31c57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31c57 mov rdi,QWORD PTR [rbp-0xb8]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // 31c5e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 31c61 mov rax,QWORD PTR [rax+0xca0]
            rax = m.r64(rax.wrapping_add(0xca0)) as u64;
            // 31c68 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31c6f call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31c71 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 31c74 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31c7b call 0xebb7c
            { let r = { let t = m.env.stub_va(0x349); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31c80 mov rbx,QWORD PTR [rip+0x10e751] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 31c87 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 31c8a mov r12,rbx
            r12 = rbx;
            // 31c8d jmp 0x31c92
            pc = 0x31c92; continue;
        }
        0x31c8f => {
            // block 31c8f
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31c8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31c8f xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x31c92;
        }
        0x31c92 => {
            // block 31c92
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31c92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31c92 cmp r12,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 31c96 jne 0x31cc4
            if fl.ne() { pc = 0x31cc4; continue; }
            pc = 0x31c98;
        }
        0x31c98 => {
            // block 31c98
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31c98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31c98 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 31c9b add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 31ca2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 31ca3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 31ca5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 31ca7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 31ca9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 31cab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 31cac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x31cad => {
            // block 31cad
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31cad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31cad mov rbx,rax
            rbx = rax;
            // 31cb0 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31cb7 call 0xebb7c
            { let r = { let t = m.env.stub_va(0x349); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31cbc mov rdi,rbx
            rdi = rbx;
            // 31cbf call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x31cc4 => {
            // block 31cc4
            #[cfg(feature = "trace")] eprintln!("f_0003192a pc 31cc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31cc4 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44c00 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044c00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44c00;
    loop {
        match pc {
        0x44c00 => {
            // block 44c00
            #[cfg(feature = "trace")] eprintln!("f_00044c00 pc 44c00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44c00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44c01 mov rbp,rsp
            rbp = rsp;
            // 44c04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44c06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44c08 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 44c0a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44c0b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44c0f mov r15,rsi
            r15 = rsi;
            // 44c12 mov r14,rdi
            r14 = rdi;
            // 44c15 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 44c19 mov rdx,r15
            rdx = r15;
            // 44c1c call 0xebb46
            { let r = { let t = m.env.stub_va(0x313); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44c21 mov r12,rax
            r12 = rax;
            // 44c24 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 44c28 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 44c2b je 0x44c31
            if fl.e() { pc = 0x44c31; continue; }
            pc = 0x44c2d;
        }
        0x44c2d => {
            // block 44c2d
            #[cfg(feature = "trace")] eprintln!("f_00044c00 pc 44c2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44c2d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 44c2f jmp 0x44c58
            pc = 0x44c58; continue;
        }
        0x44c31 => {
            // block 44c31
            #[cfg(feature = "trace")] eprintln!("f_00044c00 pc 44c31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44c31 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 44c36 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44c3b mov rbx,rax
            rbx = rax;
            // 44c3e mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 44c41 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 44c44 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 44c48 mov rdi,r14
            rdi = r14;
            // 44c4b mov rdx,r12
            rdx = r12;
            // 44c4e mov rcx,rbx
            rcx = rbx;
            // 44c51 call 0xebb5e
            { let r = { let t = m.env.stub_va(0x32b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44c56 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x44c58;
        }
        0x44c58 => {
            // block 44c58
            #[cfg(feature = "trace")] eprintln!("f_00044c00 pc 44c58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44c58 mov rax,rbx
            rax = rbx;
            // 44c5b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44c5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44c60 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 44c62 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44c64 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 44c66 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44c67 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e528 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e528<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e528;
    loop {
        match pc {
        0x2e528 => {
            // block 2e528
            #[cfg(feature = "trace")] eprintln!("f_0002e528 pc 2e528 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e528 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e529 mov rbp,rsp
            rbp = rsp;
            // 2e52c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e52e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e530 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e532 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e533 mov r14,rdx
            r14 = rdx;
            // 2e536 mov r12,rsi
            r12 = rsi;
            // 2e539 mov r15,rdi
            r15 = rdi;
            // 2e53c test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2e53f je 0x2e58c
            if fl.e() { pc = 0x2e58c; continue; }
            pc = 0x2e541;
        }
        0x2e541 => {
            // block 2e541
            #[cfg(feature = "trace")] eprintln!("f_0002e528 pc 2e541 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e541 mov rbx,QWORD PTR [r15+0x10]
            rbx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 2e545 jmp 0x2e54b
            pc = 0x2e54b; continue;
        }
        0x2e547 => {
            // block 2e547
            #[cfg(feature = "trace")] eprintln!("f_0002e528 pc 2e547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e547 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            pc = 0x2e54b;
        }
        0x2e54b => {
            // block 2e54b
            #[cfg(feature = "trace")] eprintln!("f_0002e528 pc 2e54b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e54b cmp rbx,QWORD PTR [r15+0x18]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x18)) as u64).1;
            // 2e54f je 0x2e58c
            if fl.e() { pc = 0x2e58c; continue; }
            pc = 0x2e551;
        }
        0x2e551 => {
            // block 2e551
            #[cfg(feature = "trace")] eprintln!("f_0002e528 pc 2e551 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e551 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 2e554 je 0x2e575
            if fl.e() { pc = 0x2e575; continue; }
            pc = 0x2e556;
        }
        0x2e556 => {
            // block 2e556
            #[cfg(feature = "trace")] eprintln!("f_0002e528 pc 2e556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e556 cmp BYTE PTR [r12],0x0
            fl = sub::<8>(m.r8(r12) as u64, 0x0u64).1;
            // 2e55b je 0x2e575
            if fl.e() { pc = 0x2e575; continue; }
            pc = 0x2e55d;
        }
        0x2e55d => {
            // block 2e55d
            #[cfg(feature = "trace")] eprintln!("f_0002e528 pc 2e55d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e55d mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 2e560 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2e563 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e566 mov rdi,r12
            rdi = r12;
            // 2e569 mov rsi,rax
            rsi = rax;
            // 2e56c call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e571 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e573 jne 0x2e547
            if fl.ne() { pc = 0x2e547; continue; }
            pc = 0x2e575;
        }
        0x2e575 => {
            // block 2e575
            #[cfg(feature = "trace")] eprintln!("f_0002e528 pc 2e575 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e575 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 2e578 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2e57b mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 2e57f mov rsi,r14
            rsi = r14;
            // 2e582 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e583 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e585 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e587 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e589 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e58a jmp rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2e58c => {
            // block 2e58c
            #[cfg(feature = "trace")] eprintln!("f_0002e528 pc 2e58c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e58c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e58e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e58f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e591 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e593 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e595 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e596 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x167f0 size=12
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000167f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x167f0;
    loop {
        match pc {
        0x167f0 => {
            // block 167f0
            #[cfg(feature = "trace")] eprintln!("f_000167f0 pc 167f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 167f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 167f1 mov rbp,rsp
            rbp = rsp;
            // 167f4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 167f7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 167f8 jmp QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44b5e size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044b5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44b5e;
    loop {
        match pc {
        0x44b5e => {
            // block 44b5e
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44b5f mov rbp,rsp
            rbp = rsp;
            // 44b62 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 44b66 test rax,rax
            fl = logic::<64>(rax & rax);
            // 44b69 je 0x44b9c
            if fl.e() { pc = 0x44b9c; continue; }
            pc = 0x44b6b;
        }
        0x44b6b => {
            // block 44b6b
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b6b mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 44b6d jmp 0x44b72
            pc = 0x44b72; continue;
        }
        0x44b6f => {
            // block 44b6f
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b6f mov rax,rdx
            rax = rdx;
            pc = 0x44b72;
        }
        0x44b72 => {
            // block 44b72
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b72 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 44b75 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 44b77 jge 0x44b83
            if fl.ge() { pc = 0x44b83; continue; }
            pc = 0x44b79;
        }
        0x44b79 => {
            // block 44b79
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b79 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 44b7c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 44b7f jne 0x44b6f
            if fl.ne() { pc = 0x44b6f; continue; }
            pc = 0x44b81;
        }
        0x44b81 => {
            // block 44b81
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b81 jmp 0x44ba8
            pc = 0x44ba8; continue;
        }
        0x44b83 => {
            // block 44b83
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b83 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 44b85 jge 0x44bb0
            if fl.ge() { pc = 0x44bb0; continue; }
            pc = 0x44b87;
        }
        0x44b87 => {
            // block 44b87
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b87 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 44b8b test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 44b8e jne 0x44b6f
            if fl.ne() { pc = 0x44b6f; continue; }
            pc = 0x44b90;
        }
        0x44b90 => {
            // block 44b90
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b90 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 44b93 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 44b97 mov rsi,rax
            rsi = rax;
            // 44b9a jmp 0x44bb3
            pc = 0x44bb3; continue;
        }
        0x44b9c => {
            // block 44b9c
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b9c add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 44ba0 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 44ba3 mov rsi,rdi
            rsi = rdi;
            // 44ba6 jmp 0x44bb3
            pc = 0x44bb3; continue;
        }
        0x44ba8 => {
            // block 44ba8
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44ba8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44ba8 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 44bab mov rsi,rax
            rsi = rax;
            // 44bae jmp 0x44bb3
            pc = 0x44bb3; continue;
        }
        0x44bb0 => {
            // block 44bb0
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44bb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44bb0 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x44bb3;
        }
        0x44bb3 => {
            // block 44bb3
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44bb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44bb3 mov rax,rsi
            rax = rsi;
            // 44bb6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44bb7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dea2 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dea2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2dea2;
    loop {
        match pc {
        0x2dea2 => {
            // block 2dea2
            #[cfg(feature = "trace")] eprintln!("f_0002dea2 pc 2dea2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dea2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dea3 mov rbp,rsp
            rbp = rsp;
            // 2dea6 lea rdi,[rip+0x11923b] # 0x1470e8
            rdi = m.env.data_va(374965);
            // 2dead lea rsi,[rip+0xfffffffffffffe92] # 0x2dd46
            rsi = m.env.code_va(0x2dd46);
            // 2deb4 call 0xec266
            { let r = { let t = m.env.stub_va(0xa33); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2deb9 mov rax,QWORD PTR [rip+0x193b78] # 0x1c1a38
            rax = m.r64(m.env.data_va(877061)) as u64;
            // 2dec0 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dec3 je 0x2dec8
            if fl.e() { pc = 0x2dec8; continue; }
            pc = 0x2dec5;
        }
        0x2dec5 => {
            // block 2dec5
            #[cfg(feature = "trace")] eprintln!("f_0002dea2 pc 2dec5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dec5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dec6 jmp rax
            { let r = { let t = rax; m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
        0x2dec8 => {
            // block 2dec8
            #[cfg(feature = "trace")] eprintln!("f_0002dea2 pc 2dec8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dec8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2deca pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2decb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28dca size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028dca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28dca;
    loop {
        match pc {
        0x28dca => {
            // block 28dca
            #[cfg(feature = "trace")] eprintln!("f_00028dca pc 28dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28dca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28dcb mov rbp,rsp
            rbp = rsp;
            // 28dce mov rcx,rsi
            rcx = rsi;
            // 28dd1 mov rax,rdi
            rax = rdi;
            // 28dd4 mov rsi,QWORD PTR [rcx]
            rsi = m.r64(rcx) as u64;
            // 28dd7 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 28dda je 0x28e0d
            if fl.e() { pc = 0x28e0d; continue; }
            pc = 0x28ddc;
        }
        0x28ddc => {
            // block 28ddc
            #[cfg(feature = "trace")] eprintln!("f_00028dca pc 28ddc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ddc mov ecx,DWORD PTR [rcx+0x8]
            rcx = (m.r32(rcx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28ddf test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 28de1 je 0x28e0d
            if fl.e() { pc = 0x28e0d; continue; }
            pc = 0x28de3;
        }
        0x28de3 => {
            // block 28de3
            #[cfg(feature = "trace")] eprintln!("f_00028dca pc 28de3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28de3 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 28de6 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 28de9 je 0x28e0d
            if fl.e() { pc = 0x28e0d; continue; }
            pc = 0x28deb;
        }
        0x28deb => {
            // block 28deb
            #[cfg(feature = "trace")] eprintln!("f_00028dca pc 28deb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28deb mov edx,DWORD PTR [rax+0x8]
            rdx = (m.r32(rax.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28dee test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 28df0 je 0x28e0d
            if fl.e() { pc = 0x28e0d; continue; }
            pc = 0x28df2;
        }
        0x28df2 => {
            // block 28df2
            #[cfg(feature = "trace")] eprintln!("f_00028dca pc 28df2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28df2 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 28df4 jne 0x28dfd
            if fl.ne() { pc = 0x28dfd; continue; }
            pc = 0x28df6;
        }
        0x28df6 => {
            // block 28df6
            #[cfg(feature = "trace")] eprintln!("f_00028dca pc 28df6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28df6 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 28df8 cmp rsi,rdi
            fl = sub::<64>(rsi, rdi).1;
            // 28dfb je 0x28e0f
            if fl.e() { pc = 0x28e0f; continue; }
            pc = 0x28dfd;
        }
        0x28dfd => {
            // block 28dfd
            #[cfg(feature = "trace")] eprintln!("f_00028dca pc 28dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28dfd cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 28dff cmova ecx,edx
            if fl.a() { rcx = ((rdx as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 28e02 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 28e04 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e09 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 28e0b jmp 0x28e0f
            pc = 0x28e0f; continue;
        }
        0x28e0d => {
            // block 28e0d
            #[cfg(feature = "trace")] eprintln!("f_00028dca pc 28e0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e0d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x28e0f;
        }
        0x28e0f => {
            // block 28e0f
            #[cfg(feature = "trace")] eprintln!("f_00028dca pc 28e0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e0f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28e10 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdb6a size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000db6a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdb6a;
    loop {
        match pc {
        0xdb6a => {
            // block db6a
            #[cfg(feature = "trace")] eprintln!("f_0000db6a pc db6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db6a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // db6b mov rbp,rsp
            rbp = rsp;
            // db6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // db70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // db71 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // db74 mov rbx,rdi
            rbx = rdi;
            // db77 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db7a call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db80 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db82 je 0xdb88
            if fl.e() { pc = 0xdb88; continue; }
            pc = 0xdb84;
        }
        0xdb84 => {
            // block db84
            #[cfg(feature = "trace")] eprintln!("f_0000db6a pc db84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db84 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db86 jmp 0xdbc0
            pc = 0xdbc0; continue;
        }
        0xdb88 => {
            // block db88
            #[cfg(feature = "trace")] eprintln!("f_0000db6a pc db88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db88 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db8b mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // db90 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db92 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // db97 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // db9d mov rdi,rbx
            rdi = rbx;
            // dba0 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dba3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dba6 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // dbaa mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // dbaf mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // dbb4 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // dbba mov rdi,rbx
            rdi = rbx;
            // dbbd call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdbc0;
        }
        0xdbc0 => {
            // block dbc0
            #[cfg(feature = "trace")] eprintln!("f_0000db6a pc dbc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbc0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dbc1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dbc3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dbc4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6808c size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006808c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6808c;
    loop {
        match pc {
        0x6808c => {
            // block 6808c
            #[cfg(feature = "trace")] eprintln!("f_0006808c pc 6808c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6808c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6808d mov rbp,rsp
            rbp = rsp;
            // 68090 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 68093 mov rax,QWORD PTR [rax+0xb60]
            rax = m.r64(rax.wrapping_add(0xb60)) as u64;
            // 6809a mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 6809c lea rdx,[rip+0x99c1d] # 0x101cc0
            rdx = m.env.data_va(91277);
            // 680a3 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 680a6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 680a7 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41ed2 size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041ed2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41ed2;
    loop {
        match pc {
        0x41ed2 => {
            // block 41ed2
            #[cfg(feature = "trace")] eprintln!("f_00041ed2 pc 41ed2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41ed2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41ed3 mov rbp,rsp
            rbp = rsp;
            // 41ed6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 41ed7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 41ed8 mov rbx,rsi
            rbx = rsi;
            // 41edb mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 41ee2 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 41ee5 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 41ee7 lea rdx,[rip+0xbe4d2] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 41eee mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 41ef1 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 41ef5 mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 41efa mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 41f00 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41f03 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 41f05 je 0x41f0c
            if fl.e() { pc = 0x41f0c; continue; }
            pc = 0x41f07;
        }
        0x41f07 => {
            // block 41f07
            #[cfg(feature = "trace")] eprintln!("f_00041ed2 pc 41f07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41f07 mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 41f0a mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0x41f0c;
        }
        0x41f0c => {
            // block 41f0c
            #[cfg(feature = "trace")] eprintln!("f_00041ed2 pc 41f0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41f0c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 41f10 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 41f11 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41f12 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7b00 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007b00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x7b00;
    loop {
        match pc {
        0x7b00 => {
            // block 7b00
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7b01 mov rbp,rsp
            rbp = rsp;
            // 7b04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7b06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7b08 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7b0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7b0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7b0d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7b11 mov r15,rsi
            r15 = rsi;
            // 7b14 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 7b18 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 7b1b je 0x7c1b
            if fl.e() { pc = 0x7c1b; continue; }
            pc = 0x7b21;
        }
        0x7b21 => {
            // block 7b21
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b21 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 7b25 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 7b2a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7b2e mov r14,QWORD PTR [rbp-0x38]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7b32;
        }
        0x7b32 => {
            // block 7b32
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b32 movsd xmm0,QWORD PTR [r15-0x8]
            x0 = m.r64(r15.wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7b38 addsd xmm0,QWORD PTR [rip+0xf6d98] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b40 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 7b45 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 7b47 mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // 7b4c js 0x7b5d
            if fl.s() { pc = 0x7b5d; continue; }
            pc = 0x7b4e;
        }
        0x7b4e => {
            // block 7b4e
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b4e cmp eax,0x3ff
            fl = sub::<32>((rax as u32 as u64), 0x3ffu64).1;
            // 7b53 mov ecx,0x3ff
            rcx = (0x3ffu64) as u32 as u64;
            // 7b58 cmovg eax,ecx
            if fl.g() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 7b5b mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x7b5d;
        }
        0x7b5d => {
            // block 7b5d
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b5d mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7b61 lea r12,[rax+0x1ff]
            r12 = rax.wrapping_add(0x1ff);
            // 7b68 movsd xmm0,QWORD PTR [r15]
            x0 = m.r64(r15) as u128;
            // 7b6d addsd xmm0,QWORD PTR [rip+0xf6d63] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b75 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7b7a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7b7c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7b7e js 0x7b90
            if fl.s() { pc = 0x7b90; continue; }
            pc = 0x7b80;
        }
        0x7b80 => {
            // block 7b80
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b80 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7b86 mov eax,0x3ff
            rax = (0x3ffu64) as u32 as u64;
            // 7b8b cmovg ecx,eax
            if fl.g() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7b8e mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7b90;
        }
        0x7b90 => {
            // block 7b90
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b90 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7b93 shl ebx,0x6
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 7b96 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 7b98 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7b9b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7b9e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7ba3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7ba6 mov rdi,r14
            rdi = r14;
            // 7ba9 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bac mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bae call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bb1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bb3 je 0x7c1e
            if fl.e() { pc = 0x7c1e; continue; }
            pc = 0x7bb5;
        }
        0x7bb5 => {
            // block 7bb5
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7bb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bb5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bb8 add r12,0x200
            { let (r, f) = add::<64>(r12, 0x200u64);
              r12 = r;
              fl = f; }
            // 7bbf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7bc2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7bc7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7bca mov rdi,r14
            rdi = r14;
            // 7bcd mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bd0 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bd2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bd5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bd7 je 0x7c1e
            if fl.e() { pc = 0x7c1e; continue; }
            pc = 0x7bd9;
        }
        0x7bd9 => {
            // block 7bd9
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7bd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bd9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bdc add r12d,0x200
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x200u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 7be3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7be8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7beb mov rdi,r14
            rdi = r14;
            // 7bee mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bf1 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bf3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bf6 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7bf9 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7bfd cmp rax,0x1ff
            fl = sub::<64>(rax, 0x1ffu64).1;
            // 7c03 ja 0x7c1e
            if fl.a() { pc = 0x7c1e; continue; }
            pc = 0x7c05;
        }
        0x7c05 => {
            // block 7c05
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c05 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // 7c09 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 7c0c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7c10 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 7c13 jne 0x7b32
            if fl.ne() { pc = 0x7b32; continue; }
            pc = 0x7c19;
        }
        0x7c19 => {
            // block 7c19
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c19 jmp 0x7c1e
            pc = 0x7c1e; continue;
        }
        0x7c1b => {
            // block 7c1b
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c1e;
        }
        0x7c1e => {
            // block 7c1e
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1e mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 7c21 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7c25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7c26 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7c28 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7c2a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7c2c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7c2e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7c2f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dd46 size=247
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dd46<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2dd46;
    loop {
        match pc {
        0x2dd46 => {
            // block 2dd46
            #[cfg(feature = "trace")] eprintln!("f_0002dd46 pc 2dd46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd46 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dd47 mov rbp,rsp
            rbp = rsp;
            // 2dd4a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2dd4c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2dd4d mov rax,QWORD PTR [rip+0x112694] # 0x1403e8
            rax = m.r64(m.env.data_va(347061)) as u64;
            // 2dd54 mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            // 2dd57 lea rsi,[rip+0x118d4a] # 0x146aa8
            rsi = m.env.data_va(373365);
            // 2dd5e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2dd60 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 2dd65 mov rdi,rbx
            rdi = rbx;
            // 2dd68 call 0xebf5a
            { let r = { let t = m.env.stub_va(0x727); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dd6d mov r14,rax
            r14 = rax;
            // 2dd70 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2dd73 je 0x2de38
            if fl.e() { pc = 0x2de38; continue; }
            pc = 0x2dd79;
        }
        0x2dd79 => {
            // block 2dd79
            #[cfg(feature = "trace")] eprintln!("f_0002dd46 pc 2dd79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd79 mov rdi,rbx
            rdi = rbx;
            // 2dd7c mov rsi,r14
            rsi = r14;
            // 2dd7f call 0xebf36
            { let r = { let t = m.env.stub_va(0x703); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dd84 mov QWORD PTR [rip+0x193c95],rax # 0x1c1a20
            m.w64(m.env.data_va(877037), (rax) as u64);
            // 2dd8b test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dd8e je 0x2de2c
            if fl.e() { pc = 0x2de2c; continue; }
            pc = 0x2dd94;
        }
        0x2dd94 => {
            // block 2dd94
            #[cfg(feature = "trace")] eprintln!("f_0002dd46 pc 2dd94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dd94 lea rsi,[rip+0x118d2d] # 0x146ac8
            rsi = m.env.data_va(373397);
            // 2dd9b mov rdi,rax
            rdi = rax;
            // 2dd9e call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dda3 mov QWORD PTR [rip+0x193c7e],rax # 0x1c1a28
            m.w64(m.env.data_va(877045), (rax) as u64);
            // 2ddaa mov rdi,QWORD PTR [rip+0x193c6f] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2ddb1 lea rsi,[rip+0x118d30] # 0x146ae8
            rsi = m.env.data_va(373429);
            // 2ddb8 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ddbd mov QWORD PTR [rip+0x193c6c],rax # 0x1c1a30
            m.w64(m.env.data_va(877053), (rax) as u64);
            // 2ddc4 mov rdi,QWORD PTR [rip+0x193c55] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2ddcb lea rsi,[rip+0x118d36] # 0x146b08
            rsi = m.env.data_va(373461);
            // 2ddd2 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ddd7 mov QWORD PTR [rip+0x193c5a],rax # 0x1c1a38
            m.w64(m.env.data_va(877061), (rax) as u64);
            // 2ddde mov rdi,QWORD PTR [rip+0x193c3b] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2dde5 lea rsi,[rip+0x118d3c] # 0x146b28
            rsi = m.env.data_va(373493);
            // 2ddec call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ddf1 mov QWORD PTR [rip+0x193c48],rax # 0x1c1a40
            m.w64(m.env.data_va(877069), (rax) as u64);
            // 2ddf8 mov rdi,QWORD PTR [rip+0x193c21] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2ddff lea rsi,[rip+0x118d42] # 0x146b48
            rsi = m.env.data_va(373525);
            // 2de06 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2de0b mov QWORD PTR [rip+0x193c36],rax # 0x1c1a48
            m.w64(m.env.data_va(877077), (rax) as u64);
            // 2de12 mov rdi,QWORD PTR [rip+0x193c07] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2de19 lea rsi,[rip+0x118d48] # 0x146b68
            rsi = m.env.data_va(373557);
            // 2de20 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2de25 mov QWORD PTR [rip+0x193c24],rax # 0x1c1a50
            m.w64(m.env.data_va(877085), (rax) as u64);
            pc = 0x2de2c;
        }
        0x2de2c => {
            // block 2de2c
            #[cfg(feature = "trace")] eprintln!("f_0002dd46 pc 2de2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2de2c mov rdi,r14
            rdi = r14;
            // 2de2f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2de30 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2de32 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2de33 jmp 0xebf42
            { let r = { let t = m.env.stub_va(0x70f); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2de38 => {
            // block 2de38
            #[cfg(feature = "trace")] eprintln!("f_0002dd46 pc 2de38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2de38 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2de39 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2de3b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2de3c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4b54 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004b54<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4b54;
    loop {
        match pc {
        0x4b54 => {
            // block 4b54
            #[cfg(feature = "trace")] eprintln!("f_00004b54 pc 4b54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b54 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4b55 mov rbp,rsp
            rbp = rsp;
            // 4b58 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4b5a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4b5c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4b5d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4b5e mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 4b61 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 4b64 mov rbx,rdi
            rbx = rdi;
            // 4b67 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b6a mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 4b6d call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b73 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4b75 je 0x4b7b
            if fl.e() { pc = 0x4b7b; continue; }
            pc = 0x4b77;
        }
        0x4b77 => {
            // block 4b77
            #[cfg(feature = "trace")] eprintln!("f_00004b54 pc 4b77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b77 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4b79 jmp 0x4bb4
            pc = 0x4bb4; continue;
        }
        0x4b7b => {
            // block 4b7b
            #[cfg(feature = "trace")] eprintln!("f_00004b54 pc 4b7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b7b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b7e mov rdi,rbx
            rdi = rbx;
            // 4b81 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b87 mov r9,QWORD PTR [rbx]
            r9 = m.r64(rbx) as u64;
            // 4b8a mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 4b8d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b8f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4b91 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // 4b95 lea rax,[rip+0xfa454] # 0xfeff0
            rax = m.env.data_va(79805);
            // 4b9c mov esi,DWORD PTR [rax+rdx*4]
            rsi = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4b9f mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 4ba4 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 4baa mov rdi,rbx
            rdi = rbx;
            // 4bad mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 4bb0 call QWORD PTR [r9+0x30]
            { let r = { let t = m.r64(r9.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4bb4;
        }
        0x4bb4 => {
            // block 4bb4
            #[cfg(feature = "trace")] eprintln!("f_00004b54 pc 4bb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bb4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4bb8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4bb9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4bbb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4bbd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4bbe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41c1e size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00041c1e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41c1e;
    loop {
        match pc {
        0x41c1e => {
            // block 41c1e
            #[cfg(feature = "trace")] eprintln!("f_00041c1e pc 41c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41c1e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41c1f mov rbp,rsp
            rbp = rsp;
            // 41c22 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 41c24 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 41c26 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 41c28 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 41c2a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 41c2b sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 41c2f mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 41c33 mov r12,rdi
            r12 = rdi;
            // 41c36 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 41c3a mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 41c3f sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 41c42 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 41c46 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 41c49 mov rax,rbx
            rax = rbx;
            // 41c4c shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 41c50 je 0x41c5e
            if fl.e() { pc = 0x41c5e; continue; }
            pc = 0x41c52;
        }
        0x41c52 => {
            // block 41c52
            #[cfg(feature = "trace")] eprintln!("f_00041c1e pc 41c52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41c52 mov rdi,r12
            rdi = r12;
            // 41c55 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x41c5e => {
            // block 41c5e
            #[cfg(feature = "trace")] eprintln!("f_00041c1e pc 41c5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41c5e lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 41c63 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 41c68 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 41c6c sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 41c6f mov rcx,r15
            rcx = r15;
            // 41c72 sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 41c76 movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 41c80 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 41c83 ja 0x41c91
            if fl.a() { pc = 0x41c91; continue; }
            pc = 0x41c85;
        }
        0x41c85 => {
            // block 41c85
            #[cfg(feature = "trace")] eprintln!("f_00041c1e pc 41c85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41c85 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 41c88 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 41c8b cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 41c8f jmp 0x41c9b
            pc = 0x41c9b; continue;
        }
        0x41c91 => {
            // block 41c91
            #[cfg(feature = "trace")] eprintln!("f_00041c1e pc 41c91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41c91 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x41c9b;
        }
        0x41c9b => {
            // block 41c9b
            #[cfg(feature = "trace")] eprintln!("f_00041c1e pc 41c9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41c9b mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 41ca0 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 41ca4 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 41ca7 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 41cab mov r13,r14
            r13 = r14;
            // 41cae sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 41cb2 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 41cba mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41cbe xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 41cc0 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 41cc3 je 0x41cd5
            if fl.e() { pc = 0x41cd5; continue; }
            pc = 0x41cc5;
        }
        0x41cc5 => {
            // block 41cc5
            #[cfg(feature = "trace")] eprintln!("f_00041c1e pc 41cc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41cc5 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 41ccd call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41cd2 mov rbx,rax
            rbx = rax;
            pc = 0x41cd5;
        }
        0x41cd5 => {
            // block 41cd5
            #[cfg(feature = "trace")] eprintln!("f_00041c1e pc 41cd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41cd5 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 41cd9 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 41cdd mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 41ce1 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 41ce3 mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 41ce7 lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 41cec mov rdi,rbx
            rdi = rbx;
            // 41cef mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 41cf3 mov rsi,r15
            rsi = r15;
            // 41cf6 mov rdx,r14
            rdx = r14;
            // 41cf9 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41cfe mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 41d02 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 41d06 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 41d0b mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 41d0f mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 41d13 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 41d17 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 41d1c mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 41d20 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 41d24 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 41d28 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 41d2c call 0xebb34
            { let r = { let t = m.env.stub_va(0x301); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41d31 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 41d35 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 41d36 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 41d38 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 41d3a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 41d3c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 41d3e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41d3f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28a4f size=93
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028a4f<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28a4f;
    loop {
        match pc {
        0x28a4f => {
            // block 28a4f
            #[cfg(feature = "trace")] eprintln!("f_00028a4f pc 28a4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a4f push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28a50 mov rbp,rsp
            rbp = rsp;
            // 28a53 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28a55 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28a57 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28a58 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 28a59 mov r15,rsi
            r15 = rsi;
            // 28a5c mov r14,rdi
            r14 = rdi;
            // 28a5f mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 28a62 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 28a66 jmp 0x28a99
            pc = 0x28a99; continue;
        }
        0x28a68 => {
            // block 28a68
            #[cfg(feature = "trace")] eprintln!("f_00028a4f pc 28a68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a68 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 28a6c mov rdi,r14
            rdi = r14;
            // 28a6f call 0xebace
            { let r = { let t = m.env.stub_va(0x29b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28a74 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 28a78 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28a7b je 0x28a8a
            if fl.e() { pc = 0x28a8a; continue; }
            pc = 0x28a7d;
        }
        0x28a7d => {
            // block 28a7d
            #[cfg(feature = "trace")] eprintln!("f_00028a4f pc 28a7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a7d mov rbx,rax
            rbx = rax;
            // 28a80 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28a83 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28a86 jne 0x28a7d
            if fl.ne() { pc = 0x28a7d; continue; }
            pc = 0x28a88;
        }
        0x28a88 => {
            // block 28a88
            #[cfg(feature = "trace")] eprintln!("f_00028a4f pc 28a88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a88 jmp 0x28a99
            pc = 0x28a99; continue;
        }
        0x28a8a => {
            // block 28a8a
            #[cfg(feature = "trace")] eprintln!("f_00028a4f pc 28a8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a8a mov rax,rbx
            rax = rbx;
            pc = 0x28a8d;
        }
        0x28a8d => {
            // block 28a8d
            #[cfg(feature = "trace")] eprintln!("f_00028a4f pc 28a8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a8d mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28a91 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28a94 mov rax,rbx
            rax = rbx;
            // 28a97 jne 0x28a8d
            if fl.ne() { pc = 0x28a8d; continue; }
            pc = 0x28a99;
        }
        0x28a99 => {
            // block 28a99
            #[cfg(feature = "trace")] eprintln!("f_00028a4f pc 28a99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a99 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 28a9c jne 0x28a68
            if fl.ne() { pc = 0x28a68; continue; }
            pc = 0x28a9e;
        }
        0x28a9e => {
            // block 28a9e
            #[cfg(feature = "trace")] eprintln!("f_00028a4f pc 28a9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a9e mov rax,r14
            rax = r14;
            // 28aa1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 28aa5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28aa6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28aa8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28aaa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28aab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8b4 size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d8b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd8b4;
    loop {
        match pc {
        0xd8b4 => {
            // block d8b4
            #[cfg(feature = "trace")] eprintln!("f_0000d8b4 pc d8b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8b5 mov rbp,rsp
            rbp = rsp;
            // d8b8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d8ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d8bc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d8bd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d8be mov r14,rdx
            r14 = rdx;
            // d8c1 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d8c4 mov rbx,rdi
            rbx = rdi;
            // d8c7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8ca call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d8d2 je 0xd8d8
            if fl.e() { pc = 0xd8d8; continue; }
            pc = 0xd8d4;
        }
        0xd8d4 => {
            // block d8d4
            #[cfg(feature = "trace")] eprintln!("f_0000d8b4 pc d8d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8d4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d6 jmp 0xd90c
            pc = 0xd90c; continue;
        }
        0xd8d8 => {
            // block d8d8
            #[cfg(feature = "trace")] eprintln!("f_0000d8b4 pc d8d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8d8 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // d8df mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8e2 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // d8e5 lea rdx,[rip+0xf1ad4] # 0xff3c0
            rdx = m.env.data_va(80781);
            // d8ec mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // d8ef lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d8f3 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // d8f8 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // d8fe mov rdi,rbx
            rdi = rbx;
            // d901 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d904 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // d908 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0xd90c;
        }
        0xd90c => {
            // block d90c
            #[cfg(feature = "trace")] eprintln!("f_0000d8b4 pc d90c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d90c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d910 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d911 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d913 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d915 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d916 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x764e size=998
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000764e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x764e;
    loop {
        match pc {
        0x764e => {
            // block 764e
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 764e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 764e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 764f mov rbp,rsp
            rbp = rsp;
            // 7652 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7654 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7655 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 7659 mov r14,rcx
            r14 = rcx;
            // 765c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 765e add esi,0xfffffffd
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffdu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 7661 cmp esi,0x4
            fl = sub::<32>((rsi as u32 as u64), 0x4u64).1;
            // 7664 jbe 0x7683
            if fl.be() { pc = 0x7683; continue; }
            pc = 0x7666;
        }
        0x7666 => {
            // block 7666
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7666 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7666 xorps xmm0,xmm0
            x0 = 0;
            // 7669 cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 766d movsd QWORD PTR [r14+rax*8],xmm0
            m.w64(r14.wrapping_add(rax.wrapping_mul(8)), x0 as u64);
            // 7673 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 7676 cmp rax,0x400
            fl = sub::<64>(rax, 0x400u64).1;
            // 767c jne 0x7666
            if fl.ne() { pc = 0x7666; continue; }
            pc = 0x767e;
        }
        0x767e => {
            // block 767e
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 767e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 767e jmp 0x7a15
            pc = 0x7a15; continue;
        }
        0x7683 => {
            // block 7683
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7683 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7683 lea rax,[rip+0x396] # 0x7a20
            rax = m.env.code_va(0x7a20);
            // 768a movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // 768e add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 7691 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x7693 => {
            // block 7693
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7693 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7693 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 7695 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 7698 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 769b lea rcx,[rip+0xf718e] # 0xfe830
            rcx = m.env.data_va(77821);
            // 76a2 movsd xmm0,QWORD PTR [rcx+rax*8]
            x0 = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u128;
            // 76a7 movsd QWORD PTR [rbp-0x18],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), x0 as u64);
            // 76ac xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 76ae movsd xmm1,QWORD PTR [rip+0xf719a] # 0xfe850
            x1 = m.r64(m.env.data_va(77853)) as u128;
            pc = 0x76b6;
        }
        0x76b6 => {
            // block 76b6
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 76b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 76b6 xorps xmm0,xmm0
            x0 = 0;
            // 76b9 cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // 76bd divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 76c1 movsd xmm1,QWORD PTR [rbp-0x18]
            x1 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u128;
            // 76c6 call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 76cb movsd xmm1,QWORD PTR [rip+0xf717d] # 0xfe850
            x1 = m.r64(m.env.data_va(77853)) as u128;
            // 76d3 mulsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x1)).to_bits());
            // 76d7 movsd QWORD PTR [r14+rbx*8],xmm0
            m.w64(r14.wrapping_add(rbx.wrapping_mul(8)), x0 as u64);
            // 76dd inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 76e0 cmp rbx,0x400
            fl = sub::<64>(rbx, 0x400u64).1;
            // 76e7 jne 0x76b6
            if fl.ne() { pc = 0x76b6; continue; }
            pc = 0x76e9;
        }
        0x76e9 => {
            // block 76e9
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 76e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 76e9 jmp 0x7a15
            pc = 0x7a15; continue;
        }
        0x76ee => {
            // block 76ee
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 76ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 76ee xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 76f0 movsd xmm1,QWORD PTR [rip+0xf7158] # 0xfe850
            x1 = m.r64(m.env.data_va(77853)) as u128;
            // 76f8 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 76fa je 0x776c
            if fl.e() { pc = 0x776c; continue; }
            pc = 0x76fc;
        }
        0x76fc => {
            // block 76fc
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 76fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 76fc xorps xmm0,xmm0
            x0 = 0;
            // 76ff cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // 7703 divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 7707 movsd xmm1,QWORD PTR [rip+0xf7179] # 0xfe888
            x1 = m.r64(m.env.data_va(77909)) as u128;
            // 770f ucomisd xmm1,xmm0
            fl = ucomi(f64_of(x1), f64_of(x0));
            // 7713 jbe 0x771f
            if fl.be() { pc = 0x771f; continue; }
            pc = 0x7715;
        }
        0x7715 => {
            // block 7715
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7715 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7715 divsd xmm0,QWORD PTR [rip+0xf718b] # 0xfe8a8
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(m.env.data_va(77941)) as u128))).to_bits());
            // 771d jmp 0x773c
            pc = 0x773c; continue;
        }
        0x771f => {
            // block 771f
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 771f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 771f addsd xmm0,QWORD PTR [rip+0xf7169] # 0xfe890
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77917)) as u128))).to_bits());
            // 7727 divsd xmm0,QWORD PTR [rip+0xf7169] # 0xfe898
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(m.env.data_va(77925)) as u128))).to_bits());
            // 772f movsd xmm1,QWORD PTR [rip+0xf7169] # 0xfe8a0
            x1 = m.r64(m.env.data_va(77933)) as u128;
            // 7737 call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            pc = 0x773c;
        }
        0x773c => {
            // block 773c
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 773c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 773c movsd xmm1,QWORD PTR [rip+0xf716c] # 0xfe8b0
            x1 = m.r64(m.env.data_va(77949)) as u128;
            // 7744 call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 7749 movsd xmm1,QWORD PTR [rip+0xf70ff] # 0xfe850
            x1 = m.r64(m.env.data_va(77853)) as u128;
            // 7751 mulsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x1)).to_bits());
            // 7755 movsd QWORD PTR [r14+rbx*8],xmm0
            m.w64(r14.wrapping_add(rbx.wrapping_mul(8)), x0 as u64);
            // 775b inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 775e cmp rbx,0x400
            fl = sub::<64>(rbx, 0x400u64).1;
            // 7765 jne 0x76fc
            if fl.ne() { pc = 0x76fc; continue; }
            pc = 0x7767;
        }
        0x7767 => {
            // block 7767
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7767 jmp 0x7a15
            pc = 0x7a15; continue;
        }
        0x776c => {
            // block 776c
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 776c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 776c xorps xmm0,xmm0
            x0 = 0;
            // 776f cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // 7773 divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 7777 movsd xmm1,QWORD PTR [rip+0xf7139] # 0xfe8b8
            x1 = m.r64(m.env.data_va(77957)) as u128;
            // 777f call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 7784 movsd xmm1,QWORD PTR [rip+0xf7134] # 0xfe8c0
            x1 = m.r64(m.env.data_va(77965)) as u128;
            // 778c ucomisd xmm1,xmm0
            fl = ucomi(f64_of(x1), f64_of(x0));
            // 7790 jbe 0x779c
            if fl.be() { pc = 0x779c; continue; }
            pc = 0x7792;
        }
        0x7792 => {
            // block 7792
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7792 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7792 mulsd xmm0,QWORD PTR [rip+0xf710e] # 0xfe8a8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(77941)) as u128))).to_bits());
            // 779a jmp 0x77b9
            pc = 0x77b9; continue;
        }
        0x779c => {
            // block 779c
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 779c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 779c movsd xmm1,QWORD PTR [rip+0xf7124] # 0xfe8c8
            x1 = m.r64(m.env.data_va(77973)) as u128;
            // 77a4 call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 77a9 mulsd xmm0,QWORD PTR [rip+0xf70e7] # 0xfe898
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(77925)) as u128))).to_bits());
            // 77b1 addsd xmm0,QWORD PTR [rip+0xf7117] # 0xfe8d0
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77981)) as u128))).to_bits());
            pc = 0x77b9;
        }
        0x77b9 => {
            // block 77b9
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 77b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 77b9 movsd xmm1,QWORD PTR [rip+0xf708f] # 0xfe850
            x1 = m.r64(m.env.data_va(77853)) as u128;
            // 77c1 mulsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x1)).to_bits());
            // 77c5 movsd QWORD PTR [r14+rbx*8],xmm0
            m.w64(r14.wrapping_add(rbx.wrapping_mul(8)), x0 as u64);
            // 77cb inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 77ce cmp rbx,0x400
            fl = sub::<64>(rbx, 0x400u64).1;
            // 77d5 jne 0x776c
            if fl.ne() { pc = 0x776c; continue; }
            pc = 0x77d7;
        }
        0x77d7 => {
            // block 77d7
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 77d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 77d7 jmp 0x7a15
            pc = 0x7a15; continue;
        }
        0x77dc => {
            // block 77dc
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 77dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 77dc test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 77de sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 77e1 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 77e4 lea rcx,[rip+0xf7045] # 0xfe830
            rcx = m.env.data_va(77821);
            // 77eb movsd xmm0,QWORD PTR [rcx+rax*8]
            x0 = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u128;
            // 77f0 movsd QWORD PTR [rbp-0x18],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), x0 as u64);
            // 77f5 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 77f7 movsd xmm1,QWORD PTR [rip+0xf7081] # 0xfe880
            x1 = m.r64(m.env.data_va(77901)) as u128;
            pc = 0x77ff;
        }
        0x77ff => {
            // block 77ff
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 77ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 77ff lea eax,[rbx-0x41]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffbf)) as u32 as u64;
            // 7802 cmp eax,0x36a
            fl = sub::<32>((rax as u32 as u64), 0x36au64).1;
            // 7807 xorps xmm0,xmm0
            x0 = 0;
            // 780a cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // 780e ja 0x783a
            if fl.a() { pc = 0x783a; continue; }
            pc = 0x7810;
        }
        0x7810 => {
            // block 7810
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7810 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7810 addsd xmm0,QWORD PTR [rip+0xf7060] # 0xfe878
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77893)) as u128))).to_bits());
            // 7818 divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 781c movsd xmm1,QWORD PTR [rbp-0x18]
            x1 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u128;
            // 7821 call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 7826 movsd xmm1,QWORD PTR [rip+0xf7052] # 0xfe880
            x1 = m.r64(m.env.data_va(77901)) as u128;
            // 782e mulsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x1)).to_bits());
            // 7832 addsd xmm0,QWORD PTR [rip+0xf7036] # 0xfe870
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77885)) as u128))).to_bits());
            pc = 0x783a;
        }
        0x783a => {
            // block 783a
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 783a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 783a movsd QWORD PTR [r14+rbx*8],xmm0
            m.w64(r14.wrapping_add(rbx.wrapping_mul(8)), x0 as u64);
            // 7840 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7843 cmp rbx,0x400
            fl = sub::<64>(rbx, 0x400u64).1;
            // 784a jne 0x77ff
            if fl.ne() { pc = 0x77ff; continue; }
            pc = 0x784c;
        }
        0x784c => {
            // block 784c
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 784c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 784c jmp 0x7a15
            pc = 0x7a15; continue;
        }
        0x7851 => {
            // block 7851
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7851 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 7853 movsd xmm1,QWORD PTR [rip+0xf7025] # 0xfe880
            x1 = m.r64(m.env.data_va(77901)) as u128;
            // 785b test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 785d je 0x78f1
            if fl.e() { pc = 0x78f1; continue; }
            pc = 0x7863;
        }
        0x7863 => {
            // block 7863
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7863 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7863 xorps xmm0,xmm0
            x0 = 0;
            // 7866 cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // 786a lea eax,[rbx-0x41]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffbf)) as u32 as u64;
            // 786d cmp eax,0x36b
            fl = sub::<32>((rax as u32 as u64), 0x36bu64).1;
            // 7872 jae 0x78d6
            if fl.ae() { pc = 0x78d6; continue; }
            pc = 0x7874;
        }
        0x7874 => {
            // block 7874
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7874 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7874 addsd xmm0,QWORD PTR [rip+0xf6ffc] # 0xfe878
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77893)) as u128))).to_bits());
            // 787c divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 7880 movsd xmm1,QWORD PTR [rip+0xf7000] # 0xfe888
            x1 = m.r64(m.env.data_va(77909)) as u128;
            // 7888 ucomisd xmm1,xmm0
            fl = ucomi(f64_of(x1), f64_of(x0));
            // 788c jbe 0x7898
            if fl.be() { pc = 0x7898; continue; }
            pc = 0x788e;
        }
        0x788e => {
            // block 788e
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 788e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 788e divsd xmm0,QWORD PTR [rip+0xf7012] # 0xfe8a8
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(m.env.data_va(77941)) as u128))).to_bits());
            // 7896 jmp 0x78b5
            pc = 0x78b5; continue;
        }
        0x7898 => {
            // block 7898
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7898 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7898 addsd xmm0,QWORD PTR [rip+0xf6ff0] # 0xfe890
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77917)) as u128))).to_bits());
            // 78a0 divsd xmm0,QWORD PTR [rip+0xf6ff0] # 0xfe898
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(m.env.data_va(77925)) as u128))).to_bits());
            // 78a8 movsd xmm1,QWORD PTR [rip+0xf6ff0] # 0xfe8a0
            x1 = m.r64(m.env.data_va(77933)) as u128;
            // 78b0 call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            pc = 0x78b5;
        }
        0x78b5 => {
            // block 78b5
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 78b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 78b5 movsd xmm1,QWORD PTR [rip+0xf6ff3] # 0xfe8b0
            x1 = m.r64(m.env.data_va(77949)) as u128;
            // 78bd call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 78c2 movsd xmm1,QWORD PTR [rip+0xf6fb6] # 0xfe880
            x1 = m.r64(m.env.data_va(77901)) as u128;
            // 78ca mulsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x1)).to_bits());
            // 78ce addsd xmm0,QWORD PTR [rip+0xf6f9a] # 0xfe870
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77885)) as u128))).to_bits());
            pc = 0x78d6;
        }
        0x78d6 => {
            // block 78d6
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 78d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 78d6 movsd QWORD PTR [r14+rbx*8],xmm0
            m.w64(r14.wrapping_add(rbx.wrapping_mul(8)), x0 as u64);
            // 78dc inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 78df cmp rbx,0x400
            fl = sub::<64>(rbx, 0x400u64).1;
            // 78e6 jne 0x7863
            if fl.ne() { pc = 0x7863; continue; }
            pc = 0x78ec;
        }
        0x78ec => {
            // block 78ec
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 78ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 78ec jmp 0x7a15
            pc = 0x7a15; continue;
        }
        0x78f1 => {
            // block 78f1
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 78f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 78f1 xorps xmm0,xmm0
            x0 = 0;
            // 78f4 cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // 78f8 lea eax,[rbx-0x41]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffbf)) as u32 as u64;
            // 78fb cmp eax,0x36b
            fl = sub::<32>((rax as u32 as u64), 0x36bu64).1;
            // 7900 jae 0x7964
            if fl.ae() { pc = 0x7964; continue; }
            pc = 0x7902;
        }
        0x7902 => {
            // block 7902
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7902 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7902 addsd xmm0,QWORD PTR [rip+0xf6f6e] # 0xfe878
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77893)) as u128))).to_bits());
            // 790a divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 790e movsd xmm1,QWORD PTR [rip+0xf6fa2] # 0xfe8b8
            x1 = m.r64(m.env.data_va(77957)) as u128;
            // 7916 call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 791b movsd xmm1,QWORD PTR [rip+0xf6f9d] # 0xfe8c0
            x1 = m.r64(m.env.data_va(77965)) as u128;
            // 7923 ucomisd xmm1,xmm0
            fl = ucomi(f64_of(x1), f64_of(x0));
            // 7927 jbe 0x7933
            if fl.be() { pc = 0x7933; continue; }
            pc = 0x7929;
        }
        0x7929 => {
            // block 7929
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7929 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7929 mulsd xmm0,QWORD PTR [rip+0xf6f77] # 0xfe8a8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(77941)) as u128))).to_bits());
            // 7931 jmp 0x7950
            pc = 0x7950; continue;
        }
        0x7933 => {
            // block 7933
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7933 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7933 movsd xmm1,QWORD PTR [rip+0xf6f8d] # 0xfe8c8
            x1 = m.r64(m.env.data_va(77973)) as u128;
            // 793b call 0xec23c
            { let r = { let t = m.env.stub_va(0xa09); m.call(t, &[], &[x0 as u64, x1 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 7940 mulsd xmm0,QWORD PTR [rip+0xf6f50] # 0xfe898
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(77925)) as u128))).to_bits());
            // 7948 addsd xmm0,QWORD PTR [rip+0xf6f80] # 0xfe8d0
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77981)) as u128))).to_bits());
            pc = 0x7950;
        }
        0x7950 => {
            // block 7950
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7950 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7950 movsd xmm1,QWORD PTR [rip+0xf6f28] # 0xfe880
            x1 = m.r64(m.env.data_va(77901)) as u128;
            // 7958 mulsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x1)).to_bits());
            // 795c addsd xmm0,QWORD PTR [rip+0xf6f0c] # 0xfe870
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77885)) as u128))).to_bits());
            pc = 0x7964;
        }
        0x7964 => {
            // block 7964
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7964 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7964 movsd QWORD PTR [r14+rbx*8],xmm0
            m.w64(r14.wrapping_add(rbx.wrapping_mul(8)), x0 as u64);
            // 796a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 796d cmp rbx,0x400
            fl = sub::<64>(rbx, 0x400u64).1;
            // 7974 jne 0x78f1
            if fl.ne() { pc = 0x78f1; continue; }
            pc = 0x797a;
        }
        0x797a => {
            // block 797a
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 797a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 797a jmp 0x7a15
            pc = 0x7a15; continue;
        }
        0x797f => {
            // block 797f
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 797f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 797f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7981 cmp edx,0x1
            fl = sub::<32>((rdx as u32 as u64), 0x1u64).1;
            // 7984 jne 0x79b8
            if fl.ne() { pc = 0x79b8; continue; }
            pc = 0x7986;
        }
        0x7986 => {
            // block 7986
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7986 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7986 movsd xmm0,QWORD PTR [rip+0xf6eda] # 0xfe868
            x0 = m.r64(m.env.data_va(77877)) as u128;
            // 798e movsd xmm1,QWORD PTR [rip+0xf6eda] # 0xfe870
            x1 = m.r64(m.env.data_va(77885)) as u128;
            pc = 0x7996;
        }
        0x7996 => {
            // block 7996
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7996 xorps xmm2,xmm2
            x2 = 0;
            // 7999 cvtsi2sd xmm2,eax
            x2 = set_lo64(x2, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 799d mulsd xmm2,xmm0
            x2 = set_lo64(x2, (f64_of(x2) * f64_of(x0)).to_bits());
            // 79a1 addsd xmm2,xmm1
            x2 = set_lo64(x2, (f64_of(x2) + f64_of(x1)).to_bits());
            // 79a5 movsd QWORD PTR [r14+rax*8],xmm2
            m.w64(r14.wrapping_add(rax.wrapping_mul(8)), x2 as u64);
            // 79ab inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 79ae cmp rax,0x400
            fl = sub::<64>(rax, 0x400u64).1;
            // 79b4 jne 0x7996
            if fl.ne() { pc = 0x7996; continue; }
            pc = 0x79b6;
        }
        0x79b6 => {
            // block 79b6
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 79b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79b6 jmp 0x7a15
            pc = 0x7a15; continue;
        }
        0x79b8 => {
            // block 79b8
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 79b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79b8 mov esi,0x200
            rsi = (0x200u64) as u32 as u64;
            // 79bd mov rdi,r14
            rdi = r14;
            // 79c0 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 79c5 mov eax,0x40
            rax = (0x40u64) as u32 as u64;
            // 79ca movsd xmm0,QWORD PTR [rip+0xf6e86] # 0xfe858
            x0 = m.r64(m.env.data_va(77861)) as u128;
            // 79d2 movsd xmm1,QWORD PTR [rip+0xf6e86] # 0xfe860
            x1 = m.r64(m.env.data_va(77869)) as u128;
            pc = 0x79da;
        }
        0x79da => {
            // block 79da
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 79da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79da xorps xmm2,xmm2
            x2 = 0;
            // 79dd cvtsi2sd xmm2,eax
            x2 = set_lo64(x2, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 79e1 mulsd xmm2,xmm0
            x2 = set_lo64(x2, (f64_of(x2) * f64_of(x0)).to_bits());
            // 79e5 addsd xmm2,xmm1
            x2 = set_lo64(x2, (f64_of(x2) + f64_of(x1)).to_bits());
            // 79e9 movsd QWORD PTR [r14+rax*8],xmm2
            m.w64(r14.wrapping_add(rax.wrapping_mul(8)), x2 as u64);
            // 79ef inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 79f2 cmp rax,0x3ac
            fl = sub::<64>(rax, 0x3acu64).1;
            // 79f8 jne 0x79da
            if fl.ne() { pc = 0x79da; continue; }
            pc = 0x79fa;
        }
        0x79fa => {
            // block 79fa
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 79fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79fa add r14,0x1d60
            { let (r, f) = add::<64>(r14, 0x1d60u64);
              r14 = r;
              fl = f; }
            // 7a01 lea rsi,[rip+0xf80d8] # 0xffae0
            rsi = m.env.data_va(82605);
            // 7a08 mov edx,0x2a0
            rdx = (0x2a0u64) as u32 as u64;
            // 7a0d mov rdi,r14
            rdi = r14;
            // 7a10 call 0xec230
            { let r = { let t = m.env.stub_va(0x9fd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            pc = 0x7a15;
        }
        0x7a15 => {
            // block 7a15
            #[cfg(feature = "trace")] eprintln!("f_0000764e pc 7a15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a15 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 7a17 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 7a1b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7a1c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7a1e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7a1f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a46a size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001a46a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1a46a;
    loop {
        match pc {
        0x1a46a => {
            // block 1a46a
            #[cfg(feature = "trace")] eprintln!("f_0001a46a pc 1a46a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a46a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a46b mov rbp,rsp
            rbp = rsp;
            // 1a46e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1a470 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a472 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1a474 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1a476 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a477 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a47b mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1a47f mov r12,rdi
            r12 = rdi;
            // 1a482 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1a486 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a48b sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1a48e sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1a492 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1a495 mov rax,rbx
            rax = rbx;
            // 1a498 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1a49c je 0x1a4aa
            if fl.e() { pc = 0x1a4aa; continue; }
            pc = 0x1a49e;
        }
        0x1a49e => {
            // block 1a49e
            #[cfg(feature = "trace")] eprintln!("f_0001a46a pc 1a49e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a49e mov rdi,r12
            rdi = r12;
            // 1a4a1 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x1a4aa => {
            // block 1a4aa
            #[cfg(feature = "trace")] eprintln!("f_0001a46a pc 1a4aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a4aa lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1a4af mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1a4b4 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1a4b8 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1a4bb mov rcx,r15
            rcx = r15;
            // 1a4be sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1a4c2 movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1a4cc cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1a4cf ja 0x1a4dd
            if fl.a() { pc = 0x1a4dd; continue; }
            pc = 0x1a4d1;
        }
        0x1a4d1 => {
            // block 1a4d1
            #[cfg(feature = "trace")] eprintln!("f_0001a46a pc 1a4d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a4d1 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1a4d4 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1a4d7 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1a4db jmp 0x1a4e7
            pc = 0x1a4e7; continue;
        }
        0x1a4dd => {
            // block 1a4dd
            #[cfg(feature = "trace")] eprintln!("f_0001a46a pc 1a4dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a4dd movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1a4e7;
        }
        0x1a4e7 => {
            // block 1a4e7
            #[cfg(feature = "trace")] eprintln!("f_0001a46a pc 1a4e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a4e7 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a4ec mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1a4f0 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1a4f3 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1a4f7 mov r13,r14
            r13 = r14;
            // 1a4fa sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1a4fe mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1a506 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1a50a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1a50c test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1a50f je 0x1a521
            if fl.e() { pc = 0x1a521; continue; }
            pc = 0x1a511;
        }
        0x1a511 => {
            // block 1a511
            #[cfg(feature = "trace")] eprintln!("f_0001a46a pc 1a511 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a511 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1a519 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a51e mov rbx,rax
            rbx = rax;
            pc = 0x1a521;
        }
        0x1a521 => {
            // block 1a521
            #[cfg(feature = "trace")] eprintln!("f_0001a46a pc 1a521 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a521 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1a525 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1a529 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1a52d mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1a52f mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1a533 lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1a538 mov rdi,rbx
            rdi = rbx;
            // 1a53b mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1a53f mov rsi,r15
            rsi = r15;
            // 1a542 mov rdx,r14
            rdx = r14;
            // 1a545 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a54a mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1a54e mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1a552 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1a557 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1a55b mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1a55f mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1a563 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1a568 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1a56c mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1a570 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1a574 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1a578 call 0xeb978
            { let r = { let t = m.env.stub_va(0x145); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a57d add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a581 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a582 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1a584 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1a586 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a588 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a58a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a58b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
