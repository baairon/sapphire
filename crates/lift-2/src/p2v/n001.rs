//! Mechanically lifted functions (scripts/lift_run.py), module n001. Each one is checked against the original
//! in the world harness (tests/world_n001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x5142 size=906
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00005142<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5142;
    loop {
        match pc {
        0x5142 => {
            // block 5142
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5142 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5142 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5143 mov rbp,rsp
            rbp = rsp;
            // 5146 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5148 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 514a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 514c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 514e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 514f sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 5153 mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // 5156 mov DWORD PTR [rbp-0x44],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((r14 as u32 as u64)) as u32);
            // 515a mov DWORD PTR [rbp-0x48],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((rcx as u32 as u64)) as u32);
            // 515d mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 5160 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5163 mov r13,rdi
            r13 = rdi;
            // 5166 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 516a call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5170 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 5172 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5174 mov esi,0x0
            rsi = (0x0u64) as u32 as u64;
            // 5179 cmovne esi,r14d
            if fl.ne() { rsi = ((r14 as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // 517d mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 5181 mov rdi,r13
            rdi = r13;
            // 5184 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 518a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 518c jne 0x519f
            if fl.ne() { pc = 0x519f; continue; }
            pc = 0x518e;
        }
        0x518e => {
            // block 518e
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 518e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 518e cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 5192 ja 0x519d
            if fl.a() { pc = 0x519d; continue; }
            pc = 0x5194;
        }
        0x5194 => {
            // block 5194
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5194 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 5197 cmp r12d,0xb
            fl = sub::<32>((r12 as u32 as u64), 0xbu64).1;
            // 519b jbe 0x51b0
            if fl.be() { pc = 0x51b0; continue; }
            pc = 0x519d;
        }
        0x519d => {
            // block 519d
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 519d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 519d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x519f;
        }
        0x519f => {
            // block 519f
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 519f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 519f mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 51a1 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 51a5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 51a6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 51a8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 51aa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 51ac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 51ae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51af ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x51b0 => {
            // block 51b0
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 51b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51b0 lea rcx,[rip+0x2e5] # 0x549c
            rcx = m.env.code_va(0x549c);
            // 51b7 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 51bb add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 51be jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x51c0 => {
            // block 51c0
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 51c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51c0 lea eax,[r15-0x1]
            rax = (r15.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 51c4 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 51c7 ja 0x523a
            if fl.a() { pc = 0x523a; continue; }
            pc = 0x51c9;
        }
        0x51c9 => {
            // block 51c9
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 51c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51c9 mov rdi,QWORD PTR [rip+0x13afb0] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 51d0 lea rsi,[rip+0x124d47] # 0x129f1e
            rsi = m.env.data_va(255723);
            // 51d7 mov edx,0x84
            rdx = (0x84u64) as u32 as u64;
            // 51dc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51e1 mov rbx,rax
            rbx = rax;
            // 51e4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51e7 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51eb add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 51ee lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 51f2 mov rdi,r12
            rdi = r12;
            // 51f5 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51fa mov rsi,QWORD PTR [rip+0x13af8f] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 5201 mov rdi,r12
            rdi = r12;
            // 5204 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5209 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 520c mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 5210 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 5215 mov rdi,rax
            rdi = rax;
            // 5218 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 521a mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 521d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 5221 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5226 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 522a mov rdi,rbx
            rdi = rbx;
            // 522d call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5232 mov rdi,rbx
            rdi = rbx;
            // 5235 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x523a;
        }
        0x523a => {
            // block 523a
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 523a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 523a mov esi,DWORD PTR [rbp-0x48]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 523d jmp 0x5427
            pc = 0x5427; continue;
        }
        0x5242 => {
            // block 5242
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5242 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5242 mov esi,DWORD PTR [rbp-0x48]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 5245 cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // 5248 ja 0x5275
            if fl.a() { pc = 0x5275; continue; }
            pc = 0x524a;
        }
        0x524a => {
            // block 524a
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 524a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 524a mov eax,0x111
            rax = (0x111u64) as u32 as u64;
            // 524f bt eax,esi
            fl.cf = bit::<32>((rax as u32 as u64), (rsi as u32 as u64));
            // 5252 jae 0x5275
            if fl.ae() { pc = 0x5275; continue; }
            pc = 0x5254;
        }
        0x5254 => {
            // block 5254
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5254 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5254 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 5259 cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 525d je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x5263;
        }
        0x5263 => {
            // block 5263
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5263 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5263 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 5267 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 526a movzx esi,al
            rsi = ((rax as u8 as u64)) as u32 as u64;
            // 526d shl esi,0x3
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x3u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 5270 jmp 0x5427
            pc = 0x5427; continue;
        }
        0x5275 => {
            // block 5275
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5275 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5275 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 5277 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 527a cmp eax,0x10
            fl = sub::<32>((rax as u32 as u64), 0x10u64).1;
            // 527d je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x5283;
        }
        0x5283 => {
            // block 5283
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5283 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5283 cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // 5286 ja 0x5427
            if fl.a() { pc = 0x5427; continue; }
            pc = 0x528c;
        }
        0x528c => {
            // block 528c
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 528c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 528c mov eax,0x2420
            rax = (0x2420u64) as u32 as u64;
            // 5291 bt eax,esi
            fl.cf = bit::<32>((rax as u32 as u64), (rsi as u32 as u64));
            // 5294 jae 0x5427
            if fl.ae() { pc = 0x5427; continue; }
            pc = 0x529a;
        }
        0x529a => {
            // block 529a
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 529a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 529a mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 529f cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 52a3 je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x52a9;
        }
        0x52a9 => {
            // block 52a9
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 52a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52a9 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 52ad sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 52b0 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 52b3 lea esi,[rax+rax*2+0xa]
            rsi = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xa)) as u32 as u64;
            // 52b7 jmp 0x5427
            pc = 0x5427; continue;
        }
        0x52bc => {
            // block 52bc
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 52bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52bc mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 52c1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 52c4 jle 0x5427
            if fl.le() { pc = 0x5427; continue; }
            pc = 0x52ca;
        }
        0x52ca => {
            // block 52ca
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 52ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52ca cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 52ce setl al
            rax = (rax & !0xff) | (((fl.l() as u64)) as u8 as u64);
            // 52d1 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 52d4 lea esi,[rax*8+0x1]
            rsi = (rax.wrapping_mul(8).wrapping_add(0x1)) as u32 as u64;
            // 52db cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 52df jne 0x5427
            if fl.ne() { pc = 0x5427; continue; }
            pc = 0x52e5;
        }
        0x52e5 => {
            // block 52e5
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 52e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52e5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 52e8 mov rdi,QWORD PTR [rip+0x13ae91] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 52ef lea rsi,[rip+0x124b34] # 0x129e2a
            rsi = m.env.data_va(255479);
            // 52f6 mov edx,0x7a
            rdx = (0x7au64) as u32 as u64;
            // 52fb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5300 mov rbx,rax
            rbx = rax;
            // 5303 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5306 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 530a add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 530d lea r12,[rbp-0x40]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5311 mov rdi,r12
            rdi = r12;
            // 5314 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5319 mov rsi,QWORD PTR [rip+0x13ae70] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 5320 mov rdi,r12
            rdi = r12;
            // 5323 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5328 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 532b mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 532f mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 5334 mov rdi,rax
            rdi = rax;
            // 5337 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5339 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 533c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5340 jmp 0x540b
            pc = 0x540b; continue;
        }
        0x5345 => {
            // block 5345
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5345 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5345 mov esi,0xe
            rsi = (0xeu64) as u32 as u64;
            // 534a cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 534e je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x5354;
        }
        0x5354 => {
            // block 5354
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5354 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5354 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 5358 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 535b movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 535e lea esi,[rax*4+0x2]
            rsi = (rax.wrapping_mul(4).wrapping_add(0x2)) as u32 as u64;
            // 5365 jmp 0x5427
            pc = 0x5427; continue;
        }
        0x536a => {
            // block 536a
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 536a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 536a mov esi,0xf
            rsi = (0xfu64) as u32 as u64;
            // 536f cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 5373 je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x5379;
        }
        0x5379 => {
            // block 5379
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5379 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 537d sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5380 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 5383 lea esi,[rax*4+0x3]
            rsi = (rax.wrapping_mul(4).wrapping_add(0x3)) as u32 as u64;
            // 538a jmp 0x5427
            pc = 0x5427; continue;
        }
        0x538f => {
            // block 538f
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 538f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 538f mov esi,0xb
            rsi = (0xbu64) as u32 as u64;
            // 5394 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 5397 jle 0x5427
            if fl.le() { pc = 0x5427; continue; }
            pc = 0x539d;
        }
        0x539d => {
            // block 539d
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 539d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 539d cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 53a1 setl al
            rax = (rax & !0xff) | (((fl.l() as u64)) as u8 as u64);
            // 53a4 movzx esi,al
            rsi = ((rax as u8 as u64)) as u32 as u64;
            // 53a7 add esi,0xb
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xbu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 53aa cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 53ae jne 0x5427
            if fl.ne() { pc = 0x5427; continue; }
            pc = 0x53b0;
        }
        0x53b0 => {
            // block 53b0
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 53b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 53b0 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 53b3 mov rdi,QWORD PTR [rip+0x13adc6] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 53ba lea rsi,[rip+0x124ae4] # 0x129ea5
            rsi = m.env.data_va(255602);
            // 53c1 mov edx,0x78
            rdx = (0x78u64) as u32 as u64;
            // 53c6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 53cb mov rbx,rax
            rbx = rax;
            // 53ce mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 53d1 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 53d5 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 53d8 lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 53dc mov rdi,r12
            rdi = r12;
            // 53df call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 53e4 mov rsi,QWORD PTR [rip+0x13ada5] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 53eb mov rdi,r12
            rdi = r12;
            // 53ee call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 53f3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 53f6 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 53fa mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 53ff mov rdi,rax
            rdi = rax;
            // 5402 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5404 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 5407 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0x540b;
        }
        0x540b => {
            // block 540b
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 540b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 540b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5410 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 5414 mov rdi,rbx
            rdi = rbx;
            // 5417 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 541c mov rdi,rbx
            rdi = rbx;
            // 541f call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5424 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0x5427;
        }
        0x5427 => {
            // block 5427
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5427 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5427 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 542b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 542d mov rdi,r13
            rdi = r13;
            // 5430 mov ecx,DWORD PTR [rbp-0x44]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64) as u32 as u64;
            // 5433 call QWORD PTR [rax+0x310]
            { let r = { let t = m.r64(rax.wrapping_add(0x310)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5439 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 543d mov rdi,r13
            rdi = r13;
            // 5440 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5446 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5448 call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 544d mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 544f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5451 jne 0x519f
            if fl.ne() { pc = 0x519f; continue; }
            pc = 0x5457;
        }
        0x5457 => {
            // block 5457
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5457 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5457 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 545b dec r15d
            { let (r, f) = dec::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 545e cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 5462 sbb esi,esi
            { let (r, f) = sbb::<32>((rsi as u32 as u64), (rsi as u32 as u64), fl.cf);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5464 and esi,0x1
            { let r = (rsi as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5467 mov rdi,r13
            rdi = r13;
            // 546a call QWORD PTR [rax+0x5a0]
            { let r = { let t = m.r64(rax.wrapping_add(0x5a0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5470 jmp 0x519f
            pc = 0x519f; continue;
        }
        0x5475 => {
            // block 5475
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5475 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5475 mov rbx,rax
            rbx = rax;
            // 5478 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0x547c;
        }
        0x547c => {
            // block 547c
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 547c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 547c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5481 mov rdi,rbx
            rdi = rbx;
            // 5484 call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
        0x5492 => {
            // block 5492
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5492 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5492 mov rbx,rax
            rbx = rax;
            // 5495 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 5499 jmp 0x547c
            pc = 0x547c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10330 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x8fb
pub fn f_00010330<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10330;
    loop {
        match pc {
        0x10330 => {
            // block 10330
            #[cfg(feature = "trace")] eprintln!("f_00010330 pc 10330 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10330 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10331 mov rbp,rsp
            rbp = rsp;
            // 10334 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10339 call 0xec12e
            { let _ = { let t = m.env.stub_va(0x8fb); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10690 size=1138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
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
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
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

/// parity: P2v @0x18098 size=2576
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
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
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 180f1 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 180f5 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 180fa lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 180fe call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18103 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 18107 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1810c lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 18113 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18118 add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 1811f mov rdi,rbx
            rdi = rbx;
            // 18122 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18153 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 18157 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1815c lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 18160 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18165 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 18169 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1816e lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 18175 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1817a add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 18181 mov rdi,rbx
            rdi = rbx;
            // 18184 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18279 mov r12,r14
            r12 = r14;
            // 1827c lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 18280 mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 18287 lea rsi,[r13-0x18]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 1828b call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18290 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 18297 lea r14,[r15+0x90]
            r14 = r15.wrapping_add(0x90);
            // 1829e mov rdi,r14
            rdi = r14;
            // 182a1 mov rsi,r13
            rsi = r13;
            // 182a4 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182a9 lea rbx,[r15+0xa8]
            rbx = r15.wrapping_add(0xa8);
            // 182b0 lea rsi,[r13+0x18]
            rsi = r13.wrapping_add(0x18);
            // 182b4 mov rdi,rbx
            rdi = rbx;
            // 182b7 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182bc lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 182c3 lea rsi,[r13+0x30]
            rsi = r13.wrapping_add(0x30);
            // 182c7 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 183e5 mov QWORD PTR [rbp-0x150],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r14) as u64);
            // 183ec lea r14,[r15+0x78]
            r14 = r15.wrapping_add(0x78);
            // 183f0 lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 183f4 mov rdi,r14
            rdi = r14;
            // 183f7 call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 183fc mov QWORD PTR [rbp-0x158],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r14) as u64);
            // 18403 lea r14,[r15+0x90]
            r14 = r15.wrapping_add(0x90);
            // 1840a lea rsi,[rbx+0x90]
            rsi = rbx.wrapping_add(0x90);
            // 18411 mov rdi,r14
            rdi = r14;
            // 18414 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18419 lea r12,[r15+0xa8]
            r12 = r15.wrapping_add(0xa8);
            // 18420 lea rsi,[rbx+0xa8]
            rsi = rbx.wrapping_add(0xa8);
            // 18427 mov rdi,r12
            rdi = r12;
            // 1842a call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1842f lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 18436 lea rsi,[rbx+0xc0]
            rsi = rbx.wrapping_add(0xc0);
            // 1843d call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18512 lea rsi,[r13+0x78]
            rsi = r13.wrapping_add(0x78);
            // 18516 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1851d call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18522 lea rsi,[r13+0x90]
            rsi = r13.wrapping_add(0x90);
            // 18529 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18530 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18535 lea rsi,[r13+0xa8]
            rsi = r13.wrapping_add(0xa8);
            // 1853c lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18540 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18545 lea rsi,[r13+0xc0]
            rsi = r13.wrapping_add(0xc0);
            // 1854c lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 18550 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1865b lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 1865f mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 18666 lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 1866a call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1866f mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 18676 lea rdi,[r15+0x90]
            rdi = r15.wrapping_add(0x90);
            // 1867d mov QWORD PTR [rbp-0x168],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rdi) as u64);
            // 18684 lea rsi,[rbx+0x90]
            rsi = rbx.wrapping_add(0x90);
            // 1868b call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18690 lea r12,[r15+0xa8]
            r12 = r15.wrapping_add(0xa8);
            // 18697 lea rsi,[rbx+0xa8]
            rsi = rbx.wrapping_add(0xa8);
            // 1869e mov rdi,r12
            rdi = r12;
            // 186a1 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 186a6 lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 186ad lea rsi,[rbx+0xc0]
            rsi = rbx.wrapping_add(0xc0);
            // 186b4 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 187df mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // 187e6 lea rdi,[r12+0x78]
            rdi = r12.wrapping_add(0x78);
            // 187eb mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 187f2 lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 187f9 call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 187fe mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18805 lea rbx,[r12+0x90]
            rbx = r12.wrapping_add(0x90);
            // 1880d mov rdi,rbx
            rdi = rbx;
            // 18810 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18817 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1881c lea r15,[r12+0xa8]
            r15 = r12.wrapping_add(0xa8);
            // 18824 mov rdi,r15
            rdi = r15;
            // 18827 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1882b call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18830 lea rdi,[r12+0xc0]
            rdi = r12.wrapping_add(0xc0);
            // 18838 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1883c call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188aa lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 188ae call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188b3 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 188ba call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188bf lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 188c6 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188cb lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 188d2 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188d7 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 188de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189a3;
        }
        0x189a3 => {
            // block 189a3
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189a3 mov rdi,r14
            rdi = r14;
            // 189a6 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189ab;
        }
        0x189ab => {
            // block 189ab
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189ab mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 189b2 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189b7;
        }
        0x189b7 => {
            // block 189b7
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189b7 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 189be call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a3a;
        }
        0x18a3a => {
            // block 18a3a
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a3a mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 18a41 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a46;
        }
        0x18a46 => {
            // block 18a46
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a46 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 18a4d call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a52;
        }
        0x18a52 => {
            // block 18a52
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a52 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 18a59 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a5e;
        }
        0x18a5e => {
            // block 18a5e
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a5e lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 18a62 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a67;
        }
        0x18a67 => {
            // block 18a67
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a67 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18a6b call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a70;
        }
        0x18a70 => {
            // block 18a70
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a70 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18a77 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a7c;
        }
        0x18a7c => {
            // block 18a7c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a7c lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 18a83 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a88;
        }
        0x18a88 => {
            // block 18a88
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a88 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 18a8f call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18aa0 mov rdi,r13
            rdi = r13;
            // 18aa3 call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18ca1 size=682
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
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
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28aac size=276
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00028aac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28aac;
    loop {
        match pc {
        0x28aac => {
            // block 28aac
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28aac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28aac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28aad mov rbp,rsp
            rbp = rsp;
            // 28ab0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28ab2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28ab4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 28ab6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 28ab8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28ab9 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28abd mov r12,rsi
            r12 = rsi;
            // 28ac0 mov r14,rdi
            r14 = rdi;
            // 28ac3 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 28ac7 mov rsi,QWORD PTR [r12+0x10]
            rsi = m.r64(r12.wrapping_add(0x10)) as u64;
            // 28acc call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28ad1 cmp QWORD PTR [r12+0x10],0x1
            fl = sub::<64>(m.r64(r12.wrapping_add(0x10)) as u64, 0x1u64).1;
            // 28ad7 lea rcx,[rip+0x101d52] # 0x12a830
            rcx = m.env.data_va(258045);
            // 28ade lea rsi,[rip+0x101d5c] # 0x12a841
            rsi = m.env.data_va(258062);
            // 28ae5 cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 28ae9 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 28aec movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // 28aef or rdx,0x10
            { let r = rdx | 0x10u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 28af3 mov rdi,rax
            rdi = rax;
            // 28af6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28afb add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 28aff cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28b02 je 0x28b9a
            if fl.e() { pc = 0x28b9a; continue; }
            pc = 0x28b08;
        }
        0x28b08 => {
            // block 28b08
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b08 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28b0c lea r13,[rip+0x10198a] # 0x12a49d
            r13 = m.env.data_va(257130);
            pc = 0x28b13;
        }
        0x28b13 => {
            // block 28b13
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b13 mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 28b16 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28b18 mov rdi,r15
            rdi = r15;
            // 28b1b call 0x62a70
            { let r = { let t = m.env.code_va(0x62a70); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28b20 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 28b24 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 28b27 jne 0x28b32
            if fl.ne() { pc = 0x28b32; continue; }
            pc = 0x28b29;
        }
        0x28b29 => {
            // block 28b29
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b29 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 28b2c lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 28b30 jmp 0x28b3a
            pc = 0x28b3a; continue;
        }
        0x28b32 => {
            // block 28b32
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b32 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 28b36 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x28b3a;
        }
        0x28b3a => {
            // block 28b3a
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b3a mov rdi,r14
            rdi = r14;
            // 28b3d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28b42 mov rdi,r15
            rdi = r15;
            // 28b45 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28b4a mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 28b4e test rax,rax
            fl = logic::<64>(rax & rax);
            // 28b51 je 0x28b60
            if fl.e() { pc = 0x28b60; continue; }
            pc = 0x28b53;
        }
        0x28b53 => {
            // block 28b53
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b53 mov rbx,rax
            rbx = rax;
            // 28b56 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28b59 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28b5c jne 0x28b53
            if fl.ne() { pc = 0x28b53; continue; }
            pc = 0x28b5e;
        }
        0x28b5e => {
            // block 28b5e
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b5e jmp 0x28b6f
            pc = 0x28b6f; continue;
        }
        0x28b60 => {
            // block 28b60
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b60 mov rax,rbx
            rax = rbx;
            pc = 0x28b63;
        }
        0x28b63 => {
            // block 28b63
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b63 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28b67 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28b6a mov rax,rbx
            rax = rbx;
            // 28b6d jne 0x28b63
            if fl.ne() { pc = 0x28b63; continue; }
            pc = 0x28b6f;
        }
        0x28b6f => {
            // block 28b6f
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b6f cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28b72 mov rsi,r13
            rsi = r13;
            // 28b75 lea rax,[rip+0x11014c] # 0x138cc8
            rax = m.env.data_va(316565);
            // 28b7c cmove rsi,rax
            if fl.e() { rsi = rax; }
            // 28b80 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 28b83 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 28b86 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 28b89 mov rdi,r14
            rdi = r14;
            // 28b8c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28b91 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28b94 jne 0x28b13
            if fl.ne() { pc = 0x28b13; continue; }
            pc = 0x28b9a;
        }
        0x28b9a => {
            // block 28b9a
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b9a mov rax,r14
            rax = r14;
            // 28b9d add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28ba1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28ba2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 28ba4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 28ba6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28ba8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28baa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28bab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x28bac => {
            // block 28bac
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28bac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28bac mov rbx,rax
            rbx = rax;
            // 28baf lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28bb3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28bb8 mov rdi,rbx
            rdi = rbx;
            // 28bbb call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3101e size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_0003101e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3101e;
    loop {
        match pc {
        0x3101e => {
            // block 3101e
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3101e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3101e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3101f mov rbp,rsp
            rbp = rsp;
            // 31022 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31024 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31025 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 31029 mov r14,rdx
            r14 = rdx;
            // 3102c mov rbx,rdi
            rbx = rdi;
            // 3102f xorps xmm0,xmm0
            x0 = 0;
            // 31032 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 31036 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 3103e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31041 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 31045 mov ecx,DWORD PTR [r14+0x8]
            rcx = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // 31049 sub ecx,DWORD PTR [r14]
            { let (r, f) = sub::<32>((rcx as u32 as u64), m.r32(r14) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3104c lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 31050 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31052 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31054 je 0x3108d
            if fl.e() { pc = 0x3108d; continue; }
            pc = 0x31056;
        }
        0x31056 => {
            // block 31056
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31056 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31056 movzx edx,BYTE PTR [rbx+0x8]
            rdx = (m.r8(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 3105a lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 3105e mov rdi,r14
            rdi = r14;
            // 31061 call 0x2eee0
            { let r = { let t = m.env.code_va(0x2eee0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31066 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 31068 jmp 0x3108f
            pc = 0x3108f; continue;
        }
        0x3106a => {
            // block 3106a
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3106a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3106a mov rbx,rax
            rbx = rax;
            // 3106d mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 31071 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31074 je 0x31085
            if fl.e() { pc = 0x31085; continue; }
            pc = 0x31076;
        }
        0x31076 => {
            // block 31076
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31076 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31076 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 3107a je 0x31080
            if fl.e() { pc = 0x31080; continue; }
            pc = 0x3107c;
        }
        0x3107c => {
            // block 3107c
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3107c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3107c mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x31080;
        }
        0x31080 => {
            // block 31080
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31080 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31080 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31085;
        }
        0x31085 => {
            // block 31085
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31085 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31085 mov rdi,rbx
            rdi = rbx;
            // 31088 call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
        0x3108d => {
            // block 3108d
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3108d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3108d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x3108f;
        }
        0x3108f => {
            // block 3108f
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3108f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3108f mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 31093 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31096 je 0x310a7
            if fl.e() { pc = 0x310a7; continue; }
            pc = 0x31098;
        }
        0x31098 => {
            // block 31098
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31098 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 3109c je 0x310a2
            if fl.e() { pc = 0x310a2; continue; }
            pc = 0x3109e;
        }
        0x3109e => {
            // block 3109e
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3109e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3109e mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x310a2;
        }
        0x310a2 => {
            // block 310a2
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 310a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 310a2 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x310a7;
        }
        0x310a7 => {
            // block 310a7
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 310a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 310a7 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 310a9 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 310ad pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 310ae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 310b0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 310b1 ret
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

/// parity: P2v @0x31ccc size=1046
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f, stub:0x961
pub fn f_00031ccc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x31ccc;
    loop {
        match pc {
        0x31ccc => {
            // block 31ccc
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31ccc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31ccc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 31ccd mov rbp,rsp
            rbp = rsp;
            // 31cd0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 31cd2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31cd4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 31cd6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 31cd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31cd9 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 31ce0 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 31ce3 mov r15,rsi
            r15 = rsi;
            // 31ce6 mov r13,rdi
            r13 = rdi;
            // 31ce9 mov r14,QWORD PTR [rip+0x10e6e8] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // 31cf0 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // 31cf3 mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 31cf7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 31cfa mov rdi,r15
            rdi = r15;
            // 31cfd call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31d03 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31d05 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31d07 je 0x3208c
            if fl.e() { pc = 0x3208c; continue; }
            pc = 0x31d0d;
        }
        0x31d0d => {
            // block 31d0d
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31d0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31d0d mov QWORD PTR [rbp-0xc0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r13) as u64);
            // 31d14 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 31d17 lea rcx,[rip+0xce142] # 0xffe60
            rcx = m.env.data_va(83501);
            // 31d1e mov r14d,DWORD PTR [rcx+rax*4]
            r14 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 31d22 lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 31d29 mov QWORD PTR [rbp-0x88],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0x0u64) as u64);
            // 31d34 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 31d3f mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 31d46 lea r12,[rbp-0x98]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 31d4d lea r13,[rbp-0x9c]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff64);
            pc = 0x31d54;
        }
        0x31d54 => {
            // block 31d54
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31d54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31d54 lea eax,[r14+rbx*1]
            rax = (r14.wrapping_add(rbx)) as u32 as u64;
            // 31d58 mov DWORD PTR [rbp-0x9c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff64), ((rax as u32 as u64)) as u32);
            // 31d5e mov rdi,r12
            rdi = r12;
            // 31d61 mov rsi,r13
            rsi = r13;
            // 31d64 call 0xebb22
            { let r = { let t = m.env.stub_va(0x2ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31d69 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 31d6b cmp ebx,0x11
            fl = sub::<32>((rbx as u32 as u64), 0x11u64).1;
            // 31d6e jl 0x31d54
            if fl.l() { pc = 0x31d54; continue; }
            pc = 0x31d70;
        }
        0x31d70 => {
            // block 31d70
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31d70 lea rbx,[rbp-0xb0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31d77 mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 31d82 mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // 31d8d mov QWORD PTR [rbp-0xb8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rbx) as u64);
            // 31d94 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 31d97 mov rax,QWORD PTR [rax+0xc90]
            rax = m.r64(rax.wrapping_add(0xc90)) as u64;
            // 31d9e lea rsi,[rbp-0x98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 31da5 lea rdx,[rbp-0xb8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 31dac mov rdi,r15
            rdi = r15;
            // 31daf call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31db1 mov r14,QWORD PTR [rip+0x10e620] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // 31db8 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // 31dbb mov r15,QWORD PTR [rbp-0xc0]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // 31dc2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31dc4 je 0x31e4f
            if fl.e() { pc = 0x31e4f; continue; }
            pc = 0x31dca;
        }
        0x31dca => {
            // block 31dca
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dca cmp QWORD PTR [rbp-0xa8],0x11
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, 0x11u64).1;
            // 31dd2 jne 0x31e4f
            if fl.ne() { pc = 0x31e4f; continue; }
            pc = 0x31dd4;
        }
        0x31dd4 => {
            // block 31dd4
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dd4 mov rcx,QWORD PTR [rbp-0xb8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // 31ddb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31ddd cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 31de0 je 0x31e56
            if fl.e() { pc = 0x31e56; continue; }
            pc = 0x31de2;
        }
        0x31de2 => {
            // block 31de2
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31de2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31de2 mov edx,DWORD PTR [rcx+0x20]
            rdx = (m.r32(rcx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 31de5 mov DWORD PTR [rbp+rax*4-0x80],edx
            m.w32(rbp.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff80), ((rdx as u32 as u64)) as u32);
            // 31de9 mov rdx,QWORD PTR [rcx+0x8]
            rdx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 31ded test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 31df0 je 0x31dff
            if fl.e() { pc = 0x31dff; continue; }
            pc = 0x31df2;
        }
        0x31df2 => {
            // block 31df2
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31df2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31df2 mov rcx,rdx
            rcx = rdx;
            // 31df5 mov rdx,QWORD PTR [rcx]
            rdx = m.r64(rcx) as u64;
            // 31df8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 31dfb jne 0x31df2
            if fl.ne() { pc = 0x31df2; continue; }
            pc = 0x31dfd;
        }
        0x31dfd => {
            // block 31dfd
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dfd jmp 0x31e0e
            pc = 0x31e0e; continue;
        }
        0x31dff => {
            // block 31dff
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dff mov rdx,rcx
            rdx = rcx;
            pc = 0x31e02;
        }
        0x31e02 => {
            // block 31e02
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e02 mov rcx,QWORD PTR [rdx+0x10]
            rcx = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 31e06 cmp QWORD PTR [rcx],rdx
            fl = sub::<64>(m.r64(rcx) as u64, rdx).1;
            // 31e09 mov rdx,rcx
            rdx = rcx;
            // 31e0c jne 0x31e02
            if fl.ne() { pc = 0x31e02; continue; }
            pc = 0x31e0e;
        }
        0x31e0e => {
            // block 31e0e
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e0e inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 31e11 cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 31e14 jne 0x31de2
            if fl.ne() { pc = 0x31de2; continue; }
            pc = 0x31e16;
        }
        0x31e16 => {
            // block 31e16
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e16 mov ecx,DWORD PTR [rbp-0x80]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            // 31e19 mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // 31e1c mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 31e1e shr edx,0x4
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x4u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 31e21 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 31e23 shr esi,0xc
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xcu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 31e26 movd xmm0,ecx
            x0 = ((rcx as u32 as u64)) as u32 as u128;
            // 31e2a shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 31e2d movd xmm1,ecx
            x1 = ((rcx as u32 as u64)) as u32 as u128;
            // 31e31 movd xmm2,edx
            x2 = ((rdx as u32 as u64)) as u32 as u128;
            // 31e35 punpckldq xmm2,xmm1
            { let a = x2; let b = x1; x2 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e39 movd xmm1,esi
            x1 = ((rsi as u32 as u64)) as u32 as u128;
            // 31e3d punpckldq xmm0,xmm1
            { let a = x0; let b = x1; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e41 punpckldq xmm0,xmm2
            { let a = x0; let b = x2; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e45 pand xmm0,XMMWORD PTR [rip+0xce003] # 0xffe50
            x0 &= m.r128(m.env.data_va(83485));
            // 31e4d jmp 0x31e5d
            pc = 0x31e5d; continue;
        }
        0x31e4f => {
            // block 31e4f
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e4f xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31e51 jmp 0x32066
            pc = 0x32066; continue;
        }
        0x31e56 => {
            // block 31e56
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e56 pxor xmm0,xmm0
            x0 = 0;
            // 31e5a mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            pc = 0x31e5d;
        }
        0x31e5d => {
            // block 31e5d
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e5d movdqu XMMWORD PTR [r15+0x8],xmm0
            m.w128(r15.wrapping_add(0x8), x0);
            // 31e63 lea rbx,[r15+0x30]
            rbx = r15.wrapping_add(0x30);
            // 31e67 mov rcx,QWORD PTR [r15+0x30]
            rcx = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31e6b mov rcx,QWORD PTR [rcx+0x30]
            rcx = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 31e6f movsx edx,ax
            rdx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // 31e72 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31e74 mov rdi,rbx
            rdi = rbx;
            // 31e77 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31e79 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31e7c mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31e80 movsx edx,WORD PTR [rbp-0x7a]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff86)) as u64) as u64) as u32 as u64;
            // 31e84 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31e89 mov rdi,rbx
            rdi = rbx;
            // 31e8c call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31e8e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31e91 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31e95 movsx edx,WORD PTR [rbp-0x78]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u64) as u32 as u64;
            // 31e99 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31e9e mov rdi,rbx
            rdi = rbx;
            // 31ea1 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ea3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ea6 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31eaa mov ecx,DWORD PTR [rbp-0x74]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // 31ead cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31eb2 mulsd xmm0,QWORD PTR [rip+0xcdf86] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31eba xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31ebc mov rdi,rbx
            rdi = rbx;
            // 31ebf call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ec1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ec4 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31ec8 mov ecx,DWORD PTR [rbp-0x70]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // 31ecb xorps xmm0,xmm0
            x0 = 0;
            // 31ece cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31ed3 mulsd xmm0,QWORD PTR [rip+0xcdf65] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31edb mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31ee0 mov rdi,rbx
            rdi = rbx;
            // 31ee3 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ee5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ee8 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31eec mov ecx,DWORD PTR [rbp-0x6c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // 31eef xorps xmm0,xmm0
            x0 = 0;
            // 31ef2 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31ef7 mulsd xmm0,QWORD PTR [rip+0xcdf41] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31eff mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 31f04 mov rdi,rbx
            rdi = rbx;
            // 31f07 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f09 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f0c mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f10 mov ecx,DWORD PTR [rbp-0x68]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 31f13 xorps xmm0,xmm0
            x0 = 0;
            // 31f16 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f1b mulsd xmm0,QWORD PTR [rip+0xcdf1d] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f23 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31f28 mov rdi,rbx
            rdi = rbx;
            // 31f2b call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f2d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f30 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f34 mov ecx,DWORD PTR [rbp-0x64]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 31f37 xorps xmm0,xmm0
            x0 = 0;
            // 31f3a cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f3f mulsd xmm0,QWORD PTR [rip+0xcdef9] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f47 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 31f4c mov rdi,rbx
            rdi = rbx;
            // 31f4f call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f51 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f54 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f58 mov ecx,DWORD PTR [rbp-0x60]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 31f5b xorps xmm0,xmm0
            x0 = 0;
            // 31f5e cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f63 mulsd xmm0,QWORD PTR [rip+0xcded5] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f6b mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 31f70 mov rdi,rbx
            rdi = rbx;
            // 31f73 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f75 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f78 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f7c mov ecx,DWORD PTR [rbp-0x5c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // 31f7f xorps xmm0,xmm0
            x0 = 0;
            // 31f82 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f87 mulsd xmm0,QWORD PTR [rip+0xcdeb1] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f8f mov esi,0x6
            rsi = (0x6u64) as u32 as u64;
            // 31f94 mov rdi,rbx
            rdi = rbx;
            // 31f97 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f99 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f9c mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31fa0 mov ecx,DWORD PTR [rbp-0x58]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 31fa3 xorps xmm0,xmm0
            x0 = 0;
            // 31fa6 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31fab mulsd xmm0,QWORD PTR [rip+0xcde8d] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31fb3 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 31fb8 mov rdi,rbx
            rdi = rbx;
            // 31fbb call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31fbd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31fc0 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31fc4 mov ecx,DWORD PTR [rbp-0x54]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64) as u32 as u64;
            // 31fc7 xorps xmm0,xmm0
            x0 = 0;
            // 31fca cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31fcf mulsd xmm0,QWORD PTR [rip+0xcde69] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31fd7 mov esi,0x8
            rsi = (0x8u64) as u32 as u64;
            // 31fdc mov rdi,rbx
            rdi = rbx;
            // 31fdf call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31fe1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31fe4 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31fe8 movsx edx,WORD PTR [rbp-0x50]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64) as u64) as u32 as u64;
            // 31fec mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31ff1 mov rdi,rbx
            rdi = rbx;
            // 31ff4 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ff6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ff9 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31ffd movsx edx,WORD PTR [rbp-0x4e]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb2)) as u64) as u64) as u32 as u64;
            // 32001 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 32006 mov rdi,rbx
            rdi = rbx;
            // 32009 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 3200b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3200e mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 32012 movsx edx,WORD PTR [rbp-0x4c]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u64) as u32 as u64;
            // 32016 mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 3201b mov rdi,rbx
            rdi = rbx;
            // 3201e call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 32020 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 32024 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32026 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 32029 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3202c mov DWORD PTR [r15+0x1c],ecx
            m.w32(r15.wrapping_add(0x1c), ((rcx as u32 as u64)) as u32);
            // 32030 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32032 and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 32035 mov DWORD PTR [r15+0x18],ecx
            m.w32(r15.wrapping_add(0x18), ((rcx as u32 as u64)) as u32);
            // 32039 mov rcx,rax
            rcx = rax;
            // 3203c shr rcx,0x20
            { let (r, f) = shr::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // 32040 mov WORD PTR [r15+0x20],cx
            m.w16(r15.wrapping_add(0x20), ((rcx as u16 as u64)) as u16);
            // 32045 shr rax,0x30
            { let (r, f) = shr::<64>(rax, 0x30u64, fl); rax = r; fl = f; }
            // 32049 mov WORD PTR [r15+0x22],ax
            m.w16(r15.wrapping_add(0x22), ((rax as u16 as u64)) as u16);
            // 3204e mov eax,DWORD PTR [rbp-0x40]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 32051 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 32056 mulsd xmm0,QWORD PTR [rip+0xcddea] # 0xffe48
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83477)) as u128))).to_bits());
            // 3205e movsd QWORD PTR [r15+0x28],xmm0
            m.w64(r15.wrapping_add(0x28), x0 as u64);
            // 32064 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x32066;
        }
        0x32066 => {
            // block 32066
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 32066 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32066 mov rsi,QWORD PTR [rbp-0xb0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 3206d lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 32074 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 32079 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 32080 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 32087 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            pc = 0x3208c;
        }
        0x3208c => {
            // block 3208c
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 3208c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3208c cmp r14,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 32090 jne 0x320dc
            if fl.ne() { pc = 0x320dc; continue; }
            pc = 0x32092;
        }
        0x32092 => {
            // block 32092
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 32092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32092 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 32094 add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 3209b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3209c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 3209e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 320a0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 320a2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 320a4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 320a5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x320a6 => {
            // block 320a6
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 320a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320a6 mov rbx,rax
            rbx = rax;
            pc = 0x320a9;
        }
        0x320a9 => {
            // block 320a9
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 320a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320a9 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 320b0 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 320b7 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 320bc mov rdi,rbx
            rdi = rbx;
            // 320bf call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
        0x320dc => {
            // block 320dc
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 320dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320dc call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4403e size=194
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_0004403e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4403e;
    loop {
        match pc {
        0x4403e => {
            // block 4403e
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 4403e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4403e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4403f mov rbp,rsp
            rbp = rsp;
            // 44042 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44044 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44046 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 44048 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44049 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 4404d mov r15,rcx
            r15 = rcx;
            // 44050 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 44052 lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 44056 mov QWORD PTR [rbp-0x28],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u64);
            // 4405e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 44066 mov QWORD PTR [rbp-0x38],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r12) as u64);
            // 4406a mov BYTE PTR [r15],0x0
            m.w8(r15, (0x0u64) as u8);
            // 4406e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 44071 mov rax,QWORD PTR [rax+0x810]
            rax = m.r64(rax.wrapping_add(0x810)) as u64;
            // 44078 lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 4407c call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4407e mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 44082 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44084 je 0x440b4
            if fl.e() { pc = 0x440b4; continue; }
            pc = 0x44086;
        }
        0x44086 => {
            // block 44086
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 44086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44086 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 44089 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 4408c je 0x440cc
            if fl.e() { pc = 0x440cc; continue; }
            pc = 0x4408e;
        }
        0x4408e => {
            // block 4408e
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 4408e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4408e mov rcx,r12
            rcx = r12;
            // 44091 mov rdx,rsi
            rdx = rsi;
            pc = 0x44094;
        }
        0x44094 => {
            // block 44094
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 44094 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44094 mov rax,rdx
            rax = rdx;
            pc = 0x44097;
        }
        0x44097 => {
            // block 44097
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 44097 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44097 cmp DWORD PTR [rax+0x1c],ebx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rbx as u32 as u64)).1;
            // 4409a jge 0x440a7
            if fl.ge() { pc = 0x440a7; continue; }
            pc = 0x4409c;
        }
        0x4409c => {
            // block 4409c
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 4409c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4409c mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 440a0 test rax,rax
            fl = logic::<64>(rax & rax);
            // 440a3 jne 0x44097
            if fl.ne() { pc = 0x44097; continue; }
            pc = 0x440a5;
        }
        0x440a5 => {
            // block 440a5
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440a5 jmp 0x440b9
            pc = 0x440b9; continue;
        }
        0x440a7 => {
            // block 440a7
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440a7 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 440aa test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 440ad mov rcx,rax
            rcx = rax;
            // 440b0 jne 0x44094
            if fl.ne() { pc = 0x44094; continue; }
            pc = 0x440b2;
        }
        0x440b2 => {
            // block 440b2
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440b2 jmp 0x440bc
            pc = 0x440bc; continue;
        }
        0x440b4 => {
            // block 440b4
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440b4 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 440b7 jmp 0x440ce
            pc = 0x440ce; continue;
        }
        0x440b9 => {
            // block 440b9
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440b9 mov rax,rcx
            rax = rcx;
            pc = 0x440bc;
        }
        0x440bc => {
            // block 440bc
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440bc cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 440bf je 0x440ce
            if fl.e() { pc = 0x440ce; continue; }
            pc = 0x440c1;
        }
        0x440c1 => {
            // block 440c1
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440c1 cmp DWORD PTR [rax+0x1c],ebx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rbx as u32 as u64)).1;
            // 440c4 jg 0x440ce
            if fl.g() { pc = 0x440ce; continue; }
            pc = 0x440c6;
        }
        0x440c6 => {
            // block 440c6
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440c6 mov BYTE PTR [r15],0x1
            m.w8(r15, (0x1u64) as u8);
            // 440ca jmp 0x440ce
            pc = 0x440ce; continue;
        }
        0x440cc => {
            // block 440cc
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440cc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x440ce;
        }
        0x440ce => {
            // block 440ce
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440ce lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 440d2 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 440d7 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 440da add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 440de pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 440df pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 440e1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 440e3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 440e5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 440e6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x440e7 => {
            // block 440e7
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440e7 mov rbx,rax
            rbx = rax;
            // 440ea mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 440ee lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 440f2 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 440f7 mov rdi,rbx
            rdi = rbx;
            // 440fa call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63714 size=1439
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00063714<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x63714;
    loop {
        match pc {
        0x63714 => {
            // block 63714
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63714 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63714 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63715 mov rbp,rsp
            rbp = rsp;
            // 63718 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6371a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6371c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6371d sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // 63724 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 63726 mov r15,rdi
            r15 = rdi;
            // 63729 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 6372d call 0x633ff
            { let r = { let t = m.env.code_va(0x633ff); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63732 movzx edx,BYTE PTR [r15]
            rdx = (m.r8(r15) as u64) as u32 as u64;
            // 63736 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63738 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6373a jne 0x63741
            if fl.ne() { pc = 0x63741; continue; }
            pc = 0x6373c;
        }
        0x6373c => {
            // block 6373c
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6373c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6373c shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 6373f jmp 0x63745
            pc = 0x63745; continue;
        }
        0x63741 => {
            // block 63741
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63741 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63741 mov rdx,QWORD PTR [r15+0x8]
            rdx = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x63745;
        }
        0x63745 => {
            // block 63745
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63745 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63745 movzx esi,BYTE PTR [rbp-0x30]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 63749 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 6374c and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6374f jne 0x63756
            if fl.ne() { pc = 0x63756; continue; }
            pc = 0x63751;
        }
        0x63751 => {
            // block 63751
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63751 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63751 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63754 jmp 0x6375a
            pc = 0x6375a; continue;
        }
        0x63756 => {
            // block 63756
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63756 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63756 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            pc = 0x6375a;
        }
        0x6375a => {
            // block 6375a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6375a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6375a cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 6375d jne 0x637bb
            if fl.ne() { pc = 0x637bb; continue; }
            pc = 0x6375f;
        }
        0x6375f => {
            // block 6375f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6375f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6375f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63761 jne 0x63769
            if fl.ne() { pc = 0x63769; continue; }
            pc = 0x63763;
        }
        0x63763 => {
            // block 63763
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63763 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63763 lea rdi,[r15+0x1]
            rdi = r15.wrapping_add(0x1);
            // 63767 jmp 0x6376d
            pc = 0x6376d; continue;
        }
        0x63769 => {
            // block 63769
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63769 mov rdi,QWORD PTR [r15+0x10]
            rdi = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x6376d;
        }
        0x6376d => {
            // block 6376d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6376d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6376d test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 6376f jne 0x63777
            if fl.ne() { pc = 0x63777; continue; }
            pc = 0x63771;
        }
        0x63771 => {
            // block 63771
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63771 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63771 lea rsi,[rbp-0x2f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd1);
            // 63775 jmp 0x6377b
            pc = 0x6377b; continue;
        }
        0x63777 => {
            // block 63777
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63777 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63777 mov rsi,QWORD PTR [rbp-0x20]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            pc = 0x6377b;
        }
        0x6377b => {
            // block 6377b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6377b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6377b mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 6377e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63780 jne 0x637a5
            if fl.ne() { pc = 0x637a5; continue; }
            pc = 0x63782;
        }
        0x63782 => {
            // block 63782
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63782 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63782 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63785 je 0x63c85
            if fl.e() { pc = 0x63c85; continue; }
            pc = 0x6378b;
        }
        0x6378b => {
            // block 6378b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6378b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6378b movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 6378e movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63791 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63793 jne 0x637bb
            if fl.ne() { pc = 0x637bb; continue; }
            pc = 0x63795;
        }
        0x63795 => {
            // block 63795
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63795 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63795 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63798 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 6379b dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 6379e jne 0x6378b
            if fl.ne() { pc = 0x6378b; continue; }
            pc = 0x637a0;
        }
        0x637a0 => {
            // block 637a0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637a0 jmp 0x63c85
            pc = 0x63c85; continue;
        }
        0x637a5 => {
            // block 637a5
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637a5 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 637a8 je 0x63c85
            if fl.e() { pc = 0x63c85; continue; }
            pc = 0x637ae;
        }
        0x637ae => {
            // block 637ae
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637ae call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 637b3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 637b5 je 0x63c85
            if fl.e() { pc = 0x63c85; continue; }
            pc = 0x637bb;
        }
        0x637bb => {
            // block 637bb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637bb xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 637be cmp ebx,0x104782ff
            fl = sub::<32>((rbx as u32 as u64), 0x104782ffu64).1;
            // 637c4 jg 0x637f9
            if fl.g() { pc = 0x637f9; continue; }
            pc = 0x637c6;
        }
        0x637c6 => {
            // block 637c6
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637c6 cmp ebx,0x10294700
            fl = sub::<32>((rbx as u32 as u64), 0x10294700u64).1;
            // 637cc jne 0x6383e
            if fl.ne() { pc = 0x6383e; continue; }
            pc = 0x637ce;
        }
        0x637ce => {
            // block 637ce
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637ce lea rsi,[rip+0xce0b8] # 0x13188d
            rsi = m.env.data_va(286810);
            // 637d5 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 637d9 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 637de call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 637e3 movzx edx,BYTE PTR [rbp-0x60]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 637e7 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 637e9 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 637eb jne 0x63922
            if fl.ne() { pc = 0x63922; continue; }
            pc = 0x637f1;
        }
        0x637f1 => {
            // block 637f1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637f1 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 637f4 jmp 0x63926
            pc = 0x63926; continue;
        }
        0x637f9 => {
            // block 637f9
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637f9 cmp ebx,0x105183ff
            fl = sub::<32>((rbx as u32 as u64), 0x105183ffu64).1;
            // 637ff jg 0x63875
            if fl.g() { pc = 0x63875; continue; }
            pc = 0x63801;
        }
        0x63801 => {
            // block 63801
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63801 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63801 cmp ebx,0x10478300
            fl = sub::<32>((rbx as u32 as u64), 0x10478300u64).1;
            // 63807 jne 0x638a8
            if fl.ne() { pc = 0x638a8; continue; }
            pc = 0x6380d;
        }
        0x6380d => {
            // block 6380d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6380d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6380d lea rsi,[rip+0xce03b] # 0x13184f
            rsi = m.env.data_va(286748);
            // 63814 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 6381b mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 63820 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63825 movzx edx,BYTE PTR [rbp-0xa8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64) as u32 as u64;
            // 6382c mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 6382e and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63830 jne 0x6394e
            if fl.ne() { pc = 0x6394e; continue; }
            pc = 0x63836;
        }
        0x63836 => {
            // block 63836
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63836 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63836 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 63839 jmp 0x63955
            pc = 0x63955; continue;
        }
        0x6383e => {
            // block 6383e
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6383e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6383e cmp ebx,0x10322950
            fl = sub::<32>((rbx as u32 as u64), 0x10322950u64).1;
            // 63844 jne 0x63c85
            if fl.ne() { pc = 0x63c85; continue; }
            pc = 0x6384a;
        }
        0x6384a => {
            // block 6384a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6384a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6384a lea rsi,[rip+0xce030] # 0x131881
            rsi = m.env.data_va(286798);
            // 63851 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 63855 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 6385a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6385f movzx edx,BYTE PTR [rbp-0x48]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 63863 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63865 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63867 jne 0x63966
            if fl.ne() { pc = 0x63966; continue; }
            pc = 0x6386d;
        }
        0x6386d => {
            // block 6386d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6386d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6386d shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 63870 jmp 0x6396a
            pc = 0x6396a; continue;
        }
        0x63875 => {
            // block 63875
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63875 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63875 cmp ebx,0x10518400
            fl = sub::<32>((rbx as u32 as u64), 0x10518400u64).1;
            // 6387b jne 0x638e5
            if fl.ne() { pc = 0x638e5; continue; }
            pc = 0x6387d;
        }
        0x6387d => {
            // block 6387d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6387d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6387d lea rsi,[rip+0xce026] # 0x1318aa
            rsi = m.env.data_va(286839);
            // 63884 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 63888 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 6388d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63892 movzx edx,BYTE PTR [rbp-0x78]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // 63896 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63898 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6389a jne 0x6397b
            if fl.ne() { pc = 0x6397b; continue; }
            pc = 0x638a0;
        }
        0x638a0 => {
            // block 638a0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638a0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 638a3 jmp 0x6397f
            pc = 0x6397f; continue;
        }
        0x638a8 => {
            // block 638a8
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638a8 cmp ebx,0x10478350
            fl = sub::<32>((rbx as u32 as u64), 0x10478350u64).1;
            // 638ae jne 0x63c85
            if fl.ne() { pc = 0x63c85; continue; }
            pc = 0x638b4;
        }
        0x638b4 => {
            // block 638b4
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638b4 lea rsi,[rip+0xcdf87] # 0x131842
            rsi = m.env.data_va(286735);
            // 638bb lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 638c2 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 638c7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 638cc movzx edx,BYTE PTR [rbp-0xc0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64) as u32 as u64;
            // 638d3 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 638d5 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 638d7 jne 0x63a43
            if fl.ne() { pc = 0x63a43; continue; }
            pc = 0x638dd;
        }
        0x638dd => {
            // block 638dd
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638dd shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 638e0 jmp 0x63a4a
            pc = 0x63a4a; continue;
        }
        0x638e5 => {
            // block 638e5
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638e5 cmp ebx,0x10518450
            fl = sub::<32>((rbx as u32 as u64), 0x10518450u64).1;
            // 638eb jne 0x63c85
            if fl.ne() { pc = 0x63c85; continue; }
            pc = 0x638f1;
        }
        0x638f1 => {
            // block 638f1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638f1 lea rsi,[rip+0xcdfa2] # 0x13189a
            rsi = m.env.data_va(286823);
            // 638f8 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 638ff mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 63904 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63909 movzx edx,BYTE PTR [rbp-0x90]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64) as u32 as u64;
            // 63910 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63912 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63914 jne 0x63a5b
            if fl.ne() { pc = 0x63a5b; continue; }
            pc = 0x6391a;
        }
        0x6391a => {
            // block 6391a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6391a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6391a shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 6391d jmp 0x63a62
            pc = 0x63a62; continue;
        }
        0x63922 => {
            // block 63922
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63922 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63922 mov rdx,QWORD PTR [rbp-0x58]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x63926;
        }
        0x63926 => {
            // block 63926
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63926 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63926 movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 6392a mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 6392d and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63930 jne 0x63937
            if fl.ne() { pc = 0x63937; continue; }
            pc = 0x63932;
        }
        0x63932 => {
            // block 63932
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63932 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63932 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63935 jmp 0x6393b
            pc = 0x6393b; continue;
        }
        0x63937 => {
            // block 63937
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63937 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63937 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x6393b;
        }
        0x6393b => {
            // block 6393b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6393b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6393b cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 6393e jne 0x63a22
            if fl.ne() { pc = 0x63a22; continue; }
            pc = 0x63944;
        }
        0x63944 => {
            // block 63944
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63944 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63944 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63946 jne 0x63990
            if fl.ne() { pc = 0x63990; continue; }
            pc = 0x63948;
        }
        0x63948 => {
            // block 63948
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63948 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63948 lea rdi,[rbp-0x5f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa1);
            // 6394c jmp 0x63994
            pc = 0x63994; continue;
        }
        0x6394e => {
            // block 6394e
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6394e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6394e mov rdx,QWORD PTR [rbp-0xa0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            pc = 0x63955;
        }
        0x63955 => {
            // block 63955
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63955 movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63959 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 6395c and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6395f jne 0x6399d
            if fl.ne() { pc = 0x6399d; continue; }
            pc = 0x63961;
        }
        0x63961 => {
            // block 63961
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63961 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63961 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63964 jmp 0x639a1
            pc = 0x639a1; continue;
        }
        0x63966 => {
            // block 63966
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63966 mov rdx,QWORD PTR [rbp-0x40]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            pc = 0x6396a;
        }
        0x6396a => {
            // block 6396a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6396a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6396a movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 6396e mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63971 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63974 jne 0x639be
            if fl.ne() { pc = 0x639be; continue; }
            pc = 0x63976;
        }
        0x63976 => {
            // block 63976
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63976 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63979 jmp 0x639c2
            pc = 0x639c2; continue;
        }
        0x6397b => {
            // block 6397b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6397b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6397b mov rdx,QWORD PTR [rbp-0x70]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x6397f;
        }
        0x6397f => {
            // block 6397f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6397f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6397f movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63983 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63986 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63989 jne 0x639dc
            if fl.ne() { pc = 0x639dc; continue; }
            pc = 0x6398b;
        }
        0x6398b => {
            // block 6398b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6398b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6398b shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 6398e jmp 0x639e0
            pc = 0x639e0; continue;
        }
        0x63990 => {
            // block 63990
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63990 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63990 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0x63994;
        }
        0x63994 => {
            // block 63994
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63994 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63996 jne 0x639fa
            if fl.ne() { pc = 0x639fa; continue; }
            pc = 0x63998;
        }
        0x63998 => {
            // block 63998
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63998 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63998 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 6399b jmp 0x639fe
            pc = 0x639fe; continue;
        }
        0x6399d => {
            // block 6399d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6399d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6399d mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x639a1;
        }
        0x639a1 => {
            // block 639a1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639a1 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 639a4 jne 0x63b1d
            if fl.ne() { pc = 0x63b1d; continue; }
            pc = 0x639aa;
        }
        0x639aa => {
            // block 639aa
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639aa test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639ac jne 0x63a73
            if fl.ne() { pc = 0x63a73; continue; }
            pc = 0x639b2;
        }
        0x639b2 => {
            // block 639b2
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639b2 lea rdi,[rbp-0xa7]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff59);
            // 639b9 jmp 0x63a7a
            pc = 0x63a7a; continue;
        }
        0x639be => {
            // block 639be
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639be mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x639c2;
        }
        0x639c2 => {
            // block 639c2
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639c2 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 639c5 jne 0x63b4a
            if fl.ne() { pc = 0x63b4a; continue; }
            pc = 0x639cb;
        }
        0x639cb => {
            // block 639cb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639cb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639cd jne 0x63a83
            if fl.ne() { pc = 0x63a83; continue; }
            pc = 0x639d3;
        }
        0x639d3 => {
            // block 639d3
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639d3 lea rdi,[rbp-0x47]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb9);
            // 639d7 jmp 0x63a87
            pc = 0x63a87; continue;
        }
        0x639dc => {
            // block 639dc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639dc mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x639e0;
        }
        0x639e0 => {
            // block 639e0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639e0 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 639e3 jne 0x63b77
            if fl.ne() { pc = 0x63b77; continue; }
            pc = 0x639e9;
        }
        0x639e9 => {
            // block 639e9
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639e9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639eb jne 0x63a97
            if fl.ne() { pc = 0x63a97; continue; }
            pc = 0x639f1;
        }
        0x639f1 => {
            // block 639f1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639f1 lea rdi,[rbp-0x77]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff89);
            // 639f5 jmp 0x63a9b
            pc = 0x63a9b; continue;
        }
        0x639fa => {
            // block 639fa
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639fa mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x639fe;
        }
        0x639fe => {
            // block 639fe
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639fe mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63a01 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63a03 jne 0x63a27
            if fl.ne() { pc = 0x63a27; continue; }
            pc = 0x63a05;
        }
        0x63a05 => {
            // block 63a05
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a05 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63a08 je 0x63a3a
            if fl.e() { pc = 0x63a3a; continue; }
            pc = 0x63a0a;
        }
        0x63a0a => {
            // block 63a0a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a0a movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63a0e movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63a11 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63a13 jne 0x63a22
            if fl.ne() { pc = 0x63a22; continue; }
            pc = 0x63a15;
        }
        0x63a15 => {
            // block 63a15
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a15 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63a18 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63a1b dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63a1e jne 0x63a0a
            if fl.ne() { pc = 0x63a0a; continue; }
            pc = 0x63a20;
        }
        0x63a20 => {
            // block 63a20
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a20 jmp 0x63a3a
            pc = 0x63a3a; continue;
        }
        0x63a22 => {
            // block 63a22
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a22 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63a25 jmp 0x63a3a
            pc = 0x63a3a; continue;
        }
        0x63a27 => {
            // block 63a27
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a27 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63a2a je 0x63a3a
            if fl.e() { pc = 0x63a3a; continue; }
            pc = 0x63a2c;
        }
        0x63a2c => {
            // block 63a2c
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a2c mov rsi,r15
            rsi = r15;
            // 63a2f call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63a34 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63a36 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63a3a;
        }
        0x63a3a => {
            // block 63a3a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a3a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 63a3e jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63a43 => {
            // block 63a43
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a43 mov rdx,QWORD PTR [rbp-0xb8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            pc = 0x63a4a;
        }
        0x63a4a => {
            // block 63a4a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a4a movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63a4e mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63a51 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63a54 jne 0x63aab
            if fl.ne() { pc = 0x63aab; continue; }
            pc = 0x63a56;
        }
        0x63a56 => {
            // block 63a56
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a56 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63a59 jmp 0x63aaf
            pc = 0x63aaf; continue;
        }
        0x63a5b => {
            // block 63a5b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a5b mov rdx,QWORD PTR [rbp-0x88]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x63a62;
        }
        0x63a62 => {
            // block 63a62
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a62 movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63a66 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63a69 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63a6c jne 0x63acc
            if fl.ne() { pc = 0x63acc; continue; }
            pc = 0x63a6e;
        }
        0x63a6e => {
            // block 63a6e
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a6e shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63a71 jmp 0x63ad0
            pc = 0x63ad0; continue;
        }
        0x63a73 => {
            // block 63a73
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a73 mov rdi,QWORD PTR [rbp-0x98]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            pc = 0x63a7a;
        }
        0x63a7a => {
            // block 63a7a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a7a test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63a7c jne 0x63aed
            if fl.ne() { pc = 0x63aed; continue; }
            pc = 0x63a7e;
        }
        0x63a7e => {
            // block 63a7e
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a7e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63a81 jmp 0x63af1
            pc = 0x63af1; continue;
        }
        0x63a83 => {
            // block 63a83
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a83 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x63a87;
        }
        0x63a87 => {
            // block 63a87
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a87 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63a89 jne 0x63b22
            if fl.ne() { pc = 0x63b22; continue; }
            pc = 0x63a8f;
        }
        0x63a8f => {
            // block 63a8f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a8f inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63a92 jmp 0x63b26
            pc = 0x63b26; continue;
        }
        0x63a97 => {
            // block 63a97
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a97 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0x63a9b;
        }
        0x63a9b => {
            // block 63a9b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a9b test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63a9d jne 0x63b4f
            if fl.ne() { pc = 0x63b4f; continue; }
            pc = 0x63aa3;
        }
        0x63aa3 => {
            // block 63aa3
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63aa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63aa3 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63aa6 jmp 0x63b53
            pc = 0x63b53; continue;
        }
        0x63aab => {
            // block 63aab
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63aab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63aab mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x63aaf;
        }
        0x63aaf => {
            // block 63aaf
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63aaf cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 63ab2 jne 0x63c18
            if fl.ne() { pc = 0x63c18; continue; }
            pc = 0x63ab8;
        }
        0x63ab8 => {
            // block 63ab8
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ab8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ab8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63aba jne 0x63bd3
            if fl.ne() { pc = 0x63bd3; continue; }
            pc = 0x63ac0;
        }
        0x63ac0 => {
            // block 63ac0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ac0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ac0 lea rdi,[rbp-0xbf]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff41);
            // 63ac7 jmp 0x63bda
            pc = 0x63bda; continue;
        }
        0x63acc => {
            // block 63acc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63acc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63acc mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x63ad0;
        }
        0x63ad0 => {
            // block 63ad0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ad0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ad0 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 63ad3 jne 0x63c45
            if fl.ne() { pc = 0x63c45; continue; }
            pc = 0x63ad9;
        }
        0x63ad9 => {
            // block 63ad9
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ad9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ad9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63adb jne 0x63be3
            if fl.ne() { pc = 0x63be3; continue; }
            pc = 0x63ae1;
        }
        0x63ae1 => {
            // block 63ae1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ae1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ae1 lea rdi,[rbp-0x8f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff71);
            // 63ae8 jmp 0x63be7
            pc = 0x63be7; continue;
        }
        0x63aed => {
            // block 63aed
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63aed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63aed mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63af1;
        }
        0x63af1 => {
            // block 63af1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63af1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63af1 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63af4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63af6 jne 0x63b7c
            if fl.ne() { pc = 0x63b7c; continue; }
            pc = 0x63afc;
        }
        0x63afc => {
            // block 63afc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63afc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63afc test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63aff je 0x63b8f
            if fl.e() { pc = 0x63b8f; continue; }
            pc = 0x63b05;
        }
        0x63b05 => {
            // block 63b05
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b05 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63b09 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63b0c cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63b0e jne 0x63b1d
            if fl.ne() { pc = 0x63b1d; continue; }
            pc = 0x63b10;
        }
        0x63b10 => {
            // block 63b10
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b10 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63b13 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63b16 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63b19 jne 0x63b05
            if fl.ne() { pc = 0x63b05; continue; }
            pc = 0x63b1b;
        }
        0x63b1b => {
            // block 63b1b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b1b jmp 0x63b8f
            pc = 0x63b8f; continue;
        }
        0x63b1d => {
            // block 63b1d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b1d xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63b20 jmp 0x63b8f
            pc = 0x63b8f; continue;
        }
        0x63b22 => {
            // block 63b22
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b22 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63b26;
        }
        0x63b26 => {
            // block 63b26
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b26 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63b29 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63b2b jne 0x63b9b
            if fl.ne() { pc = 0x63b9b; continue; }
            pc = 0x63b2d;
        }
        0x63b2d => {
            // block 63b2d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b2d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b30 je 0x63bae
            if fl.e() { pc = 0x63bae; continue; }
            pc = 0x63b32;
        }
        0x63b32 => {
            // block 63b32
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b32 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63b36 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63b39 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63b3b jne 0x63b4a
            if fl.ne() { pc = 0x63b4a; continue; }
            pc = 0x63b3d;
        }
        0x63b3d => {
            // block 63b3d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b3d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63b40 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63b43 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63b46 jne 0x63b32
            if fl.ne() { pc = 0x63b32; continue; }
            pc = 0x63b48;
        }
        0x63b48 => {
            // block 63b48
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b48 jmp 0x63bae
            pc = 0x63bae; continue;
        }
        0x63b4a => {
            // block 63b4a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b4a xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63b4d jmp 0x63bae
            pc = 0x63bae; continue;
        }
        0x63b4f => {
            // block 63b4f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b4f mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63b53;
        }
        0x63b53 => {
            // block 63b53
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b53 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63b56 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63b58 jne 0x63bb7
            if fl.ne() { pc = 0x63bb7; continue; }
            pc = 0x63b5a;
        }
        0x63b5a => {
            // block 63b5a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b5a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b5d je 0x63bca
            if fl.e() { pc = 0x63bca; continue; }
            pc = 0x63b5f;
        }
        0x63b5f => {
            // block 63b5f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b5f movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63b63 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63b66 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63b68 jne 0x63b77
            if fl.ne() { pc = 0x63b77; continue; }
            pc = 0x63b6a;
        }
        0x63b6a => {
            // block 63b6a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b6a inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63b6d inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63b70 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63b73 jne 0x63b5f
            if fl.ne() { pc = 0x63b5f; continue; }
            pc = 0x63b75;
        }
        0x63b75 => {
            // block 63b75
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b75 jmp 0x63bca
            pc = 0x63bca; continue;
        }
        0x63b77 => {
            // block 63b77
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b77 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63b7a jmp 0x63bca
            pc = 0x63bca; continue;
        }
        0x63b7c => {
            // block 63b7c
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b7c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b7f je 0x63b8f
            if fl.e() { pc = 0x63b8f; continue; }
            pc = 0x63b81;
        }
        0x63b81 => {
            // block 63b81
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b81 mov rsi,r15
            rsi = r15;
            // 63b84 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63b89 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63b8b sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63b8f;
        }
        0x63b8f => {
            // block 63b8f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b8f lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 63b96 jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63b9b => {
            // block 63b9b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b9b test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b9e je 0x63bae
            if fl.e() { pc = 0x63bae; continue; }
            pc = 0x63ba0;
        }
        0x63ba0 => {
            // block 63ba0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ba0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ba0 mov rsi,r15
            rsi = r15;
            // 63ba3 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63ba8 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63baa sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63bae;
        }
        0x63bae => {
            // block 63bae
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bae lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 63bb2 jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63bb7 => {
            // block 63bb7
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bb7 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63bba je 0x63bca
            if fl.e() { pc = 0x63bca; continue; }
            pc = 0x63bbc;
        }
        0x63bbc => {
            // block 63bbc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bbc mov rsi,r15
            rsi = r15;
            // 63bbf call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63bc4 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63bc6 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63bca;
        }
        0x63bca => {
            // block 63bca
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bca lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 63bce jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63bd3 => {
            // block 63bd3
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bd3 mov rdi,QWORD PTR [rbp-0xb0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            pc = 0x63bda;
        }
        0x63bda => {
            // block 63bda
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bda test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63bdc jne 0x63bf0
            if fl.ne() { pc = 0x63bf0; continue; }
            pc = 0x63bde;
        }
        0x63bde => {
            // block 63bde
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bde inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63be1 jmp 0x63bf4
            pc = 0x63bf4; continue;
        }
        0x63be3 => {
            // block 63be3
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63be3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63be3 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            pc = 0x63be7;
        }
        0x63be7 => {
            // block 63be7
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63be7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63be7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63be9 jne 0x63c1d
            if fl.ne() { pc = 0x63c1d; continue; }
            pc = 0x63beb;
        }
        0x63beb => {
            // block 63beb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63beb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63beb inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63bee jmp 0x63c21
            pc = 0x63c21; continue;
        }
        0x63bf0 => {
            // block 63bf0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bf0 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63bf4;
        }
        0x63bf4 => {
            // block 63bf4
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bf4 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63bf7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63bf9 jne 0x63c4a
            if fl.ne() { pc = 0x63c4a; continue; }
            pc = 0x63bfb;
        }
        0x63bfb => {
            // block 63bfb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bfb test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63bfe je 0x63c5d
            if fl.e() { pc = 0x63c5d; continue; }
            pc = 0x63c00;
        }
        0x63c00 => {
            // block 63c00
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c00 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63c04 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63c07 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63c09 jne 0x63c18
            if fl.ne() { pc = 0x63c18; continue; }
            pc = 0x63c0b;
        }
        0x63c0b => {
            // block 63c0b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c0b inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63c0e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63c11 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63c14 jne 0x63c00
            if fl.ne() { pc = 0x63c00; continue; }
            pc = 0x63c16;
        }
        0x63c16 => {
            // block 63c16
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c16 jmp 0x63c5d
            pc = 0x63c5d; continue;
        }
        0x63c18 => {
            // block 63c18
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c18 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63c1b jmp 0x63c5d
            pc = 0x63c5d; continue;
        }
        0x63c1d => {
            // block 63c1d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c1d mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63c21;
        }
        0x63c21 => {
            // block 63c21
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c21 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63c24 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63c26 jne 0x63c66
            if fl.ne() { pc = 0x63c66; continue; }
            pc = 0x63c28;
        }
        0x63c28 => {
            // block 63c28
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c28 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c2b je 0x63c79
            if fl.e() { pc = 0x63c79; continue; }
            pc = 0x63c2d;
        }
        0x63c2d => {
            // block 63c2d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c2d movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63c31 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63c34 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63c36 jne 0x63c45
            if fl.ne() { pc = 0x63c45; continue; }
            pc = 0x63c38;
        }
        0x63c38 => {
            // block 63c38
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c38 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63c3b inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63c3e dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63c41 jne 0x63c2d
            if fl.ne() { pc = 0x63c2d; continue; }
            pc = 0x63c43;
        }
        0x63c43 => {
            // block 63c43
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c43 jmp 0x63c79
            pc = 0x63c79; continue;
        }
        0x63c45 => {
            // block 63c45
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c45 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63c48 jmp 0x63c79
            pc = 0x63c79; continue;
        }
        0x63c4a => {
            // block 63c4a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c4a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c4d je 0x63c5d
            if fl.e() { pc = 0x63c5d; continue; }
            pc = 0x63c4f;
        }
        0x63c4f => {
            // block 63c4f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c4f mov rsi,r15
            rsi = r15;
            // 63c52 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c57 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63c59 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63c5d;
        }
        0x63c5d => {
            // block 63c5d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c5d lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 63c64 jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63c66 => {
            // block 63c66
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c66 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c69 je 0x63c79
            if fl.e() { pc = 0x63c79; continue; }
            pc = 0x63c6b;
        }
        0x63c6b => {
            // block 63c6b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c6b mov rsi,r15
            rsi = r15;
            // 63c6e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c73 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63c75 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63c79;
        }
        0x63c79 => {
            // block 63c79
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c79 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            pc = 0x63c80;
        }
        0x63c80 => {
            // block 63c80
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c80 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x63c85;
        }
        0x63c85 => {
            // block 63c85
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c85 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 63c89 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c8e mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 63c91 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // 63c98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 63c99 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 63c9b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 63c9d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 63c9e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x63c9f => {
            // block 63c9f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c9f mov rbx,rax
            rbx = rax;
            // 63ca2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 63ca6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63cab mov rdi,rbx
            rdi = rbx;
            // 63cae call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63cb3 size=252
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00063cb3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x63cb3;
    loop {
        match pc {
        0x63cb3 => {
            // block 63cb3
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63cb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63cb3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63cb4 mov rbp,rsp
            rbp = rsp;
            // 63cb7 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 63cb9 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 63cbb push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 63cbd push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 63cbf push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 63cc0 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 63cc4 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 63cc8 mov al,BYTE PTR [rip+0x1645ea] # 0x1c82b8
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(903813)) as u64) as u8 as u64);
            // 63cce test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63cd0 jne 0x63d2c
            if fl.ne() { pc = 0x63d2c; continue; }
            pc = 0x63cd2;
        }
        0x63cd2 => {
            // block 63cd2
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63cd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63cd2 lea rdi,[rip+0x1645df] # 0x1c82b8
            rdi = m.env.data_va(903813);
            // 63cd9 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63cde test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63ce0 je 0x63d2c
            if fl.e() { pc = 0x63d2c; continue; }
            pc = 0x63ce2;
        }
        0x63ce2 => {
            // block 63ce2
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63ce2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ce2 mov QWORD PTR [rip+0x1645c3],0x0 # 0x1c82b0
            m.w64(m.env.data_va(903805), (0x0u64) as u64);
            // 63ced mov QWORD PTR [rip+0x1645b0],0x0 # 0x1c82a8
            m.w64(m.env.data_va(903797), (0x0u64) as u64);
            // 63cf8 lea rsi,[rip+0x1645a1] # 0x1c82a0
            rsi = m.env.data_va(903789);
            // 63cff lea rax,[rip+0x1645a2] # 0x1c82a8
            rax = m.env.data_va(903797);
            // 63d06 mov QWORD PTR [rip+0x164593],rax # 0x1c82a0
            m.w64(m.env.data_va(903789), (rax) as u64);
            // 63d0d lea rdi,[rip+0xfffffffffffb0fc2] # 0x14cd6
            rdi = m.env.code_va(0x14cd6);
            // 63d14 lea rdx,[rip+0xfffffffffff9b455] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // 63d1b call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d20 lea rdi,[rip+0x164591] # 0x1c82b8
            rdi = m.env.data_va(903813);
            // 63d27 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x63d2c;
        }
        0x63d2c => {
            // block 63d2c
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63d2c xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 63d2f cmp QWORD PTR [rip+0x164579],0x0 # 0x1c82b0
            fl = sub::<64>(m.r64(m.env.data_va(903805)) as u64, 0x0u64).1;
            // 63d37 jne 0x63d7a
            if fl.ne() { pc = 0x63d7a; continue; }
            pc = 0x63d39;
        }
        0x63d39 => {
            // block 63d39
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63d39 lea r14,[rip+0x9ce30] # 0x100b70
            r14 = m.env.data_va(86845);
            // 63d40 lea r12,[rbp-0x40]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 63d44 lea r15,[rip+0x164555] # 0x1c82a0
            r15 = m.env.data_va(903789);
            pc = 0x63d4b;
        }
        0x63d4b => {
            // block 63d4b
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63d4b mov ebx,DWORD PTR [r14]
            rbx = (m.r32(r14) as u64) as u32 as u64;
            // 63d4e mov rdi,r12
            rdi = r12;
            // 63d51 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 63d53 call 0x633ff
            { let r = { let t = m.env.code_va(0x633ff); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d58 mov rdi,r15
            rdi = r15;
            // 63d5b mov rsi,r12
            rsi = r12;
            // 63d5e call 0xebb9a
            { let r = { let t = m.env.stub_va(0x367); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d63 mov DWORD PTR [rax],ebx
            m.w32(rax, ((rbx as u32 as u64)) as u32);
            // 63d65 mov rdi,r12
            rdi = r12;
            // 63d68 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d6d inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // 63d70 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 63d74 cmp r13,0x10
            fl = sub::<64>(r13, 0x10u64).1;
            // 63d78 jb 0x63d4b
            if fl.b() { pc = 0x63d4b; continue; }
            pc = 0x63d7a;
        }
        0x63d7a => {
            // block 63d7a
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63d7a lea rdi,[rip+0x16451f] # 0x1c82a0
            rdi = m.env.data_va(903789);
            // 63d81 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 63d85 call 0xebba0
            { let r = { let t = m.env.stub_va(0x36d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d8a mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 63d8c add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 63d90 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 63d91 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 63d93 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 63d95 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 63d97 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 63d99 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 63d9a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x63d9b => {
            // block 63d9b
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63d9b mov rbx,rax
            rbx = rax;
            // 63d9e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 63da2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63da7 mov rdi,rbx
            rdi = rbx;
            // 63daa call 0xec182
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

/// parity: P2v @0xe171e size=592
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
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
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x6cd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x6c7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18a9 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e18b0 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe18cc;
        }
        0xe18cc => {
            // block e18cc
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18cc lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e18d3 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18d8 mov rdi,rbx
            rdi = rbx;
            // e18db call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
        0xe18e0 => {
            // block e18e0
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18e0 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // e18e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1914 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e191b call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1920 mov rsi,QWORD PTR [rip+0x5e8d9] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // e1927 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e192b lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // e1932 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1937 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e193e call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
