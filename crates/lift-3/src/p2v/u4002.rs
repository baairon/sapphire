//! Mechanically lifted functions (scripts/lift_run.py), module u4002. Each one is checked against the original
//! in the world harness (tests/world_u4002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xd8250 size=216
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d8250<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd8250;
    loop {
        match pc {
        0xd8250 => {
            // block d8250
            #[cfg(feature = "trace")] eprintln!("f_000d8250 pc d8250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8250 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8251 mov rbp,rsp
            rbp = rsp;
            // d8254 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d8255 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d8256 mov rbx,rdi
            rbx = rdi;
            // d8259 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d825c mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8261 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8263 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8268 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d826b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d826e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8271 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8276 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d827b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8280 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8283 mov rdi,rbx
            rdi = rbx;
            // d8286 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8289 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d828c mov rdi,rbx
            rdi = rbx;
            // d828f call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8295 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8298 mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d829d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d829f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d82a4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d82a7 mov rdi,rbx
            rdi = rbx;
            // d82aa call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82ad mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82b0 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d82b5 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d82ba mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d82bf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d82c2 mov rdi,rbx
            rdi = rbx;
            // d82c5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82c8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82cb mov rdi,rbx
            rdi = rbx;
            // d82ce call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82d4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82d7 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d82dc mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d82e1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d82e6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d82e9 mov rdi,rbx
            rdi = rbx;
            // d82ec call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82ef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82f2 mov rdi,rbx
            rdi = rbx;
            // d82f5 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82fb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82fe mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8303 mov edx,0xc7
            rdx = (0xc7u64) as u32 as u64;
            // d8308 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d830d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8310 mov rdi,rbx
            rdi = rbx;
            // d8313 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8316 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8319 mov rdi,rbx
            rdi = rbx;
            // d831c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d8320 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d8321 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8322 jmp QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe6324 size=216
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e6324<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe6324;
    loop {
        match pc {
        0xe6324 => {
            // block e6324
            #[cfg(feature = "trace")] eprintln!("f_000e6324 pc e6324 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6324 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e6325 mov rbp,rsp
            rbp = rsp;
            // e6328 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e6329 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e632d mov rbx,QWORD PTR [rip+0x5a0a4] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e6334 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e6337 mov QWORD PTR [rbp-0x10],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (rbx) as u64);
            // e633b mov ecx,DWORD PTR [rdi+0x88]
            rcx = (m.r32(rdi.wrapping_add(0x88)) as u64) as u32 as u64;
            // e6341 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e6343 shr edx,0x1c
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x1cu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e6346 mov DWORD PTR [rdi+0x7c],edx
            m.w32(rdi.wrapping_add(0x7c), ((rdx as u32 as u64)) as u32);
            // e6349 lea rax,[rip+0x41ff0] # 0x128340
            rax = m.env.data_va(248589);
            // e6350 mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e6353 mov BYTE PTR [rbp-0x1c],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rdx as u8 as u64)) as u8);
            // e6356 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e6358 shr edx,0x14
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x14u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e635b and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e635e mov DWORD PTR [rdi+0x78],edx
            m.w32(rdi.wrapping_add(0x78), ((rdx as u32 as u64)) as u32);
            // e6361 mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e6364 mov BYTE PTR [rbp-0x1b],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe5), ((rdx as u8 as u64)) as u8);
            // e6367 mov BYTE PTR [rbp-0x1a],0x20
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), (0x20u64) as u8);
            // e636b mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e636d shr edx,0xc
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0xcu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e6370 and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e6373 mov DWORD PTR [rdi+0x74],edx
            m.w32(rdi.wrapping_add(0x74), ((rdx as u32 as u64)) as u32);
            // e6376 mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e6379 mov BYTE PTR [rbp-0x19],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe7), ((rdx as u8 as u64)) as u8);
            // e637c shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e637f and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6382 mov DWORD PTR [rdi+0x70],ecx
            m.w32(rdi.wrapping_add(0x70), ((rcx as u32 as u64)) as u32);
            // e6385 mov cl,BYTE PTR [rcx+rax*1]
            rcx = (rcx & !0xff) | ((m.r8(rcx.wrapping_add(rax)) as u64) as u8 as u64);
            // e6388 mov BYTE PTR [rbp-0x18],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), ((rcx as u8 as u64)) as u8);
            // e638b mov BYTE PTR [rbp-0x17],0x20
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe9), (0x20u64) as u8);
            // e638f mov ecx,DWORD PTR [rdi+0x84]
            rcx = (m.r32(rdi.wrapping_add(0x84)) as u64) as u32 as u64;
            // e6395 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e6397 shr edx,0x1c
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x1cu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e639a mov DWORD PTR [rdi+0x6c],edx
            m.w32(rdi.wrapping_add(0x6c), ((rdx as u32 as u64)) as u32);
            // e639d mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e63a0 mov BYTE PTR [rbp-0x16],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffea), ((rdx as u8 as u64)) as u8);
            // e63a3 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e63a5 shr edx,0x14
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x14u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e63a8 and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e63ab mov DWORD PTR [rdi+0x68],edx
            m.w32(rdi.wrapping_add(0x68), ((rdx as u32 as u64)) as u32);
            // e63ae mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e63b1 mov BYTE PTR [rbp-0x15],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffeb), ((rdx as u8 as u64)) as u8);
            // e63b4 mov BYTE PTR [rbp-0x14],0x20
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x20u64) as u8);
            // e63b8 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e63ba shr edx,0xc
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0xcu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e63bd and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e63c0 mov DWORD PTR [rdi+0x64],edx
            m.w32(rdi.wrapping_add(0x64), ((rdx as u32 as u64)) as u32);
            // e63c3 mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e63c6 mov BYTE PTR [rbp-0x13],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffed), ((rdx as u8 as u64)) as u8);
            // e63c9 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e63cc and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e63cf mov DWORD PTR [rdi+0x60],ecx
            m.w32(rdi.wrapping_add(0x60), ((rcx as u32 as u64)) as u32);
            // e63d2 mov al,BYTE PTR [rcx+rax*1]
            rax = (rax & !0xff) | ((m.r8(rcx.wrapping_add(rax)) as u64) as u8 as u64);
            // e63d5 mov BYTE PTR [rbp-0x12],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffee), ((rax as u8 as u64)) as u8);
            // e63d8 mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // e63dc add rdi,0x38
            { let (r, f) = add::<64>(rdi, 0x38u64);
              rdi = r;
              fl = f; }
            // e63e0 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e63e4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e63e9 cmp rbx,QWORD PTR [rbp-0x10]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64).1;
            // e63ed jne 0xe63f6
            if fl.ne() { pc = 0xe63f6; continue; }
            pc = 0xe63ef;
        }
        0xe63ef => {
            // block e63ef
            #[cfg(feature = "trace")] eprintln!("f_000e6324 pc e63ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e63ef add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e63f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e63f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e63f5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe63f6 => {
            // block e63f6
            #[cfg(feature = "trace")] eprintln!("f_000e6324 pc e63f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e63f6 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x426f0 size=218
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000426f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x426f0;
    loop {
        match pc {
        0x426f0 => {
            // block 426f0
            #[cfg(feature = "trace")] eprintln!("f_000426f0 pc 426f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 426f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 426f1 mov rbp,rsp
            rbp = rsp;
            // 426f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 426f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 426f8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 426fa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 426fb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 426ff mov r14,rdx
            r14 = rdx;
            // 42702 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 42704 mov r12,rdi
            r12 = rdi;
            // 42707 mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // 4270e mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 42713 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42718 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 4271b cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // 4271d jbe 0x427bb
            if fl.be() { pc = 0x427bb; continue; }
            pc = 0x42723;
        }
        0x42723 => {
            // block 42723
            #[cfg(feature = "trace")] eprintln!("f_000426f0 pc 42723 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42723 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 4272a mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            // 42731 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 42738 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 4273a lea rcx,[rip+0xbdd5f] # 0x1004a0
            rcx = m.env.data_va(85101);
            // 42741 mov r15d,DWORD PTR [rcx+rax*4]
            r15 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 42745 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 42749 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 4274d mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 42752 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 42758 mov rdi,r12
            rdi = r12;
            // 4275b mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 4275e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42761 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42763 je 0x427bb
            if fl.e() { pc = 0x427bb; continue; }
            pc = 0x42765;
        }
        0x42765 => {
            // block 42765
            #[cfg(feature = "trace")] eprintln!("f_000426f0 pc 42765 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42765 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 42769 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 4276d mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // 42772 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // 42778 mov rdi,r12
            rdi = r12;
            // 4277b mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 4277e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42781 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42783 je 0x427bb
            if fl.e() { pc = 0x427bb; continue; }
            pc = 0x42785;
        }
        0x42785 => {
            // block 42785
            #[cfg(feature = "trace")] eprintln!("f_000426f0 pc 42785 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42785 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 42789 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 4278d mov ecx,0x40000000
            rcx = (0x40000000u64) as u32 as u64;
            // 42792 mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            // 42798 mov rdi,r12
            rdi = r12;
            // 4279b mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 4279e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 427a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 427a3 je 0x427bb
            if fl.e() { pc = 0x427bb; continue; }
            pc = 0x427a5;
        }
        0x427a5 => {
            // block 427a5
            #[cfg(feature = "trace")] eprintln!("f_000426f0 pc 427a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 427a5 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 427a8 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 427ab mov ecx,DWORD PTR [rbp-0x28]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 427ae lea eax,[rax+rcx*2]
            rax = (rax.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // 427b1 add eax,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 427b4 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 427b7 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 427b9 jmp 0x427bd
            pc = 0x427bd; continue;
        }
        0x427bb => {
            // block 427bb
            #[cfg(feature = "trace")] eprintln!("f_000426f0 pc 427bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 427bb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x427bd;
        }
        0x427bd => {
            // block 427bd
            #[cfg(feature = "trace")] eprintln!("f_000426f0 pc 427bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 427bd add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 427c1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 427c2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 427c4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 427c6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 427c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 427c9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x428b4 size=218
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000428b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x428b4;
    loop {
        match pc {
        0x428b4 => {
            // block 428b4
            #[cfg(feature = "trace")] eprintln!("f_000428b4 pc 428b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 428b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 428b5 mov rbp,rsp
            rbp = rsp;
            // 428b8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 428ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 428bc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 428be push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 428bf sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 428c3 mov r14,rdx
            r14 = rdx;
            // 428c6 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 428c8 mov r12,rdi
            r12 = rdi;
            // 428cb mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // 428d2 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 428d7 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 428dc movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 428df cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // 428e1 jbe 0x4297f
            if fl.be() { pc = 0x4297f; continue; }
            pc = 0x428e7;
        }
        0x428e7 => {
            // block 428e7
            #[cfg(feature = "trace")] eprintln!("f_000428b4 pc 428e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 428e7 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 428ee mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            // 428f5 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 428fc mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 428fe lea rcx,[rip+0xbdb9b] # 0x1004a0
            rcx = m.env.data_va(85101);
            // 42905 mov r15d,DWORD PTR [rcx+rax*4]
            r15 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 42909 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4290d lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 42911 mov ecx,0x80000
            rcx = (0x80000u64) as u32 as u64;
            // 42916 mov r8d,0x13
            r8 = (0x13u64) as u32 as u64;
            // 4291c mov rdi,r12
            rdi = r12;
            // 4291f mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 42922 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42925 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42927 je 0x4297f
            if fl.e() { pc = 0x4297f; continue; }
            pc = 0x42929;
        }
        0x42929 => {
            // block 42929
            #[cfg(feature = "trace")] eprintln!("f_000428b4 pc 42929 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42929 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4292d lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 42931 mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 42936 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 4293c mov rdi,r12
            rdi = r12;
            // 4293f mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 42942 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42945 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42947 je 0x4297f
            if fl.e() { pc = 0x4297f; continue; }
            pc = 0x42949;
        }
        0x42949 => {
            // block 42949
            #[cfg(feature = "trace")] eprintln!("f_000428b4 pc 42949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42949 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4294d lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 42951 mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 42956 mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 4295c mov rdi,r12
            rdi = r12;
            // 4295f mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 42962 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42965 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42967 je 0x4297f
            if fl.e() { pc = 0x4297f; continue; }
            pc = 0x42969;
        }
        0x42969 => {
            // block 42969
            #[cfg(feature = "trace")] eprintln!("f_000428b4 pc 42969 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42969 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 4296c shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 4296f mov ecx,DWORD PTR [rbp-0x28]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 42972 lea eax,[rax+rcx*2]
            rax = (rax.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // 42975 add eax,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 42978 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 4297b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4297d jmp 0x42981
            pc = 0x42981; continue;
        }
        0x4297f => {
            // block 4297f
            #[cfg(feature = "trace")] eprintln!("f_000428b4 pc 4297f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4297f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42981;
        }
        0x42981 => {
            // block 42981
            #[cfg(feature = "trace")] eprintln!("f_000428b4 pc 42981 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42981 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 42985 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42986 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 42988 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4298a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4298c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4298d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43426 size=218
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043426<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43426;
    loop {
        match pc {
        0x43426 => {
            // block 43426
            #[cfg(feature = "trace")] eprintln!("f_00043426 pc 43426 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43426 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43427 mov rbp,rsp
            rbp = rsp;
            // 4342a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4342c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4342e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 43430 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43431 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 43434 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 43437 mov rbx,rdi
            rbx = rdi;
            // 4343a cmp r14d,0x3f
            fl = sub::<32>((r14 as u32 as u64), 0x3fu64).1;
            // 4343e ja 0x434cf
            if fl.a() { pc = 0x434cf; continue; }
            pc = 0x43444;
        }
        0x43444 => {
            // block 43444
            #[cfg(feature = "trace")] eprintln!("f_00043426 pc 43444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43444 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 43447 mov rdi,rbx
            rdi = rbx;
            // 4344a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43450 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 43452 call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43457 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 4345a jb 0x434d3
            if fl.b() { pc = 0x434d3; continue; }
            pc = 0x4345c;
        }
        0x4345c => {
            // block 4345c
            #[cfg(feature = "trace")] eprintln!("f_00043426 pc 4345c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4345c cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // 43460 setg al
            rax = (rax & !0xff) | (((fl.g() as u64)) as u8 as u64);
            // 43463 movzx r12d,al
            r12 = ((rax as u8 as u64)) as u32 as u64;
            // 43467 and r14d,0x3
            { let r = (r14 as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 4346b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4346e mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // 43473 mov ecx,0xf00000
            rcx = (0xf00000u64) as u32 as u64;
            // 43478 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 4347e mov rdi,rbx
            rdi = rbx;
            // 43481 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 43484 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43487 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4348a mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // 4348f mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // 43494 mov r8d,0x5
            r8 = (0x5u64) as u32 as u64;
            // 4349a mov rdi,rbx
            rdi = rbx;
            // 4349d mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 434a0 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 434a3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 434a6 mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // 434ab mov ecx,0x60000000
            rcx = (0x60000000u64) as u32 as u64;
            // 434b0 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 434b6 mov rdi,rbx
            rdi = rbx;
            // 434b9 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 434bc call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 434bf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 434c2 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 434c4 mov rdi,rbx
            rdi = rbx;
            // 434c7 call QWORD PTR [rax+0x1350]
            { let r = { let t = m.r64(rax.wrapping_add(0x1350)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 434cd jmp 0x434f6
            pc = 0x434f6; continue;
        }
        0x434cf => {
            // block 434cf
            #[cfg(feature = "trace")] eprintln!("f_00043426 pc 434cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 434cf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 434d1 jmp 0x434f6
            pc = 0x434f6; continue;
        }
        0x434d3 => {
            // block 434d3
            #[cfg(feature = "trace")] eprintln!("f_00043426 pc 434d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 434d3 shl r15d,0x4
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x4u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 434d7 or r15d,r14d
            { let r = (r15 as u32 as u64) | (r14 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 434da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 434dd mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 434e2 mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // 434e7 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 434ed mov rdi,rbx
            rdi = rbx;
            // 434f0 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 434f3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x434f6;
        }
        0x434f6 => {
            // block 434f6
            #[cfg(feature = "trace")] eprintln!("f_00043426 pc 434f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 434f6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 434f7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 434f9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 434fb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 434fd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 434fe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x523cc size=218
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000523cc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x523cc;
    loop {
        match pc {
        0x523cc => {
            // block 523cc
            #[cfg(feature = "trace")] eprintln!("f_000523cc pc 523cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 523cc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 523cd mov rbp,rsp
            rbp = rsp;
            // 523d0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 523d2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 523d3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 523d7 mov QWORD PTR [rip+0x175eb2],rdi # 0x1c8290
            m.w64(m.env.data_va(903773), (rdi) as u64);
            // 523de mov rdi,QWORD PTR [rip+0xedd9b] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 523e5 lea rsi,[rip+0xd8bfa] # 0x12afe6
            rsi = m.env.data_va(260019);
            // 523ec mov edx,0x2b
            rdx = (0x2bu64) as u32 as u64;
            // 523f1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 523f6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 523f9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 523fd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 52401 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 52404 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 52407 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 5240b mov rsi,QWORD PTR [rip+0x175e7e] # 0x1c8290
            rsi = m.r64(m.env.data_va(903773)) as u64;
            // 52412 mov rdi,rax
            rdi = rax;
            // 52415 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5241a mov rbx,rax
            rbx = rax;
            // 5241d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52420 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 52424 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 52428 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5242b or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5242e mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 52432 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52435 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 52439 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 5243c lea r14,[rbp-0x18]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 52440 mov rdi,r14
            rdi = r14;
            // 52443 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52448 mov rsi,QWORD PTR [rip+0xedd41] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 5244f mov rdi,r14
            rdi = r14;
            // 52452 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52457 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5245a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 5245e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 52463 mov rdi,rax
            rdi = rax;
            // 52466 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52468 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 5246b lea rdi,[rbp-0x18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 5246f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52474 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // 52478 mov rdi,rbx
            rdi = rbx;
            // 5247b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52480 mov rdi,rbx
            rdi = rbx;
            // 52483 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52488 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5248c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5248d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5248f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 52490 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x52491 => {
            // block 52491
            #[cfg(feature = "trace")] eprintln!("f_000523cc pc 52491 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52491 mov rbx,rax
            rbx = rax;
            // 52494 lea rdi,[rbp-0x18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 52498 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5249d mov rdi,rbx
            rdi = rbx;
            // 524a0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6424a size=218
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006424a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6424a;
    loop {
        match pc {
        0x6424a => {
            // block 6424a
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 6424a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6424a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6424b mov rbp,rsp
            rbp = rsp;
            // 6424e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6424f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 64250 mov rbx,rdi
            rbx = rdi;
            // 64253 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 64255 cmp esi,0x4
            fl = sub::<32>((rsi as u32 as u64), 0x4u64).1;
            // 64258 jbe 0x6426b
            if fl.be() { pc = 0x6426b; continue; }
            pc = 0x6425a;
        }
        0x6425a => {
            // block 6425a
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 6425a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6425a lea rsi,[rip+0xcae0b] # 0x12f06c
            rsi = m.env.data_va(276537);
            // 64261 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 64266 jmp 0x642fd
            pc = 0x642fd; continue;
        }
        0x6426b => {
            // block 6426b
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 6426b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6426b lea rcx,[rip+0x9e] # 0x64310
            rcx = m.env.code_va(0x64310);
            // 64272 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 64276 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 64279 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x6427b => {
            // block 6427b
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 6427b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6427b lea rax,[rip+0xcd756] # 0x1319d8
            rax = m.env.data_va(287141);
            // 64282 lea rsi,[rip+0xcd753] # 0x1319dc
            rsi = m.env.data_va(287145);
            // 64289 jmp 0x642c9
            pc = 0x642c9; continue;
        }
        0x6428b => {
            // block 6428b
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 6428b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6428b lea rax,[rip+0xcd762] # 0x1319f4
            rax = m.env.data_va(287169);
            // 64292 lea rsi,[rip+0xcd761] # 0x1319fa
            rsi = m.env.data_va(287175);
            // 64299 jmp 0x642a9
            pc = 0x642a9; continue;
        }
        0x6429b => {
            // block 6429b
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 6429b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6429b lea rax,[rip+0xcd775] # 0x131a17
            rax = m.env.data_va(287204);
            // 642a2 lea rsi,[rip+0xcd774] # 0x131a1d
            rsi = m.env.data_va(287210);
            pc = 0x642a9;
        }
        0x642a9 => {
            // block 642a9
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 642a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 642a9 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 642ab cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 642af mov eax,0x5
            rax = (0x5u64) as u32 as u64;
            // 642b4 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 642b9 jmp 0x642f9
            pc = 0x642f9; continue;
        }
        0x642bb => {
            // block 642bb
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 642bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 642bb lea rax,[rip+0xcd778] # 0x131a3a
            rax = m.env.data_va(287239);
            // 642c2 lea rsi,[rip+0xcd775] # 0x131a3e
            rsi = m.env.data_va(287243);
            pc = 0x642c9;
        }
        0x642c9 => {
            // block 642c9
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 642c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 642c9 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 642cb cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 642cf mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 642d4 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 642d9 jmp 0x642f9
            pc = 0x642f9; continue;
        }
        0x642db => {
            // block 642db
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 642db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 642db lea rax,[rip+0xc6378] # 0x12a65a
            rax = m.env.data_va(257575);
            // 642e2 lea rsi,[rip+0xcd76d] # 0x131a56
            rsi = m.env.data_va(287267);
            // 642e9 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 642eb cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 642ef mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 642f4 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            pc = 0x642f9;
        }
        0x642f9 => {
            // block 642f9
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 642f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 642f9 cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            pc = 0x642fd;
        }
        0x642fd => {
            // block 642fd
            #[cfg(feature = "trace")] eprintln!("f_0006424a pc 642fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 642fd mov rdi,rbx
            rdi = rbx;
            // 64300 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64305 mov rax,rbx
            rax = rbx;
            // 64308 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6430c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6430d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6430e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe966 size=220
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e966<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe966;
    loop {
        match pc {
        0xe966 => {
            // block e966
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc e966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e966 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e967 mov rbp,rsp
            rbp = rsp;
            // e96a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e96c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e96e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e970 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e971 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e975 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // e978 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e97a mov r12,rdi
            r12 = rdi;
            // e97d mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // e982 call 0xd2f1a
            { let r = { let t = m.env.code_va(0xd2f1a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e987 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e989 je 0xea30
            if fl.e() { pc = 0xea30; continue; }
            pc = 0xe98f;
        }
        0xe98f => {
            // block e98f
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc e98f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98f test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // e992 je 0xe9e3
            if fl.e() { pc = 0xe9e3; continue; }
            pc = 0xe994;
        }
        0xe994 => {
            // block e994
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc e994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e994 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // e99b mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e99f mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // e9a1 lea rdx,[rip+0xf0dac] # 0xff754
            rdx = m.env.data_va(81697);
            // e9a8 mov r14d,DWORD PTR [rdx+rcx*4]
            r14 = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e9ac lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // e9b0 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e9b2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e9b7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e9ba mov rdi,r12
            rdi = r12;
            // e9bd mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // e9c0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e9c5 je 0xea32
            if fl.e() { pc = 0xea32; continue; }
            pc = 0xe9c7;
        }
        0xe9c7 => {
            // block e9c7
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc e9c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c7 mov edx,DWORD PTR [rbp-0x24]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // e9ca mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e9cc and eax,0xffffdfff
            { let r = (rax as u32 as u64) & 0xffffdfffu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9d1 mov DWORD PTR [rbp-0x24],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rax as u32 as u64)) as u32);
            // e9d4 cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // e9d8 jne 0xea08
            if fl.ne() { pc = 0xea08; continue; }
            pc = 0xe9da;
        }
        0xe9da => {
            // block e9da
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc e9da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9da or eax,0x1000
            { let r = (rax as u32 as u64) | 0x1000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9df mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // e9e1 jmp 0xea14
            pc = 0xea14; continue;
        }
        0xe9e3 => {
            // block e9e3
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc e9e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9e3 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e9e7 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // e9e9 lea rdx,[rip+0xf0d64] # 0xff754
            rdx = m.env.data_va(81697);
            // e9f0 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e9f3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e9f8 mov ecx,0x2000
            rcx = (0x2000u64) as u32 as u64;
            // e9fd mov r8d,0xd
            r8 = (0xdu64) as u32 as u64;
            // ea03 mov rdi,r12
            rdi = r12;
            // ea06 jmp 0xea29
            pc = 0xea29; continue;
        }
        0xea08 => {
            // block ea08
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc ea08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea08 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // ea0c jne 0xea30
            if fl.ne() { pc = 0xea30; continue; }
            pc = 0xea0e;
        }
        0xea0e => {
            // block ea0e
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc ea0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0e and edx,0xffffcfff
            { let r = (rdx as u32 as u64) & 0xffffcfffu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xea14;
        }
        0xea14 => {
            // block ea14
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc ea14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea14 mov DWORD PTR [rbp-0x24],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rdx as u32 as u64)) as u32);
            // ea17 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ea1b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ea20 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ea23 mov rdi,r12
            rdi = r12;
            // ea26 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0xea29;
        }
        0xea29 => {
            // block ea29
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc ea29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea29 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea2c mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // ea2e jmp 0xea32
            pc = 0xea32; continue;
        }
        0xea30 => {
            // block ea30
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc ea30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea30 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xea32;
        }
        0xea32 => {
            // block ea32
            #[cfg(feature = "trace")] eprintln!("f_0000e966 pc ea32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea32 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // ea34 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ea38 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea39 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ea3b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ea3d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ea3f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea40 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bd34 size=220
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bd34<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2bd34;
    loop {
        match pc {
        0x2bd34 => {
            // block 2bd34
            #[cfg(feature = "trace")] eprintln!("f_0002bd34 pc 2bd34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bd34 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bd35 mov rbp,rsp
            rbp = rsp;
            // 2bd38 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bd3a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bd3b mov r14,rsi
            r14 = rsi;
            // 2bd3e mov rbx,rdi
            rbx = rdi;
            // 2bd41 call 0x26172
            { let r = { let t = m.env.code_va(0x26172); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bd46 lea rsi,[rip+0xfebb7] # 0x12a904
            rsi = m.env.data_va(258257);
            // 2bd4d mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 2bd52 mov rdi,rax
            rdi = rax;
            // 2bd55 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bd5a mov esi,DWORD PTR [rbx+0x20]
            rsi = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2bd5d mov rdi,rax
            rdi = rax;
            // 2bd60 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bd65 lea rsi,[rip+0xfeba3] # 0x12a90f
            rsi = m.env.data_va(258268);
            // 2bd6c mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 2bd71 mov rdi,rax
            rdi = rax;
            // 2bd74 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bd79 lea rdi,[rbx+0x24]
            rdi = rbx.wrapping_add(0x24);
            // 2bd7d mov rsi,rax
            rsi = rax;
            // 2bd80 call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bd85 lea rsi,[rip+0xfeb8d] # 0x12a919
            rsi = m.env.data_va(258278);
            // 2bd8c mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 2bd91 mov rdi,rax
            rdi = rax;
            // 2bd94 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bd99 mov esi,DWORD PTR [rbx+0x4c]
            rsi = (m.r32(rbx.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2bd9c mov rdi,rax
            rdi = rax;
            // 2bd9f call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bda4 lea rsi,[rip+0xfeb80] # 0x12a92b
            rsi = m.env.data_va(258296);
            // 2bdab mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 2bdb0 mov rdi,rax
            rdi = rax;
            // 2bdb3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bdb8 lea rdi,[rbx+0x50]
            rdi = rbx.wrapping_add(0x50);
            // 2bdbc mov rsi,rax
            rsi = rax;
            // 2bdbf call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bdc4 lea rsi,[rip+0xfeb6f] # 0x12a93a
            rsi = m.env.data_va(258311);
            // 2bdcb mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 2bdd0 mov rdi,rax
            rdi = rax;
            // 2bdd3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bdd8 lea rdi,[rbx+0x78]
            rdi = rbx.wrapping_add(0x78);
            // 2bddc mov rsi,rax
            rsi = rax;
            // 2bddf call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bde4 lea rsi,[rip+0xfe6b2] # 0x12a49d
            rsi = m.env.data_va(257130);
            // 2bdeb mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 2bdf0 mov rdi,rax
            rdi = rax;
            // 2bdf3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bdf8 lea rsi,[rbx+0xa0]
            rsi = rbx.wrapping_add(0xa0);
            // 2bdff mov rdi,rax
            rdi = rax;
            // 2be02 call 0x26804
            { let r = { let t = m.env.code_va(0x26804); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2be07 mov rax,r14
            rax = r14;
            // 2be0a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2be0b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2be0d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2be0e ret
            return Ret { rax, rdx, xmm0: x0 };
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
