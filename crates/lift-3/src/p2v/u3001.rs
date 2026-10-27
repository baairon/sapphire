//! Mechanically lifted functions (scripts/lift_run.py), module u3001. Each one is checked against the original
//! in the world harness (tests/world_u3001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x1016e size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001016e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1016e;
    loop {
        match pc {
        0x1016e => {
            // block 1016e
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 1016e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1016e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1016f mov rbp,rsp
            rbp = rsp;
            // 10172 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10174 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10175 mov r14,rsi
            r14 = rsi;
            // 10178 mov rbx,rdi
            rbx = rdi;
            // 1017b mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1017f mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 10183 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 10185 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 10188 je 0x10194
            if fl.e() { pc = 0x10194; continue; }
            pc = 0x1018a;
        }
        0x1018a => {
            // block 1018a
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 1018a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1018a shl rcx,0x7
            { let (r, f) = shl::<64>(rcx, 0x7u64, fl); rcx = r; fl = f; }
            // 1018e dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // 10191 mov rsi,rcx
            rsi = rcx;
            pc = 0x10194;
        }
        0x10194 => {
            // block 10194
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 10194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10194 mov rdx,QWORD PTR [rbx+0x20]
            rdx = m.r64(rbx.wrapping_add(0x20)) as u64;
            // 10198 mov rcx,QWORD PTR [rbx+0x28]
            rcx = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 1019c sub rsi,rdx
            { let (r, f) = sub::<64>(rsi, rdx);
              rsi = r;
              fl = f; }
            // 1019f cmp rsi,rcx
            fl = sub::<64>(rsi, rcx).1;
            // 101a2 jne 0x101b8
            if fl.ne() { pc = 0x101b8; continue; }
            pc = 0x101a4;
        }
        0x101a4 => {
            // block 101a4
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 101a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 101a4 mov rdi,rbx
            rdi = rbx;
            // 101a7 call 0xeb876
            { let r = { let t = m.env.stub_va(0x43); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 101ac mov rcx,QWORD PTR [rbx+0x28]
            rcx = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 101b0 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 101b4 mov rdx,QWORD PTR [rbx+0x20]
            rdx = m.r64(rbx.wrapping_add(0x20)) as u64;
            pc = 0x101b8;
        }
        0x101b8 => {
            // block 101b8
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 101b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 101b8 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 101bb mov rsi,rdx
            rsi = rdx;
            // 101be shr rsi,0xa
            { let (r, f) = shr::<64>(rsi, 0xau64, fl); rsi = r; fl = f; }
            // 101c2 mov rax,QWORD PTR [rax+rsi*8]
            rax = m.r64(rax.wrapping_add(rsi.wrapping_mul(8))) as u64;
            // 101c6 and rdx,0x3ff
            { let r = rdx & 0x3ffu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 101cd mov esi,DWORD PTR [r14]
            rsi = (m.r32(r14) as u64) as u32 as u64;
            // 101d0 mov DWORD PTR [rax+rdx*4],esi
            m.w32(rax.wrapping_add(rdx.wrapping_mul(4)), ((rsi as u32 as u64)) as u32);
            // 101d3 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 101d6 mov QWORD PTR [rbx+0x28],rcx
            m.w64(rbx.wrapping_add(0x28), (rcx) as u64);
            // 101da pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 101db pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 101dd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 101de ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2e82 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2e82<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2e82;
    loop {
        match pc {
        0xe2e82 => {
            // block e2e82
            #[cfg(feature = "trace")] eprintln!("f_000e2e82 pc e2e82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2e82 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2e83 mov rbp,rsp
            rbp = rsp;
            // e2e86 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2e88 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2e8a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2e8c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2e8d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2e91 mov r14,rcx
            r14 = rcx;
            // e2e94 mov r15,rdx
            r15 = rdx;
            // e2e97 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // e2e9a mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2e9d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2ea0 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // e2ea4 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2ea6 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2eab mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2eb0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2eb3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2eb6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2eb8 je 0xe2ee8
            if fl.e() { pc = 0xe2ee8; continue; }
            pc = 0xe2eba;
        }
        0xe2eba => {
            // block e2eba
            #[cfg(feature = "trace")] eprintln!("f_000e2e82 pc e2eba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2eba mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // e2ebd shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e2ec0 add r12d,r12d
            { let (r, f) = add::<32>((r12 as u32 as u64), (r12 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // e2ec3 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // e2ec8 mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2ecb shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e2ecd and edx,eax
            { let r = (rdx as u32 as u64) & (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e2ecf mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2ed2 shr edx,cl
            { let (r, f) = shr::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e2ed4 mov DWORD PTR [rbp-0x24],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rdx as u32 as u64)) as u32);
            // e2ed7 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // e2ed9 shr al,1
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x1u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // e2edb and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2edd mov BYTE PTR [r15],al
            m.w8(r15, ((rax as u8 as u64)) as u8);
            // e2ee0 and dl,0x1
            { let r = (rdx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2ee3 mov BYTE PTR [r14],dl
            m.w8(r14, ((rdx as u8 as u64)) as u8);
            // e2ee6 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe2ee8;
        }
        0xe2ee8 => {
            // block e2ee8
            #[cfg(feature = "trace")] eprintln!("f_000e2e82 pc e2ee8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2ee8 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2eea add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2eee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2eef pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2ef1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2ef3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2ef5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2ef6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe494e size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e494e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe494e;
    loop {
        match pc {
        0xe494e => {
            // block e494e
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e494e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e494e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e494f mov rbp,rsp
            rbp = rsp;
            // e4952 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4954 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4956 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4957 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e4958 mov r15,rdi
            r15 = rdi;
            // e495b mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e495e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4961 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e4965 mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // e496a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e496f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4972 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4975 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e4978 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // e497b test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e497d je 0xe49b6
            if fl.e() { pc = 0xe49b6; continue; }
            pc = 0xe497f;
        }
        0xe497f => {
            // block e497f
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e497f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e497f mov ebx,0xffffffec
            rbx = (0xffffffecu64) as u32 as u64;
            pc = 0xe4984;
        }
        0xe4984 => {
            // block e4984
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e4984 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4984 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e4987 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e498a call QWORD PTR [rax+0x1880]
            { let r = { let t = m.r64(rax.wrapping_add(0x1880)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4990 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4992 jne 0xe49b6
            if fl.ne() { pc = 0xe49b6; continue; }
            pc = 0xe4994;
        }
        0xe4994 => {
            // block e4994
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e4994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4994 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e4997 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e499a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e499c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e49a1 call QWORD PTR [rax+0xbd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e49a7 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // e49a9 jne 0xe4984
            if fl.ne() { pc = 0xe4984; continue; }
            pc = 0xe49ab;
        }
        0xe49ab => {
            // block e49ab
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e49ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e49ab mov DWORD PTR [r15+0x8],0x16
            m.w32(r15.wrapping_add(0x8), (0x16u64) as u32);
            // e49b3 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe49b6;
        }
        0xe49b6 => {
            // block e49b6
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e49b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e49b6 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // e49b9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e49bd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e49be pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e49c0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e49c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e49c3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbb45 size=119
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbb45<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbb45;
    loop {
        match pc {
        0xbbb45 => {
            // block bbb45
            #[cfg(feature = "trace")] eprintln!("f_000bbb45 pc bbb45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb45 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbb46 mov rbp,rsp
            rbp = rsp;
            // bbb49 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bbb4a push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bbb4b mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // bbb4d cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // bbb50 jb 0xbbb83
            if fl.b() { pc = 0xbbb83; continue; }
            pc = 0xbbb52;
        }
        0xbbb52 => {
            // block bbb52
            #[cfg(feature = "trace")] eprintln!("f_000bbb45 pc bbb52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb52 cmp edi,0x9
            fl = sub::<32>((rdi as u32 as u64), 0x9u64).1;
            // bbb55 jg 0xbbb83
            if fl.g() { pc = 0xbbb83; continue; }
            pc = 0xbbb57;
        }
        0xbbb57 => {
            // block bbb57
            #[cfg(feature = "trace")] eprintln!("f_000bbb45 pc bbb57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb57 call 0x5c801
            { let r = { let t = m.env.code_va(0x5c801); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbb5c mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // bbb5e mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // bbb63 cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // bbb66 ja 0xbbbb5
            if fl.a() { pc = 0xbbbb5; continue; }
            pc = 0xbbb68;
        }
        0xbbb68 => {
            // block bbb68
            #[cfg(feature = "trace")] eprintln!("f_000bbb45 pc bbb68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb68 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // bbb6a lea rcx,[rip+0x470af] # 0x102c20
            rcx = m.env.data_va(95213);
            // bbb71 lea rdx,[rip+0x47088] # 0x102c00
            rdx = m.env.data_va(95181);
            // bbb78 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // bbb7a cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbb7e mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // bbb81 jmp 0xbbbb5
            pc = 0xbbbb5; continue;
        }
        0xbbb83 => {
            // block bbb83
            #[cfg(feature = "trace")] eprintln!("f_000bbb45 pc bbb83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb83 mov eax,0x16
            rax = (0x16u64) as u32 as u64;
            // bbb88 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // bbb8a je 0xbbbb5
            if fl.e() { pc = 0xbbbb5; continue; }
            pc = 0xbbb8c;
        }
        0xbbb8c => {
            // block bbb8c
            #[cfg(feature = "trace")] eprintln!("f_000bbb45 pc bbb8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb8c mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // bbb91 cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // bbb94 jne 0xbbbb5
            if fl.ne() { pc = 0xbbbb5; continue; }
            pc = 0xbbb96;
        }
        0xbbb96 => {
            // block bbb96
            #[cfg(feature = "trace")] eprintln!("f_000bbb45 pc bbb96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb96 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // bbb99 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // bbb9c ja 0xbbbb5
            if fl.a() { pc = 0xbbbb5; continue; }
            pc = 0xbbb9e;
        }
        0xbbb9e => {
            // block bbb9e
            #[cfg(feature = "trace")] eprintln!("f_000bbb45 pc bbb9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbb9e lea rax,[rip+0x4704b] # 0x102bf0
            rax = m.env.data_va(95165);
            // bbba5 lea rsi,[rip+0x47034] # 0x102be0
            rsi = m.env.data_va(95149);
            // bbbac test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // bbbae cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // bbbb2 mov eax,DWORD PTR [rsi+rcx*4]
            rax = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbbb5;
        }
        0xbbbb5 => {
            // block bbbb5
            #[cfg(feature = "trace")] eprintln!("f_000bbb45 pc bbbb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbbb5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bbbb9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bbbba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbbbb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe774 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e774<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe774;
    loop {
        match pc {
        0xe774 => {
            // block e774
            #[cfg(feature = "trace")] eprintln!("f_0000e774 pc e774 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e774 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e775 mov rbp,rsp
            rbp = rsp;
            // e778 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e77a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e77c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e77d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e77e mov r14,rdx
            r14 = rdx;
            // e781 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e784 mov rbx,rdi
            rbx = rdi;
            // e787 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // e78a call 0xcb584
            { let r = { let t = m.env.code_va(0xcb584); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e78f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e791 je 0xe7aa
            if fl.e() { pc = 0xe7aa; continue; }
            pc = 0xe793;
        }
        0xe793 => {
            // block e793
            #[cfg(feature = "trace")] eprintln!("f_0000e774 pc e793 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e793 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e796 movzx r15d,r15w
            r15 = ((r15 as u16 as u64)) as u32 as u64;
            // e79a mov rdi,rbx
            rdi = rbx;
            // e79d mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e7a0 call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7a6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e7a8 je 0xe7ae
            if fl.e() { pc = 0xe7ae; continue; }
            pc = 0xe7aa;
        }
        0xe7aa => {
            // block e7aa
            #[cfg(feature = "trace")] eprintln!("f_0000e774 pc e7aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7aa xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e7ac jmp 0xe7e0
            pc = 0xe7e0; continue;
        }
        0xe7ae => {
            // block e7ae
            #[cfg(feature = "trace")] eprintln!("f_0000e774 pc e7ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ae mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e7b5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e7b8 lea rcx,[rip+0xf0c01] # 0xff3c0
            rcx = m.env.data_va(80781);
            // e7bf mov esi,DWORD PTR [rcx+r15*4]
            rsi = (m.r32(rcx.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e7c3 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e7c7 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // e7cc mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // e7d2 mov rdi,rbx
            rdi = rbx;
            // e7d5 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7d8 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // e7dc setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0xe7e0;
        }
        0xe7e0 => {
            // block e7e0
            #[cfg(feature = "trace")] eprintln!("f_0000e774 pc e7e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7e0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e7e4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e7e5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e7e7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e7e9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7ea ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe854 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e854<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe854;
    loop {
        match pc {
        0xe854 => {
            // block e854
            #[cfg(feature = "trace")] eprintln!("f_0000e854 pc e854 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e854 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e855 mov rbp,rsp
            rbp = rsp;
            // e858 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e85a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e85c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e85d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e85e mov r14,rdx
            r14 = rdx;
            // e861 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e864 mov rbx,rdi
            rbx = rdi;
            // e867 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // e86a call 0xcb617
            { let r = { let t = m.env.code_va(0xcb617); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e86f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e871 je 0xe88a
            if fl.e() { pc = 0xe88a; continue; }
            pc = 0xe873;
        }
        0xe873 => {
            // block e873
            #[cfg(feature = "trace")] eprintln!("f_0000e854 pc e873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e873 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e876 movzx r15d,r15w
            r15 = ((r15 as u16 as u64)) as u32 as u64;
            // e87a mov rdi,rbx
            rdi = rbx;
            // e87d mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e880 call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e886 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e888 je 0xe88e
            if fl.e() { pc = 0xe88e; continue; }
            pc = 0xe88a;
        }
        0xe88a => {
            // block e88a
            #[cfg(feature = "trace")] eprintln!("f_0000e854 pc e88a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e88a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e88c jmp 0xe8c0
            pc = 0xe8c0; continue;
        }
        0xe88e => {
            // block e88e
            #[cfg(feature = "trace")] eprintln!("f_0000e854 pc e88e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e88e mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e895 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e898 lea rcx,[rip+0xf0b21] # 0xff3c0
            rcx = m.env.data_va(80781);
            // e89f mov esi,DWORD PTR [rcx+r15*4]
            rsi = (m.r32(rcx.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e8a3 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e8a7 mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // e8ac mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // e8b2 mov rdi,rbx
            rdi = rbx;
            // e8b5 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8b8 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // e8bc setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0xe8c0;
        }
        0xe8c0 => {
            // block e8c0
            #[cfg(feature = "trace")] eprintln!("f_0000e854 pc e8c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8c0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e8c4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8c5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e8c7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e8c9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e8ca ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10228 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010228<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10228;
    loop {
        match pc {
        0x10228 => {
            // block 10228
            #[cfg(feature = "trace")] eprintln!("f_00010228 pc 10228 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10228 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10229 mov rbp,rsp
            rbp = rsp;
            // 1022c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1022e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10230 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10232 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10233 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 10237 mov r15,rsi
            r15 = rsi;
            // 1023a mov r14,rdi
            r14 = rdi;
            // 1023d lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 10241 mov rdx,r15
            rdx = r15;
            // 10244 call 0xebab0
            { let r = { let t = m.env.stub_va(0x27d); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10249 mov r12,rax
            r12 = rax;
            // 1024c mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 10250 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 10253 jne 0x10281
            if fl.ne() { pc = 0x10281; continue; }
            pc = 0x10255;
        }
        0x10255 => {
            // block 10255
            #[cfg(feature = "trace")] eprintln!("f_00010228 pc 10255 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10255 mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // 1025a call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1025f mov rbx,rax
            rbx = rax;
            // 10262 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 10265 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 10268 mov DWORD PTR [rbx+0x20],0x0
            m.w32(rbx.wrapping_add(0x20), (0x0u64) as u32);
            // 1026f mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 10273 mov rdi,r14
            rdi = r14;
            // 10276 mov rdx,r12
            rdx = r12;
            // 10279 mov rcx,rbx
            rcx = rbx;
            // 1027c call 0xeb88e
            { let r = { let t = m.env.stub_va(0x5b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x10281;
        }
        0x10281 => {
            // block 10281
            #[cfg(feature = "trace")] eprintln!("f_00010228 pc 10281 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10281 add rbx,0x20
            { let (r, f) = add::<64>(rbx, 0x20u64);
              rbx = r;
              fl = f; }
            // 10285 mov rax,rbx
            rax = rbx;
            // 10288 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 1028c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1028d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1028f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10291 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10293 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10294 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d310 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d310<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2d310;
    loop {
        match pc {
        0x2d310 => {
            // block 2d310
            #[cfg(feature = "trace")] eprintln!("f_0002d310 pc 2d310 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d310 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d311 mov rbp,rsp
            rbp = rsp;
            // 2d314 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d316 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d318 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d319 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2d31a mov rbx,rdi
            rbx = rdi;
            // 2d31d mov rax,QWORD PTR [rip+0x11309c] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 2d324 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 2d328 mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // 2d32b lea r14,[rbx+0x70]
            r14 = rbx.wrapping_add(0x70);
            // 2d32f add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 2d333 mov QWORD PTR [rbx+0x70],rax
            m.w64(rbx.wrapping_add(0x70), (rax) as u64);
            // 2d337 lea r15,[rbx+0x8]
            r15 = rbx.wrapping_add(0x8);
            // 2d33b mov rax,QWORD PTR [rip+0x113066] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 2d342 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 2d346 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 2d34a lea rdi,[rbx+0x48]
            rdi = rbx.wrapping_add(0x48);
            // 2d34e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d353 mov rdi,r15
            rdi = r15;
            // 2d356 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d35b mov rsi,QWORD PTR [rip+0x112ea6] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 2d362 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 2d366 mov rdi,rbx
            rdi = rbx;
            // 2d369 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d36e mov rdi,r14
            rdi = r14;
            // 2d371 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d376 mov rdi,rbx
            rdi = rbx;
            // 2d379 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2d37d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d37e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d380 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d382 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d383 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbfc0 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbfc0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbfc0;
    loop {
        match pc {
        0xbbfc0 => {
            // block bbfc0
            #[cfg(feature = "trace")] eprintln!("f_000bbfc0 pc bbfc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbfc0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbfc1 mov rbp,rsp
            rbp = rsp;
            // bbfc4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bbfc5 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bbfc9 mov rax,rdi
            rax = rdi;
            // bbfcc lea rbx,[rbp-0x38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bbfd0 mov rdi,rbx
            rdi = rbx;
            // bbfd3 mov rsi,rax
            rsi = rax;
            // bbfd6 call 0xbbe30
            { let r = { let t = m.env.code_va(0xbbe30); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbfdb lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // bbfdf mov rsi,rbx
            rsi = rbx;
            // bbfe2 call 0xbbdbe
            { let r = { let t = m.env.code_va(0xbbdbe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbfe7 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bbfeb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbff0 lea rdi,[rip+0x10c649] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bbff7 lea rsi,[rbp-0x20]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // bbffb call 0xebe82
            { let r = { let t = m.env.stub_va(0x64f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc000 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // bc002 lea rcx,[rip+0x10c63f] # 0x1c8648
            rcx = m.env.data_va(904725);
            // bc009 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bc00c je 0xbc011
            if fl.e() { pc = 0xbc011; continue; }
            pc = 0xbc00e;
        }
        0xbc00e => {
            // block bc00e
            #[cfg(feature = "trace")] eprintln!("f_000bbfc0 pc bc00e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc00e mov ebx,DWORD PTR [rax+0x38]
            rbx = (m.r32(rax.wrapping_add(0x38)) as u64) as u32 as u64;
            pc = 0xbc011;
        }
        0xbc011 => {
            // block bc011
            #[cfg(feature = "trace")] eprintln!("f_000bbfc0 pc bc011 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc011 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // bc015 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc01a mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // bc01c add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bc020 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc021 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc022 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbc023 => {
            // block bc023
            #[cfg(feature = "trace")] eprintln!("f_000bbfc0 pc bc023 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc023 mov rbx,rax
            rbx = rax;
            // bc026 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bc02a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc02f mov rdi,rbx
            rdi = rbx;
            // bc032 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd9f72 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d9f72<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd9f72;
    loop {
        match pc {
        0xd9f72 => {
            // block d9f72
            #[cfg(feature = "trace")] eprintln!("f_000d9f72 pc d9f72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9f72 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d9f73 mov rbp,rsp
            rbp = rsp;
            // d9f76 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d9f78 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d9f7a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d9f7b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d9f7f mov r14,rsi
            r14 = rsi;
            // d9f82 mov rbx,rdi
            rbx = rdi;
            // d9f85 lea rdi,[rip+0x5f0b4] # 0x139040
            rdi = m.env.data_va(317453);
            // d9f8c call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9f91 add rbx,0x380
            { let (r, f) = add::<64>(rbx, 0x380u64);
              rbx = r;
              fl = f; }
            // d9f98 mov rdi,r14
            rdi = r14;
            // d9f9b call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fa0 lea r15,[rbp-0x30]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d9fa4 mov rdi,r15
            rdi = r15;
            // d9fa7 mov rsi,r14
            rsi = r14;
            // d9faa mov rdx,rax
            rdx = rax;
            // d9fad call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fb2 mov rdi,rbx
            rdi = rbx;
            // d9fb5 mov rsi,r15
            rsi = r15;
            // d9fb8 call 0xe8100
            { let r = { let t = m.env.code_va(0xe8100); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fbd mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d9fbf lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d9fc3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fc8 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d9fca add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d9fce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d9fcf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d9fd1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d9fd3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d9fd4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd9fd5 => {
            // block d9fd5
            #[cfg(feature = "trace")] eprintln!("f_000d9f72 pc d9fd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9fd5 mov rbx,rax
            rbx = rax;
            // d9fd8 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d9fdc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fe1 mov rdi,rbx
            rdi = rbx;
            // d9fe4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe04ea size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e04ea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe04ea;
    loop {
        match pc {
        0xe04ea => {
            // block e04ea
            #[cfg(feature = "trace")] eprintln!("f_000e04ea pc e04ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e04ea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e04eb mov rbp,rsp
            rbp = rsp;
            // e04ee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e04f0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e04f2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e04f4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e04f5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e04f9 movsd QWORD PTR [rbp-0x28],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), x1 as u64);
            // e04fe movsd QWORD PTR [rbp-0x30],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0 as u64);
            // e0503 mov r14d,r9d
            r14 = ((r9 as u32 as u64)) as u32 as u64;
            // e0506 mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // e0509 mov r15,rcx
            r15 = rcx;
            // e050c mov rbx,rdi
            rbx = rdi;
            // e050f lea rax,[rip+0x65cb2] # 0x1461c8
            rax = m.env.data_va(371093);
            // e0516 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e0519 mov QWORD PTR [rbx+0x8],rsi
            m.w64(rbx.wrapping_add(0x8), (rsi) as u64);
            // e051d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e0520 lea rdi,[rip+0x4b6c1] # 0x12bbe8
            rdi = m.env.data_va(263093);
            // e0527 cmovne rdi,rdx
            if fl.ne() { rdi = rdx; }
            // e052b call 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // e0530 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e0534 mov DWORD PTR [rbx+0x18],r12d
            m.w32(rbx.wrapping_add(0x18), ((r12 as u32 as u64)) as u32);
            // e0538 mov DWORD PTR [rbx+0x1c],r14d
            m.w32(rbx.wrapping_add(0x1c), ((r14 as u32 as u64)) as u32);
            // e053c movsd xmm0,QWORD PTR [rbp-0x30]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u128;
            // e0541 movsd QWORD PTR [rbx+0x20],xmm0
            m.w64(rbx.wrapping_add(0x20), x0 as u64);
            // e0546 movsd xmm0,QWORD PTR [rbp-0x28]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u128;
            // e054b movsd QWORD PTR [rbx+0x28],xmm0
            m.w64(rbx.wrapping_add(0x28), x0 as u64);
            // e0550 mov QWORD PTR [rbx+0x30],r15
            m.w64(rbx.wrapping_add(0x30), (r15) as u64);
            // e0554 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0558 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0559 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e055b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e055d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e055f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0560 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0562 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0562<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0562;
    loop {
        match pc {
        0xe0562 => {
            // block e0562
            #[cfg(feature = "trace")] eprintln!("f_000e0562 pc e0562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0562 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0563 mov rbp,rsp
            rbp = rsp;
            // e0566 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e0568 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e056a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e056c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e056d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0571 movsd QWORD PTR [rbp-0x28],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), x1 as u64);
            // e0576 movsd QWORD PTR [rbp-0x30],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0 as u64);
            // e057b mov r14d,r9d
            r14 = ((r9 as u32 as u64)) as u32 as u64;
            // e057e mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // e0581 mov r15,rcx
            r15 = rcx;
            // e0584 mov rbx,rdi
            rbx = rdi;
            // e0587 lea rax,[rip+0x65c3a] # 0x1461c8
            rax = m.env.data_va(371093);
            // e058e mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e0591 mov QWORD PTR [rbx+0x8],rsi
            m.w64(rbx.wrapping_add(0x8), (rsi) as u64);
            // e0595 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e0598 lea rdi,[rip+0x4b649] # 0x12bbe8
            rdi = m.env.data_va(263093);
            // e059f cmovne rdi,rdx
            if fl.ne() { rdi = rdx; }
            // e05a3 call 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // e05a8 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e05ac mov DWORD PTR [rbx+0x18],r12d
            m.w32(rbx.wrapping_add(0x18), ((r12 as u32 as u64)) as u32);
            // e05b0 mov DWORD PTR [rbx+0x1c],r14d
            m.w32(rbx.wrapping_add(0x1c), ((r14 as u32 as u64)) as u32);
            // e05b4 movsd xmm0,QWORD PTR [rbp-0x30]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u128;
            // e05b9 movsd QWORD PTR [rbx+0x20],xmm0
            m.w64(rbx.wrapping_add(0x20), x0 as u64);
            // e05be movsd xmm0,QWORD PTR [rbp-0x28]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u128;
            // e05c3 movsd QWORD PTR [rbx+0x28],xmm0
            m.w64(rbx.wrapping_add(0x28), x0 as u64);
            // e05c8 mov QWORD PTR [rbx+0x30],r15
            m.w64(rbx.wrapping_add(0x30), (r15) as u64);
            // e05cc add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e05d0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e05d1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e05d3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e05d5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e05d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e05d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x59a2 size=122
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000059a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x59a2;
    loop {
        match pc {
        0x59a2 => {
            // block 59a2
            #[cfg(feature = "trace")] eprintln!("f_000059a2 pc 59a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 59a3 mov rbp,rsp
            rbp = rsp;
            // 59a6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 59a8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 59aa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 59ab push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 59ac mov r14,rdx
            r14 = rdx;
            // 59af mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 59b2 mov rbx,rdi
            rbx = rdi;
            // 59b5 mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // 59bc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 59bf call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59c5 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 59c7 call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59cc movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 59cf cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 59d2 jbe 0x5a0f
            if fl.be() { pc = 0x5a0f; continue; }
            pc = 0x59d4;
        }
        0x59d4 => {
            // block 59d4
            #[cfg(feature = "trace")] eprintln!("f_000059a2 pc 59d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59d4 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 59db mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 59de movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 59e2 lea rdx,[rip+0xf9997] # 0xff380
            rdx = m.env.data_va(80717);
            // 59e9 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 59ec lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 59f0 mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 59f5 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 59fb mov rdi,rbx
            rdi = rbx;
            // 59fe call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5a01 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5a03 je 0x5a0f
            if fl.e() { pc = 0x5a0f; continue; }
            pc = 0x5a05;
        }
        0x5a05 => {
            // block 5a05
            #[cfg(feature = "trace")] eprintln!("f_000059a2 pc 5a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a05 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 5a08 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 5a0b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 5a0d jmp 0x5a11
            pc = 0x5a11; continue;
        }
        0x5a0f => {
            // block 5a0f
            #[cfg(feature = "trace")] eprintln!("f_000059a2 pc 5a0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a0f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5a11;
        }
        0x5a11 => {
            // block 5a11
            #[cfg(feature = "trace")] eprintln!("f_000059a2 pc 5a11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a11 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5a15 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5a16 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5a18 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5a1a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a1b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a7c size=122
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005a7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5a7c;
    loop {
        match pc {
        0x5a7c => {
            // block 5a7c
            #[cfg(feature = "trace")] eprintln!("f_00005a7c pc 5a7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a7d mov rbp,rsp
            rbp = rsp;
            // 5a80 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5a82 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5a84 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5a85 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5a86 mov r14,rdx
            r14 = rdx;
            // 5a89 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5a8c mov rbx,rdi
            rbx = rdi;
            // 5a8f mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // 5a96 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5a99 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5a9f mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5aa1 call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5aa6 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 5aa9 cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 5aac jbe 0x5ae9
            if fl.be() { pc = 0x5ae9; continue; }
            pc = 0x5aae;
        }
        0x5aae => {
            // block 5aae
            #[cfg(feature = "trace")] eprintln!("f_00005a7c pc 5aae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aae mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 5ab5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5ab8 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 5abc lea rdx,[rip+0xf98bd] # 0xff380
            rdx = m.env.data_va(80717);
            // 5ac3 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5ac6 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 5aca mov ecx,0xc00000
            rcx = (0xc00000u64) as u32 as u64;
            // 5acf mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 5ad5 mov rdi,rbx
            rdi = rbx;
            // 5ad8 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5adb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5add je 0x5ae9
            if fl.e() { pc = 0x5ae9; continue; }
            pc = 0x5adf;
        }
        0x5adf => {
            // block 5adf
            #[cfg(feature = "trace")] eprintln!("f_00005a7c pc 5adf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5adf mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 5ae2 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 5ae5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 5ae7 jmp 0x5aeb
            pc = 0x5aeb; continue;
        }
        0x5ae9 => {
            // block 5ae9
            #[cfg(feature = "trace")] eprintln!("f_00005a7c pc 5ae9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ae9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5aeb;
        }
        0x5aeb => {
            // block 5aeb
            #[cfg(feature = "trace")] eprintln!("f_00005a7c pc 5aeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aeb add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5aef pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5af0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5af2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5af4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5af5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5b56 size=122
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005b56<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5b56;
    loop {
        match pc {
        0x5b56 => {
            // block 5b56
            #[cfg(feature = "trace")] eprintln!("f_00005b56 pc 5b56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b56 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5b57 mov rbp,rsp
            rbp = rsp;
            // 5b5a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5b5c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5b5e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5b5f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5b60 mov r14,rdx
            r14 = rdx;
            // 5b63 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5b66 mov rbx,rdi
            rbx = rdi;
            // 5b69 mov DWORD PTR [r14],0x4
            m.w32(r14, (0x4u64) as u32);
            // 5b70 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5b73 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b79 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5b7b call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b80 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 5b83 cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 5b86 jbe 0x5bc3
            if fl.be() { pc = 0x5bc3; continue; }
            pc = 0x5b88;
        }
        0x5b88 => {
            // block 5b88
            #[cfg(feature = "trace")] eprintln!("f_00005b56 pc 5b88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b88 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 5b8f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5b92 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 5b96 lea rdx,[rip+0xf97e3] # 0xff380
            rdx = m.env.data_va(80717);
            // 5b9d mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5ba0 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 5ba4 mov ecx,0x3000000
            rcx = (0x3000000u64) as u32 as u64;
            // 5ba9 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 5baf mov rdi,rbx
            rdi = rbx;
            // 5bb2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5bb5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5bb7 je 0x5bc3
            if fl.e() { pc = 0x5bc3; continue; }
            pc = 0x5bb9;
        }
        0x5bb9 => {
            // block 5bb9
            #[cfg(feature = "trace")] eprintln!("f_00005b56 pc 5bb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bb9 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 5bbc mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 5bbf mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 5bc1 jmp 0x5bc5
            pc = 0x5bc5; continue;
        }
        0x5bc3 => {
            // block 5bc3
            #[cfg(feature = "trace")] eprintln!("f_00005b56 pc 5bc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bc3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5bc5;
        }
        0x5bc5 => {
            // block 5bc5
            #[cfg(feature = "trace")] eprintln!("f_00005b56 pc 5bc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bc5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5bc9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5bca pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5bcc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5bce pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5bcf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeac6 size=122
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000eac6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeac6;
    loop {
        match pc {
        0xeac6 => {
            // block eac6
            #[cfg(feature = "trace")] eprintln!("f_0000eac6 pc eac6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eac7 mov rbp,rsp
            rbp = rsp;
            // eaca push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eacc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eace push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eacf push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ead0 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // ead2 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // ead5 mov r15,rdi
            r15 = rdi;
            // ead8 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // eadc call 0xd2f1a
            { let r = { let t = m.env.code_va(0xd2f1a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae1 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // eae3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eae5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // eae7 je 0xeb35
            if fl.e() { pc = 0xeb35; continue; }
            pc = 0xeae9;
        }
        0xeae9 => {
            // block eae9
            #[cfg(feature = "trace")] eprintln!("f_0000eac6 pc eae9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eae9 cmp ebx,0x2580
            fl = sub::<32>((rbx as u32 as u64), 0x2580u64).1;
            // eaef je 0xeb0d
            if fl.e() { pc = 0xeb0d; continue; }
            pc = 0xeaf1;
        }
        0xeaf1 => {
            // block eaf1
            #[cfg(feature = "trace")] eprintln!("f_0000eac6 pc eaf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf1 cmp ebx,0x9600
            fl = sub::<32>((rbx as u32 as u64), 0x9600u64).1;
            // eaf7 je 0xeb12
            if fl.e() { pc = 0xeb12; continue; }
            pc = 0xeaf9;
        }
        0xeaf9 => {
            // block eaf9
            #[cfg(feature = "trace")] eprintln!("f_0000eac6 pc eaf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf9 cmp ebx,0x4b00
            fl = sub::<32>((rbx as u32 as u64), 0x4b00u64).1;
            // eaff mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // eb04 jne 0xeb35
            if fl.ne() { pc = 0xeb35; continue; }
            pc = 0xeb06;
        }
        0xeb06 => {
            // block eb06
            #[cfg(feature = "trace")] eprintln!("f_0000eac6 pc eb06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb06 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // eb0b jmp 0xeb12
            pc = 0xeb12; continue;
        }
        0xeb0d => {
            // block eb0d
            #[cfg(feature = "trace")] eprintln!("f_0000eac6 pc eb0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb0d mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            pc = 0xeb12;
        }
        0xeb12 => {
            // block eb12
            #[cfg(feature = "trace")] eprintln!("f_0000eac6 pc eb12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb12 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // eb15 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // eb18 lea rdx,[rip+0xf0c35] # 0xff754
            rdx = m.env.data_va(81697);
            // eb1f mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // eb22 mov ecx,0x70000
            rcx = (0x70000u64) as u32 as u64;
            // eb27 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // eb2d mov rdi,r15
            rdi = r15;
            // eb30 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // eb32 call QWORD PTR [rbx+0x30]
            { let r = { let t = m.r64(rbx.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb35;
        }
        0xeb35 => {
            // block eb35
            #[cfg(feature = "trace")] eprintln!("f_0000eac6 pc eb35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb35 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // eb39 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb3a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb3c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb3e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb3f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x45032 size=122
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00045032<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x45032;
    loop {
        match pc {
        0x45032 => {
            // block 45032
            #[cfg(feature = "trace")] eprintln!("f_00045032 pc 45032 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 45032 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 45033 mov rbp,rsp
            rbp = rsp;
            // 45036 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 45038 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4503a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4503b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4503f mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // 45042 mov r14,rdx
            r14 = rdx;
            // 45045 mov r8d,esi
            r8 = ((rsi as u32 as u64)) as u32 as u64;
            // 45048 mov rbx,rdi
            rbx = rdi;
            // 4504b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4504d test r14,r14
            fl = logic::<64>(r14 & r14);
            // 45050 je 0x450a0
            if fl.e() { pc = 0x450a0; continue; }
            pc = 0x45052;
        }
        0x45052 => {
            // block 45052
            #[cfg(feature = "trace")] eprintln!("f_00045032 pc 45052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 45052 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 45055 je 0x450a0
            if fl.e() { pc = 0x450a0; continue; }
            pc = 0x45057;
        }
        0x45057 => {
            // block 45057
            #[cfg(feature = "trace")] eprintln!("f_00045032 pc 45057 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 45057 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 4505e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 45061 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 45065 mov rdi,rbx
            rdi = rbx;
            // 45068 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 4506a mov ecx,r8d
            rcx = ((r8 as u32 as u64)) as u32 as u64;
            // 4506d call QWORD PTR [rax+0x720]
            { let r = { let t = m.r64(rax.wrapping_add(0x720)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 45073 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 45075 je 0x4509e
            if fl.e() { pc = 0x4509e; continue; }
            pc = 0x45077;
        }
        0x45077 => {
            // block 45077
            #[cfg(feature = "trace")] eprintln!("f_00045032 pc 45077 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 45077 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4507a mov r9d,DWORD PTR [rbp-0x1c]
            r9 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 4507e mov DWORD PTR [rsp],r15d
            m.w32(rsp, ((r15 as u32 as u64)) as u32);
            // 45082 mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 4508a mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 4508f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 45091 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 45093 mov rdi,rbx
            rdi = rbx;
            // 45096 mov r8,r14
            r8 = r14;
            // 45099 call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4509c jmp 0x450a0
            pc = 0x450a0; continue;
        }
        0x4509e => {
            // block 4509e
            #[cfg(feature = "trace")] eprintln!("f_00045032 pc 4509e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4509e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x450a0;
        }
        0x450a0 => {
            // block 450a0
            #[cfg(feature = "trace")] eprintln!("f_00045032 pc 450a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 450a0 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 450a4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 450a5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 450a7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 450a9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 450aa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x46bf4 size=122
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00046bf4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x46bf4;
    loop {
        match pc {
        0x46bf4 => {
            // block 46bf4
            #[cfg(feature = "trace")] eprintln!("f_00046bf4 pc 46bf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46bf4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 46bf5 mov rbp,rsp
            rbp = rsp;
            // 46bf8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 46bfa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 46bfc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 46bfe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 46bff sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 46c03 mov r14,rdx
            r14 = rdx;
            // 46c06 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 46c09 mov rbx,rdi
            rbx = rdi;
            // 46c0c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 46c0f xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 46c12 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 46c14 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46c1a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 46c1c je 0x46c5d
            if fl.e() { pc = 0x46c5d; continue; }
            pc = 0x46c1e;
        }
        0x46c1e => {
            // block 46c1e
            #[cfg(feature = "trace")] eprintln!("f_00046bf4 pc 46c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46c1e mov QWORD PTR [rbp-0x28],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u64);
            // 46c26 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 46c29 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 46c2b mov rdi,rbx
            rdi = rbx;
            // 46c2e call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46c34 mov rcx,QWORD PTR [rip+0xf97b5] # 0x1403f0
            rcx = m.r64(m.env.data_va(347069)) as u64;
            // 46c3b mov edx,DWORD PTR [rcx]
            rdx = (m.r32(rcx) as u64) as u32 as u64;
            // 46c3d lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 46c41 mov r9d,0x1
            r9 = (0x1u64) as u32 as u64;
            // 46c47 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 46c49 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 46c4c mov rcx,r14
            rcx = r14;
            // 46c4f call 0xebf72
            { let r = { let t = m.env.stub_va(0x73f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46c54 cmp QWORD PTR [rbp-0x28],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, 0x0u64).1;
            // 46c59 setne r12b
            r12 = (r12 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x46c5d;
        }
        0x46c5d => {
            // block 46c5d
            #[cfg(feature = "trace")] eprintln!("f_00046bf4 pc 46c5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46c5d mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 46c60 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 46c64 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 46c65 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 46c67 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 46c69 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 46c6b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 46c6c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbd44 size=122
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbd44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbd44;
    loop {
        match pc {
        0xbbd44 => {
            // block bbd44
            #[cfg(feature = "trace")] eprintln!("f_000bbd44 pc bbd44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbd44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbd45 mov rbp,rsp
            rbp = rsp;
            // bbd48 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bbd49 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bbd4d mov rax,rdi
            rax = rdi;
            // bbd50 lea rbx,[rbp-0x38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bbd54 mov rdi,rbx
            rdi = rbx;
            // bbd57 mov rsi,rax
            rsi = rax;
            // bbd5a call 0xbbe30
            { let r = { let t = m.env.code_va(0xbbe30); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbd5f lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // bbd63 mov rsi,rbx
            rsi = rbx;
            // bbd66 call 0xbbdbe
            { let r = { let t = m.env.code_va(0xbbdbe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbd6b lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bbd6f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbd74 lea rdi,[rip+0x10c87d] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bbd7b lea rsi,[rbp-0x20]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // bbd7f call 0xebe6a
            { let r = { let t = m.env.stub_va(0x637); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbd84 mov ebx,0xffffffff
            rbx = (0xffffffffu64) as u32 as u64;
            // bbd89 lea rcx,[rip+0x10c870] # 0x1c8600
            rcx = m.env.data_va(904653);
            // bbd90 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bbd93 je 0xbbd98
            if fl.e() { pc = 0xbbd98; continue; }
            pc = 0xbbd95;
        }
        0xbbd95 => {
            // block bbd95
            #[cfg(feature = "trace")] eprintln!("f_000bbd44 pc bbd95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbd95 mov ebx,DWORD PTR [rax+0x38]
            rbx = (m.r32(rax.wrapping_add(0x38)) as u64) as u32 as u64;
            pc = 0xbbd98;
        }
        0xbbd98 => {
            // block bbd98
            #[cfg(feature = "trace")] eprintln!("f_000bbd44 pc bbd98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbd98 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // bbd9c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbda1 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // bbda3 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bbda7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bbda8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbda9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbbdaa => {
            // block bbdaa
            #[cfg(feature = "trace")] eprintln!("f_000bbd44 pc bbdaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbdaa mov rbx,rax
            rbx = rax;
            // bbdad lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bbdb1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbdb6 mov rdi,rbx
            rdi = rbx;
            // bbdb9 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1820 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001820<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1820;
    loop {
        match pc {
        0x1820 => {
            // block 1820
            #[cfg(feature = "trace")] eprintln!("f_00001820 pc 1820 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1820 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1821 mov rbp,rsp
            rbp = rsp;
            // 1824 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1826 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1828 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 182a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 182b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 182f mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 1832 mov r14,rsi
            r14 = rsi;
            // 1835 mov rbx,rdi
            rbx = rdi;
            // 1838 mov BYTE PTR [rbp-0x21],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf), (0x0u64) as u8);
            // 183c mov BYTE PTR [rbp-0x22],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffde), (0x0u64) as u8);
            // 1840 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1843 lea rsi,[rbp-0x21]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdf);
            // 1847 call QWORD PTR [rax+0x488]
            { let r = { let t = m.r64(rax.wrapping_add(0x488)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 184d mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1850 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 1853 call 0xcb6a1
            { let r = { let t = m.env.code_va(0xcb6a1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1858 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 185a mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 185c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 185e je 0x1873
            if fl.e() { pc = 0x1873; continue; }
            pc = 0x1860;
        }
        0x1860 => {
            // block 1860
            #[cfg(feature = "trace")] eprintln!("f_00001820 pc 1860 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1860 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1863 lea rsi,[rbp-0x22]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffde);
            // 1867 mov rdi,rbx
            rdi = rbx;
            // 186a mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 186d call QWORD PTR [rax+0x4a0]
            { let r = { let t = m.r64(rax.wrapping_add(0x4a0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1873;
        }
        0x1873 => {
            // block 1873
            #[cfg(feature = "trace")] eprintln!("f_00001820 pc 1873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1873 and al,r15b
            { let r = (rax as u8 as u64) & (r15 as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 1876 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 1878 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 187b cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 187e jne 0x1889
            if fl.ne() { pc = 0x1889; continue; }
            pc = 0x1880;
        }
        0x1880 => {
            // block 1880
            #[cfg(feature = "trace")] eprintln!("f_00001820 pc 1880 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1880 mov al,BYTE PTR [rbp-0x22]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffde)) as u64) as u8 as u64);
            // 1883 or al,BYTE PTR [rbp-0x21]
            { let r = (rax as u8 as u64) | m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 1886 movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            pc = 0x1889;
        }
        0x1889 => {
            // block 1889
            #[cfg(feature = "trace")] eprintln!("f_00001820 pc 1889 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1889 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            // 188c mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 188f add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 1893 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1894 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1896 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1898 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 189a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 189b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7e4 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d7e4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd7e4;
    loop {
        match pc {
        0xd7e4 => {
            // block d7e4
            #[cfg(feature = "trace")] eprintln!("f_0000d7e4 pc d7e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7e4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7e5 mov rbp,rsp
            rbp = rsp;
            // d7e8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d7ea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7ec push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7ed push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d7ee mov r14,rdx
            r14 = rdx;
            // d7f1 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d7f3 mov r15,rdi
            r15 = rdi;
            // d7f6 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // d7fd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d800 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d806 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d808 je 0xd80e
            if fl.e() { pc = 0xd80e; continue; }
            pc = 0xd80a;
        }
        0xd80a => {
            // block d80a
            #[cfg(feature = "trace")] eprintln!("f_0000d7e4 pc d80a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d80c jmp 0xd854
            pc = 0xd854; continue;
        }
        0xd80e => {
            // block d80e
            #[cfg(feature = "trace")] eprintln!("f_0000d7e4 pc d80e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80e dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // d810 cmp ebx,0x6
            fl = sub::<32>((rbx as u32 as u64), 0x6u64).1;
            // d813 ja 0xd82f
            if fl.a() { pc = 0xd82f; continue; }
            pc = 0xd815;
        }
        0xd815 => {
            // block d815
            #[cfg(feature = "trace")] eprintln!("f_0000d7e4 pc d815 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d815 movsxd rax,ebx
            rax = sx::<32>((rbx as u32 as u64)) as u64;
            // d818 lea rcx,[rip+0xf2221] # 0xffa40
            rcx = m.env.data_va(82445);
            // d81f mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // d822 lea rdx,[rip+0xf2237] # 0xffa60
            rdx = m.env.data_va(82477);
            // d829 mov r8d,DWORD PTR [rdx+rax*4]
            r8 = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // d82d jmp 0xd83a
            pc = 0xd83a; continue;
        }
        0xd82f => {
            // block d82f
            #[cfg(feature = "trace")] eprintln!("f_0000d7e4 pc d82f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d82f mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // d835 mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            pc = 0xd83a;
        }
        0xd83a => {
            // block d83a
            #[cfg(feature = "trace")] eprintln!("f_0000d7e4 pc d83a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d83a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d83d lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d841 mov esi,0x100
            rsi = (0x100u64) as u32 as u64;
            // d846 mov rdi,r15
            rdi = r15;
            // d849 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d84c cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // d850 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0xd854;
        }
        0xd854 => {
            // block d854
            #[cfg(feature = "trace")] eprintln!("f_0000d7e4 pc d854 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d854 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d858 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d859 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d85b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d85d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d85e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42d7c size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042d7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x42d7c;
    loop {
        match pc {
        0x42d7c => {
            // block 42d7c
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42d7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42d7d mov rbp,rsp
            rbp = rsp;
            // 42d80 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42d82 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42d84 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 42d86 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42d87 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 42d8a mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // 42d8d mov rbx,rdi
            rbx = rdi;
            // 42d90 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42d93 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42d99 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42d9c cmp r14d,0x1ffff
            fl = sub::<32>((r14 as u32 as u64), 0x1ffffu64).1;
            // 42da3 ja 0x42deb
            if fl.a() { pc = 0x42deb; continue; }
            pc = 0x42da5;
        }
        0x42da5 => {
            // block 42da5
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42da5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42da5 cmp r12d,0x1
            fl = sub::<32>((r12 as u32 as u64), 0x1u64).1;
            // 42da9 jg 0x42deb
            if fl.g() { pc = 0x42deb; continue; }
            pc = 0x42dab;
        }
        0x42dab => {
            // block 42dab
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42dab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42dab xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 42dad jne 0x42deb
            if fl.ne() { pc = 0x42deb; continue; }
            pc = 0x42daf;
        }
        0x42daf => {
            // block 42daf
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42daf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42daf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42db2 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42db5 mov esi,0x902
            rsi = (0x902u64) as u32 as u64;
            // 42dba mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42dbf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42dc2 mov rdi,rbx
            rdi = rbx;
            // 42dc5 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42dc8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42dcb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42dcd je 0x42deb
            if fl.e() { pc = 0x42deb; continue; }
            pc = 0x42dcf;
        }
        0x42dcf => {
            // block 42dcf
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42dcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42dcf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42dd2 mov esi,0x906
            rsi = (0x906u64) as u32 as u64;
            // 42dd7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42ddc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42ddf mov rdi,rbx
            rdi = rbx;
            // 42de2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42de5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42de8 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x42deb;
        }
        0x42deb => {
            // block 42deb
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42deb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42deb mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 42dee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42def pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 42df1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42df3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42df5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42df6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3446 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3446<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd3446;
    loop {
        match pc {
        0xd3446 => {
            // block d3446
            #[cfg(feature = "trace")] eprintln!("f_000d3446 pc d3446 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3446 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3447 mov rbp,rsp
            rbp = rsp;
            // d344a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d344c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d344e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d3450 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d3452 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3453 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d3454 mov r14,rdi
            r14 = rdi;
            // d3457 mov esi,DWORD PTR [r14]
            rsi = (m.r32(r14) as u64) as u32 as u64;
            // d345a lea rdi,[rip+0x657ca] # 0x138c2b
            rdi = m.env.data_va(316408);
            // d3461 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3463 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3468 cmp DWORD PTR [r14],0x0
            fl = sub::<32>(m.r32(r14) as u64, 0x0u64).1;
            // d346c je 0xd34b3
            if fl.e() { pc = 0xd34b3; continue; }
            pc = 0xd346e;
        }
        0xd346e => {
            // block d346e
            #[cfg(feature = "trace")] eprintln!("f_000d3446 pc d346e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d346e lea rbx,[r14+0x10]
            rbx = r14.wrapping_add(0x10);
            // d3472 lea r15,[rip+0x657c1] # 0x138c3a
            r15 = m.env.data_va(316423);
            // d3479 lea r12,[rip+0x657f8] # 0x138c78
            r12 = m.env.data_va(316485);
            // d3480 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3483;
        }
        0xd3483 => {
            // block d3483
            #[cfg(feature = "trace")] eprintln!("f_000d3446 pc d3483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3483 mov edx,DWORD PTR [rbx-0xc]
            rdx = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // d3486 mov ecx,DWORD PTR [rbx-0x8]
            rcx = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fff8)) as u64) as u32 as u64;
            // d3489 mov r8d,DWORD PTR [rbx-0x4]
            r8 = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // d348d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d348f mov rdi,r15
            rdi = r15;
            // d3492 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // d3495 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d349a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d349c mov rdi,r12
            rdi = r12;
            // d349f mov rsi,rbx
            rsi = rbx;
            // d34a2 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d34a7 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // d34aa add rbx,0x2c
            { let (r, f) = add::<64>(rbx, 0x2cu64);
              rbx = r;
              fl = f; }
            // d34ae cmp r13d,DWORD PTR [r14]
            fl = sub::<32>((r13 as u32 as u64), m.r32(r14) as u64).1;
            // d34b1 jb 0xd3483
            if fl.b() { pc = 0xd3483; continue; }
            pc = 0xd34b3;
        }
        0xd34b3 => {
            // block d34b3
            #[cfg(feature = "trace")] eprintln!("f_000d3446 pc d34b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d34b3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d34b7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d34b8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d34ba pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d34bc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d34be pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d34c0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d34c1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd40e0 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d40e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd40e0;
    loop {
        match pc {
        0xd40e0 => {
            // block d40e0
            #[cfg(feature = "trace")] eprintln!("f_000d40e0 pc d40e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d40e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d40e1 mov rbp,rsp
            rbp = rsp;
            // d40e4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d40e6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d40e8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d40e9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d40ea mov r14,rdx
            r14 = rdx;
            // d40ed mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d40f0 mov rbx,rdi
            rbx = rdi;
            // d40f3 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // d40fa mov DWORD PTR [r14],0x12
            m.w32(r14, (0x12u64) as u32);
            // d4101 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d4104 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d410a mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d410c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d410e cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // d4112 ja 0xd4150
            if fl.a() { pc = 0xd4150; continue; }
            pc = 0xd4114;
        }
        0xd4114 => {
            // block d4114
            #[cfg(feature = "trace")] eprintln!("f_000d40e0 pc d4114 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4114 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d4116 je 0xd4150
            if fl.e() { pc = 0xd4150; continue; }
            pc = 0xd4118;
        }
        0xd4118 => {
            // block d4118
            #[cfg(feature = "trace")] eprintln!("f_000d40e0 pc d4118 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4118 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d411b lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d411f mov rdi,rbx
            rdi = rbx;
            // d4122 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d4125 call QWORD PTR [rax+0x370]
            { let r = { let t = m.r64(rax.wrapping_add(0x370)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d412b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d412d je 0xd413f
            if fl.e() { pc = 0xd413f; continue; }
            pc = 0xd412f;
        }
        0xd412f => {
            // block d412f
            #[cfg(feature = "trace")] eprintln!("f_000d40e0 pc d412f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d412f cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // d4133 je 0xd4143
            if fl.e() { pc = 0xd4143; continue; }
            pc = 0xd4135;
        }
        0xd4135 => {
            // block d4135
            #[cfg(feature = "trace")] eprintln!("f_000d40e0 pc d4135 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4135 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // d4138 call 0x5c6f2
            { let r = { let t = m.env.code_va(0x5c6f2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d413d jmp 0xd414b
            pc = 0xd414b; continue;
        }
        0xd413f => {
            // block d413f
            #[cfg(feature = "trace")] eprintln!("f_000d40e0 pc d413f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d413f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d4141 jmp 0xd4150
            pc = 0xd4150; continue;
        }
        0xd4143 => {
            // block d4143
            #[cfg(feature = "trace")] eprintln!("f_000d40e0 pc d4143 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4143 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // d4146 call 0x5c70e
            { let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd414b;
        }
        0xd414b => {
            // block d414b
            #[cfg(feature = "trace")] eprintln!("f_000d40e0 pc d414b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d414b mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // d414e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd4150;
        }
        0xd4150 => {
            // block d4150
            #[cfg(feature = "trace")] eprintln!("f_000d40e0 pc d4150 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4150 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d4154 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4155 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4157 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4159 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d415a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe8294 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e8294<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe8294;
    loop {
        match pc {
        0xe8294 => {
            // block e8294
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e8294 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8294 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e8295 mov rbp,rsp
            rbp = rsp;
            // e8298 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e829a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e829c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e829d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e829e mov rbx,rsi
            rbx = rsi;
            // e82a1 mov r14,rdi
            r14 = rdi;
            // e82a4 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82a7 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82aa lea r15,[r14+0x8]
            r15 = r14.wrapping_add(0x8);
            // e82ae test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // e82b1 jne 0xe82b8
            if fl.ne() { pc = 0xe82b8; continue; }
            pc = 0xe82b3;
        }
        0xe82b3 => {
            // block e82b3
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82b3 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e82b6 jmp 0xe82bc
            pc = 0xe82bc; continue;
        }
        0xe82b8 => {
            // block e82b8
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82b8 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xe82bc;
        }
        0xe82bc => {
            // block e82bc
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82bc mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // e82c1 mov rdi,r15
            rdi = r15;
            // e82c4 mov rsi,rbx
            rsi = rbx;
            // e82c7 call 0xeb918
            { let r = { let t = m.env.stub_va(0xe5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82cc mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e82cf mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e82d3 test BYTE PTR [rax+r15*1+0x20],0x5
            fl = logic::<8>(m.r8(rax.wrapping_add(r15).wrapping_add(0x20)) as u64 & 0x5u64);
            // e82d9 je 0xe82df
            if fl.e() { pc = 0xe82df; continue; }
            pc = 0xe82db;
        }
        0xe82db => {
            // block e82db
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82db xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e82dd jmp 0xe8304
            pc = 0xe8304; continue;
        }
        0xe82df => {
            // block e82df
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82df mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82e2 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // e82e7 mov rdi,r14
            rdi = r14;
            // e82ea call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82ed mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82f0 mov rdi,r14
            rdi = r14;
            // e82f3 call QWORD PTR [rax+0xa0]
            { let r = { let t = m.r64(rax.wrapping_add(0xa0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82f9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82fc mov rdi,r14
            rdi = r14;
            // e82ff call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8302 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe8304;
        }
        0xe8304 => {
            // block e8304
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e8304 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8304 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e8308 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8309 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e830b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e830d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e830e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4f48 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004f48<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4f48;
    loop {
        match pc {
        0x4f48 => {
            // block 4f48
            #[cfg(feature = "trace")] eprintln!("f_00004f48 pc 4f48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f48 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4f49 mov rbp,rsp
            rbp = rsp;
            // 4f4c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4f4e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4f50 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4f52 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4f53 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4f57 mov r14,rdx
            r14 = rdx;
            // 4f5a mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 4f5c mov r12,rdi
            r12 = rdi;
            // 4f5f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4f63 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f69 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4f6b jne 0x4fb2
            if fl.ne() { pc = 0x4fb2; continue; }
            pc = 0x4f6d;
        }
        0x4f6d => {
            // block 4f6d
            #[cfg(feature = "trace")] eprintln!("f_00004f48 pc 4f6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f6d mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 4f74 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 4f77 ja 0x4fb2
            if fl.a() { pc = 0x4fb2; continue; }
            pc = 0x4f79;
        }
        0x4f79 => {
            // block 4f79
            #[cfg(feature = "trace")] eprintln!("f_00004f48 pc 4f79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f79 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 4f7b lea rcx,[rip+0xfa36e] # 0xff2f0
            rcx = m.env.data_va(80573);
            // 4f82 mov esi,DWORD PTR [rcx+rax*4]
            rsi = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 4f85 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4f89 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 4f8d xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 4f90 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4f95 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4f98 mov rdi,r12
            rdi = r12;
            // 4f9b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f9e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4fa0 je 0x4fb5
            if fl.e() { pc = 0x4fb5; continue; }
            pc = 0x4fa2;
        }
        0x4fa2 => {
            // block 4fa2
            #[cfg(feature = "trace")] eprintln!("f_00004f48 pc 4fa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fa2 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 4fa5 shr eax,0x17
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x17u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 4fa8 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 4faa mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // 4fad mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 4fb0 jmp 0x4fb5
            pc = 0x4fb5; continue;
        }
        0x4fb2 => {
            // block 4fb2
            #[cfg(feature = "trace")] eprintln!("f_00004f48 pc 4fb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fb2 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x4fb5;
        }
        0x4fb5 => {
            // block 4fb5
            #[cfg(feature = "trace")] eprintln!("f_00004f48 pc 4fb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fb5 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 4fb8 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4fbc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4fbd pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4fbf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4fc1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4fc3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4fc4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe608 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e608<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe608;
    loop {
        match pc {
        0xe608 => {
            // block e608
            #[cfg(feature = "trace")] eprintln!("f_0000e608 pc e608 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e608 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e609 mov rbp,rsp
            rbp = rsp;
            // e60c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e60e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e610 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e611 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e612 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // e615 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e618 mov rbx,rdi
            rbx = rdi;
            // e61b mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // e61e call 0xcb584
            { let r = { let t = m.env.code_va(0xcb584); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e623 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e625 je 0xe679
            if fl.e() { pc = 0xe679; continue; }
            pc = 0xe627;
        }
        0xe627 => {
            // block e627
            #[cfg(feature = "trace")] eprintln!("f_0000e608 pc e627 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e627 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e62a movzx r15d,r15w
            r15 = ((r15 as u16 as u64)) as u32 as u64;
            // e62e mov rdi,rbx
            rdi = rbx;
            // e631 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e634 call QWORD PTR [rax+0x1d38]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e63a mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e63c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e63e cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // e642 ja 0xe67b
            if fl.a() { pc = 0xe67b; continue; }
            pc = 0xe644;
        }
        0xe644 => {
            // block e644
            #[cfg(feature = "trace")] eprintln!("f_0000e608 pc e644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e644 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e647 je 0xe67b
            if fl.e() { pc = 0xe67b; continue; }
            pc = 0xe649;
        }
        0xe649 => {
            // block e649
            #[cfg(feature = "trace")] eprintln!("f_0000e608 pc e649 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e649 lea rax,[rip+0xf1430] # 0xffa80
            rax = m.env.data_va(82509);
            // e650 mov esi,DWORD PTR [rax+r15*4]
            rsi = (m.r32(rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e654 lea rax,[rip+0xf1445] # 0xffaa0
            rax = m.env.data_va(82541);
            // e65b mov ecx,DWORD PTR [rax+r15*4]
            rcx = (m.r32(rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e65f lea rax,[rip+0xf145a] # 0xffac0
            rax = m.env.data_va(82573);
            // e666 mov r8d,DWORD PTR [rax+r15*4]
            r8 = (m.r32(rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e66a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e66d movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // e671 mov rdi,rbx
            rdi = rbx;
            // e674 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e677 jmp 0xe67b
            pc = 0xe67b; continue;
        }
        0xe679 => {
            // block e679
            #[cfg(feature = "trace")] eprintln!("f_0000e608 pc e679 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e679 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe67b;
        }
        0xe67b => {
            // block e67b
            #[cfg(feature = "trace")] eprintln!("f_0000e608 pc e67b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e67b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e67f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e680 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e682 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e684 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e685 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3a4a size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3a4a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3a4a;
    loop {
        match pc {
        0xc3a4a => {
            // block c3a4a
            #[cfg(feature = "trace")] eprintln!("f_000c3a4a pc c3a4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3a4a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3a4b mov rbp,rsp
            rbp = rsp;
            // c3a4e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3a50 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3a52 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3a54 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3a55 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3a59 mov r12,rsi
            r12 = rsi;
            // c3a5c mov r14,rdi
            r14 = rdi;
            // c3a5f lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3a63 mov rdx,r12
            rdx = r12;
            // c3a66 call 0xebd1a
            { let r = { let t = m.env.stub_va(0x4e7); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3a6b mov r15,rax
            r15 = rax;
            // c3a6e mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3a71 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3a74 jne 0xc3ab4
            if fl.ne() { pc = 0xc3ab4; continue; }
            pc = 0xc3a76;
        }
        0xc3a76 => {
            // block c3a76
            #[cfg(feature = "trace")] eprintln!("f_000c3a4a pc c3a76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3a76 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c3a7b call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3a80 mov rbx,rax
            rbx = rax;
            // c3a83 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // c3a87 mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // c3a8a mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // c3a92 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // c3a9a mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // c3aa2 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3aa6 mov rdi,r14
            rdi = r14;
            // c3aa9 mov rdx,r15
            rdx = r15;
            // c3aac mov rcx,rbx
            rcx = rbx;
            // c3aaf call 0xebe28
            { let r = { let t = m.env.stub_va(0x5f5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3ab4;
        }
        0xc3ab4 => {
            // block c3ab4
            #[cfg(feature = "trace")] eprintln!("f_000c3a4a pc c3ab4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3ab4 add rbx,0x28
            { let (r, f) = add::<64>(rbx, 0x28u64);
              rbx = r;
              fl = f; }
            // c3ab8 mov rax,rbx
            rax = rbx;
            // c3abb add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3abf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3ac0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3ac2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3ac4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3ac6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3ac7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3bd0 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3bd0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3bd0;
    loop {
        match pc {
        0xc3bd0 => {
            // block c3bd0
            #[cfg(feature = "trace")] eprintln!("f_000c3bd0 pc c3bd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3bd0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3bd1 mov rbp,rsp
            rbp = rsp;
            // c3bd4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3bd6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3bd8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3bda push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3bdb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3bdf mov r12,rsi
            r12 = rsi;
            // c3be2 mov r14,rdi
            r14 = rdi;
            // c3be5 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3be9 mov rdx,r12
            rdx = r12;
            // c3bec call 0xebd32
            { let r = { let t = m.env.stub_va(0x4ff); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3bf1 mov r15,rax
            r15 = rax;
            // c3bf4 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3bf7 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3bfa jne 0xc3c3a
            if fl.ne() { pc = 0xc3c3a; continue; }
            pc = 0xc3bfc;
        }
        0xc3bfc => {
            // block c3bfc
            #[cfg(feature = "trace")] eprintln!("f_000c3bd0 pc c3bfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3bfc mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c3c01 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3c06 mov rbx,rax
            rbx = rax;
            // c3c09 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // c3c0d mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // c3c10 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // c3c18 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // c3c20 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // c3c28 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3c2c mov rdi,r14
            rdi = r14;
            // c3c2f mov rdx,r15
            rdx = r15;
            // c3c32 mov rcx,rbx
            rcx = rbx;
            // c3c35 call 0xebe4c
            { let r = { let t = m.env.stub_va(0x619); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3c3a;
        }
        0xc3c3a => {
            // block c3c3a
            #[cfg(feature = "trace")] eprintln!("f_000c3bd0 pc c3c3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3c3a add rbx,0x28
            { let (r, f) = add::<64>(rbx, 0x28u64);
              rbx = r;
              fl = f; }
            // c3c3e mov rax,rbx
            rax = rbx;
            // c3c41 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3c45 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3c46 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3c48 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3c4a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3c4c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3c4d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd648e size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d648e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd648e;
    loop {
        match pc {
        0xd648e => {
            // block d648e
            #[cfg(feature = "trace")] eprintln!("f_000d648e pc d648e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d648e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d648f mov rbp,rsp
            rbp = rsp;
            // d6492 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d6494 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d6496 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6498 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6499 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d649d mov r12,rsi
            r12 = rsi;
            // d64a0 mov r14,rdi
            r14 = rdi;
            // d64a3 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d64a7 mov rdx,r12
            rdx = r12;
            // d64aa call 0xebeb8
            { let r = { let t = m.env.stub_va(0x685); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d64af mov r15,rax
            r15 = rax;
            // d64b2 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // d64b5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d64b8 jne 0xd64f8
            if fl.ne() { pc = 0xd64f8; continue; }
            pc = 0xd64ba;
        }
        0xd64ba => {
            // block d64ba
            #[cfg(feature = "trace")] eprintln!("f_000d648e pc d64ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d64ba mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // d64bf call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d64c4 mov rbx,rax
            rbx = rax;
            // d64c7 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // d64cb mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // d64ce mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // d64d6 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // d64de mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // d64e6 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // d64ea mov rdi,r14
            rdi = r14;
            // d64ed mov rdx,r15
            rdx = r15;
            // d64f0 mov rcx,rbx
            rcx = rbx;
            // d64f3 call 0xebec4
            { let r = { let t = m.env.stub_va(0x691); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd64f8;
        }
        0xd64f8 => {
            // block d64f8
            #[cfg(feature = "trace")] eprintln!("f_000d648e pc d64f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d64f8 add rbx,0x28
            { let (r, f) = add::<64>(rbx, 0x28u64);
              rbx = r;
              fl = f; }
            // d64fc mov rax,rbx
            rax = rbx;
            // d64ff add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d6503 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6504 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6506 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6508 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d650a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d650b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd65bc size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d65bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd65bc;
    loop {
        match pc {
        0xd65bc => {
            // block d65bc
            #[cfg(feature = "trace")] eprintln!("f_000d65bc pc d65bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d65bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d65bd mov rbp,rsp
            rbp = rsp;
            // d65c0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d65c2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d65c4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d65c5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d65c6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d65c9 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // d65cd lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // d65d1 mov rcx,QWORD PTR [rip+0x69de0] # 0x1403b8
            rcx = m.r64(m.env.data_va(347013)) as u64;
            // d65d8 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // d65dc mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // d65e0 lea r14,[rdi+rax*1+0x78]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x78);
            // d65e5 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // d65e9 mov QWORD PTR [rdi+rax*1+0x78],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x78), (rcx) as u64);
            // d65ee lea rbx,[rdi+rax*1+0x10]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x10);
            // d65f3 mov rcx,QWORD PTR [rip+0x69dae] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // d65fa add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // d65fe mov QWORD PTR [rdi+rax*1+0x10],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x10), (rcx) as u64);
            // d6603 lea rdi,[rdi+rax*1+0x50]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x50);
            // d6608 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d660d mov rdi,rbx
            rdi = rbx;
            // d6610 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6615 mov rsi,QWORD PTR [rip+0x69be4] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // d661c add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // d6620 mov rdi,r15
            rdi = r15;
            // d6623 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6628 mov rdi,r14
            rdi = r14;
            // d662b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d662f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6630 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6632 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d6634 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6635 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c90 size=128
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005c90<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c90;
    loop {
        match pc {
        0x5c90 => {
            // block 5c90
            #[cfg(feature = "trace")] eprintln!("f_00005c90 pc 5c90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c90 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c91 mov rbp,rsp
            rbp = rsp;
            // 5c94 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5c96 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5c98 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5c99 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5c9a mov r14,rdx
            r14 = rdx;
            // 5c9d mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5ca0 mov rbx,rdi
            rbx = rdi;
            // 5ca3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5ca6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5cac mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5cae call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5cb3 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 5cb6 cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 5cb9 jbe 0x5cff
            if fl.be() { pc = 0x5cff; continue; }
            pc = 0x5cbb;
        }
        0x5cbb => {
            // block 5cbb
            #[cfg(feature = "trace")] eprintln!("f_00005c90 pc 5cbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5cbb mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 5cc2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5cc5 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 5cc9 lea rdx,[rip+0xf96b0] # 0xff380
            rdx = m.env.data_va(80717);
            // 5cd0 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5cd3 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 5cd7 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 5cda mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 5cdf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5ce2 mov rdi,rbx
            rdi = rbx;
            // 5ce5 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5ce8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5cea je 0x5d02
            if fl.e() { pc = 0x5d02; continue; }
            pc = 0x5cec;
        }
        0x5cec => {
            // block 5cec
            #[cfg(feature = "trace")] eprintln!("f_00005c90 pc 5cec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5cec mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 5cef shr eax,0x1b
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1bu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 5cf2 not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // 5cf4 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5cf7 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // 5cfa mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 5cfd jmp 0x5d02
            pc = 0x5d02; continue;
        }
        0x5cff => {
            // block 5cff
            #[cfg(feature = "trace")] eprintln!("f_00005c90 pc 5cff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5cff xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x5d02;
        }
        0x5d02 => {
            // block 5d02
            #[cfg(feature = "trace")] eprintln!("f_00005c90 pc 5d02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d02 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 5d05 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5d09 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5d0a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5d0c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5d0e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5d0f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11654 size=128
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011654<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x11654;
    loop {
        match pc {
        0x11654 => {
            // block 11654
            #[cfg(feature = "trace")] eprintln!("f_00011654 pc 11654 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11654 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11655 mov rbp,rsp
            rbp = rsp;
            // 11658 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1165a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1165c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1165d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1165e mov r15,rdi
            r15 = rdi;
            // 11661 call 0x2de4e
            { let r = { let t = m.env.code_va(0x2de4e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11666 mov rbx,rax
            rbx = rax;
            // 11669 mov r14d,0xffffffff
            r14 = (0xffffffffu64) as u32 as u64;
            // 1166f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 11672 je 0x116c5
            if fl.e() { pc = 0x116c5; continue; }
            pc = 0x11674;
        }
        0x11674 => {
            // block 11674
            #[cfg(feature = "trace")] eprintln!("f_00011654 pc 11674 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11674 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 11677 lea rsi,[r15+0x38]
            rsi = r15.wrapping_add(0x38);
            // 1167b mov rdi,rbx
            rdi = rbx;
            // 1167e call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11681 mov rdi,QWORD PTR [r15+0x38]
            rdi = m.r64(r15.wrapping_add(0x38)) as u64;
            // 11685 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 11688 je 0x116c5
            if fl.e() { pc = 0x116c5; continue; }
            pc = 0x1168a;
        }
        0x1168a => {
            // block 1168a
            #[cfg(feature = "trace")] eprintln!("f_00011654 pc 1168a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1168a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1168d lea rcx,[r15+0x40]
            rcx = r15.wrapping_add(0x40);
            // 11691 movabs rsi,0x3a4b2f3f6e8a5ccc
            rsi = 0x3a4b_2f3f_6e8a_5cccu64;
            // 1169b movabs rdx,0x640530af78fdea87
            rdx = 0x6405_30af_78fd_ea87u64;
            // 116a5 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 116a7 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 116aa cmp QWORD PTR [r15+0x40],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x40)) as u64, 0x0u64).1;
            // 116af je 0x116bc
            if fl.e() { pc = 0x116bc; continue; }
            pc = 0x116b1;
        }
        0x116b1 => {
            // block 116b1
            #[cfg(feature = "trace")] eprintln!("f_00011654 pc 116b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 116b1 mov rdi,r15
            rdi = r15;
            // 116b4 call 0xe09b6
            { let r = { let t = m.env.code_va(0xe09b6); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 116b9 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x116bc;
        }
        0x116bc => {
            // block 116bc
            #[cfg(feature = "trace")] eprintln!("f_00011654 pc 116bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 116bc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 116bf mov rdi,rbx
            rdi = rbx;
            // 116c2 call QWORD PTR [rax+0x10]
            { let r = { let t = m.r64(rax.wrapping_add(0x10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x116c5;
        }
        0x116c5 => {
            // block 116c5
            #[cfg(feature = "trace")] eprintln!("f_00011654 pc 116c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 116c5 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 116c8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 116cc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 116cd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 116cf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 116d1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 116d2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14e6c size=128
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014e6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x14e6c;
    loop {
        match pc {
        0x14e6c => {
            // block 14e6c
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14e6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14e6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14e6d mov rbp,rsp
            rbp = rsp;
            // 14e70 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 14e72 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14e73 mov rbx,rdi
            rbx = rdi;
            // 14e76 mov rax,QWORD PTR [rip+0x12b51b] # 0x140398
            rax = m.r64(m.env.data_va(346981)) as u64;
            // 14e7d add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 14e81 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 14e84 call 0xebf12
            { let r = { let t = m.env.stub_va(0x6df); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x14e89;
        }
        0x14e89 => {
            // block 14e89
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14e89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14e89 cmp BYTE PTR [rbx+0x190],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x190)) as u64, 0x0u64).1;
            // 14e90 je 0x14ea0
            if fl.e() { pc = 0x14ea0; continue; }
            pc = 0x14e92;
        }
        0x14e92 => {
            // block 14e92
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14e92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14e92 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 14e96 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 14e99 je 0x14ea0
            if fl.e() { pc = 0x14ea0; continue; }
            pc = 0x14e9b;
        }
        0x14e9b => {
            // block 14e9b
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14e9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14e9b call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x14ea0;
        }
        0x14ea0 => {
            // block 14ea0
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14ea0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14ea0 cmp BYTE PTR [rbx+0x191],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x191)) as u64, 0x0u64).1;
            // 14ea7 je 0x14eb7
            if fl.e() { pc = 0x14eb7; continue; }
            pc = 0x14ea9;
        }
        0x14ea9 => {
            // block 14ea9
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14ea9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14ea9 mov rdi,QWORD PTR [rbx+0x68]
            rdi = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 14ead test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 14eb0 je 0x14eb7
            if fl.e() { pc = 0x14eb7; continue; }
            pc = 0x14eb2;
        }
        0x14eb2 => {
            // block 14eb2
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14eb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14eb2 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x14eb7;
        }
        0x14eb7 => {
            // block 14eb7
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14eb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14eb7 mov rdi,rbx
            rdi = rbx;
            // 14eba pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14ebb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 14ebd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14ebe jmp 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x14ec3 => {
            // block 14ec3
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14ec3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14ec3 mov rdi,rax
            rdi = rax;
            // 14ec6 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14ecb call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14ed0 jmp 0x14e89
            pc = 0x14e89; continue;
        }
        0x14ed2 => {
            // block 14ed2
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14ed2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14ed2 mov r14,rax
            r14 = rax;
            // 14ed5 mov rdi,rbx
            rdi = rbx;
            // 14ed8 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14edd mov rdi,r14
            rdi = r14;
            // 14ee0 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64c70 size=128
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064c70<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x64c70;
    loop {
        match pc {
        0x64c70 => {
            // block 64c70
            #[cfg(feature = "trace")] eprintln!("f_00064c70 pc 64c70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64c70 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64c71 mov rbp,rsp
            rbp = rsp;
            // 64c74 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 64c76 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 64c78 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 64c79 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 64c7a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 64c7d mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 64c81 lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // 64c85 mov rcx,QWORD PTR [rip+0xdb734] # 0x1403c0
            rcx = m.r64(m.env.data_va(347021)) as u64;
            // 64c8c lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // 64c90 mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // 64c94 lea r14,[rdi+rax*1+0x70]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x70);
            // 64c99 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 64c9d mov QWORD PTR [rdi+rax*1+0x70],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x70), (rcx) as u64);
            // 64ca2 lea rbx,[rdi+rax*1+0x8]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x8);
            // 64ca7 mov rcx,QWORD PTR [rip+0xdb6fa] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // 64cae add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // 64cb2 mov QWORD PTR [rdi+rax*1+0x8],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x8), (rcx) as u64);
            // 64cb7 lea rdi,[rdi+rax*1+0x48]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x48);
            // 64cbc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64cc1 mov rdi,rbx
            rdi = rbx;
            // 64cc4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64cc9 mov rsi,QWORD PTR [rip+0xdb538] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 64cd0 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 64cd4 mov rdi,r15
            rdi = r15;
            // 64cd7 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64cdc mov rdi,r14
            rdi = r14;
            // 64cdf add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 64ce3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64ce4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 64ce6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64ce8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64ce9 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64996 size=129
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064996<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x64996;
    loop {
        match pc {
        0x64996 => {
            // block 64996
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 64996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64996 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64997 mov rbp,rsp
            rbp = rsp;
            // 6499a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6499c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6499e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 649a0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 649a2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 649a3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 649a4 mov r14,rsi
            r14 = rsi;
            // 649a7 mov r12,rdi
            r12 = rdi;
            // 649aa mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // 649ad add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 649b1 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 649b4 je 0x64a05
            if fl.e() { pc = 0x64a05; continue; }
            pc = 0x649b6;
        }
        0x649b6 => {
            // block 649b6
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649b6 lea r15,[rip+0xc5ae0] # 0x12a49d
            r15 = m.env.data_va(257130);
            // 649bd jmp 0x649d2
            pc = 0x649d2; continue;
        }
        0x649bf => {
            // block 649bf
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649bf mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 649c4 mov rdi,r12
            rdi = r12;
            // 649c7 mov rsi,r15
            rsi = r15;
            // 649ca call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 649cf mov r13,rbx
            r13 = rbx;
            pc = 0x649d2;
        }
        0x649d2 => {
            // block 649d2
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649d2 lea rsi,[r13+0x20]
            rsi = r13.wrapping_add(0x20);
            // 649d6 mov rdi,r12
            rdi = r12;
            // 649d9 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 649de mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 649e2 test rax,rax
            fl = logic::<64>(rax & rax);
            // 649e5 je 0x649f4
            if fl.e() { pc = 0x649f4; continue; }
            pc = 0x649e7;
        }
        0x649e7 => {
            // block 649e7
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649e7 mov rbx,rax
            rbx = rax;
            // 649ea mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 649ed test rax,rax
            fl = logic::<64>(rax & rax);
            // 649f0 jne 0x649e7
            if fl.ne() { pc = 0x649e7; continue; }
            pc = 0x649f2;
        }
        0x649f2 => {
            // block 649f2
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649f2 jmp 0x64a00
            pc = 0x64a00; continue;
        }
        0x649f4 => {
            // block 649f4
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649f4 mov rbx,QWORD PTR [r13+0x10]
            rbx = m.r64(r13.wrapping_add(0x10)) as u64;
            // 649f8 cmp QWORD PTR [rbx],r13
            fl = sub::<64>(m.r64(rbx) as u64, r13).1;
            // 649fb mov r13,rbx
            r13 = rbx;
            // 649fe jne 0x649f4
            if fl.ne() { pc = 0x649f4; continue; }
            pc = 0x64a00;
        }
        0x64a00 => {
            // block 64a00
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 64a00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a00 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 64a03 jne 0x649bf
            if fl.ne() { pc = 0x649bf; continue; }
            pc = 0x64a05;
        }
        0x64a05 => {
            // block 64a05
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 64a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a05 mov rax,r12
            rax = r12;
            // 64a08 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 64a0c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64a0d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 64a0f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 64a11 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 64a13 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64a15 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64a16 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x581da size=130
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000581da<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x581da;
    loop {
        match pc {
        0x581da => {
            // block 581da
            #[cfg(feature = "trace")] eprintln!("f_000581da pc 581da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 581da push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 581db mov rbp,rsp
            rbp = rsp;
            // 581de push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 581e0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 581e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 581e3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 581e4 mov rax,QWORD PTR [rip+0xe81c5] # 0x1403b0
            rax = m.r64(m.env.data_va(347005)) as u64;
            // 581eb lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 581ef mov QWORD PTR [rdi-0x10],rcx
            m.w64(rdi.wrapping_add(0xffff_ffff_ffff_fff0), (rcx) as u64);
            // 581f3 lea rcx,[rax+0x68]
            rcx = rax.wrapping_add(0x68);
            // 581f7 mov QWORD PTR [rdi+0x70],rcx
            m.w64(rdi.wrapping_add(0x70), (rcx) as u64);
            // 581fb add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 581ff mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 58202 mov rax,QWORD PTR [rip+0xe819f] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 58209 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 5820d mov QWORD PTR [rdi+0x8],rax
            m.w64(rdi.wrapping_add(0x8), (rax) as u64);
            // 58211 lea r15,[rdi-0x10]
            r15 = rdi.wrapping_add(0xffff_ffff_ffff_fff0);
            // 58215 lea r14,[rdi+0x70]
            r14 = rdi.wrapping_add(0x70);
            // 58219 lea rbx,[rdi+0x8]
            rbx = rdi.wrapping_add(0x8);
            // 5821d lea rdi,[rdi+0x48]
            rdi = rdi.wrapping_add(0x48);
            // 58221 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58226 mov rdi,rbx
            rdi = rbx;
            // 58229 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5822e mov rsi,QWORD PTR [rip+0xe7fc3] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // 58235 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 58239 mov rdi,r15
            rdi = r15;
            // 5823c call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58241 mov rdi,r14
            rdi = r14;
            // 58244 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58249 mov rdi,r15
            rdi = r15;
            // 5824c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 58250 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58251 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58253 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58255 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58256 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb54d4 size=130
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b54d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb54d4;
    loop {
        match pc {
        0xb54d4 => {
            // block b54d4
            #[cfg(feature = "trace")] eprintln!("f_000b54d4 pc b54d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b54d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b54d5 mov rbp,rsp
            rbp = rsp;
            // b54d8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b54da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b54dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b54dd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b54de mov rbx,rsi
            rbx = rsi;
            // b54e1 mov r14,rdi
            r14 = rdi;
            // b54e4 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // b54e9 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b54ee mov r15,rax
            r15 = rax;
            // b54f1 lea rdx,[r15+0x20]
            rdx = r15.wrapping_add(0x20);
            // b54f5 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b54f9 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // b54fd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b5500 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5504 mov QWORD PTR [r15+0x28],rcx
            m.w64(r15.wrapping_add(0x28), (rcx) as u64);
            // b5508 mov QWORD PTR [r15+0x20],rax
            m.w64(r15.wrapping_add(0x20), (rax) as u64);
            // b550c mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // b5514 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // b551c mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // b5523 mov eax,DWORD PTR [rbx+0x18]
            rax = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // b5526 mov DWORD PTR [r15+0x38],eax
            m.w32(r15.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // b552a lea rsi,[rbp-0x20]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b552e mov rdi,r14
            rdi = r14;
            // b5531 call 0xebc2a
            { let r = { let t = m.env.stub_va(0x3f7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5536 mov rsi,QWORD PTR [rbp-0x20]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // b553a mov rdi,r14
            rdi = r14;
            // b553d mov rdx,rax
            rdx = rax;
            // b5540 mov rcx,r15
            rcx = r15;
            // b5543 call 0xebc30
            { let r = { let t = m.env.stub_va(0x3fd); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5548 mov rax,r15
            rax = r15;
            // b554b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b554f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5550 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5552 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5554 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5555 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0a7c size=130
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0a7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0a7c;
    loop {
        match pc {
        0xe0a7c => {
            // block e0a7c
            #[cfg(feature = "trace")] eprintln!("f_000e0a7c pc e0a7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0a7d mov rbp,rsp
            rbp = rsp;
            // e0a80 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0a81 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0a82 mov rbx,rdi
            rbx = rdi;
            // e0a85 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0a88 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0a8a call QWORD PTR [rax+0x1a48]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a48)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a90 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0a93 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0a95 mov rdi,rbx
            rdi = rbx;
            // e0a98 call QWORD PTR [rax+0x1a50]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a50)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a9e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0aa1 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // e0aa6 mov rdi,rbx
            rdi = rbx;
            // e0aa9 call QWORD PTR [rax+0x1a58]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a58)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0aaf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ab2 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // e0ab7 mov rdi,rbx
            rdi = rbx;
            // e0aba call QWORD PTR [rax+0x1a60]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a60)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ac0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ac3 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e0ac8 mov rdi,rbx
            rdi = rbx;
            // e0acb call QWORD PTR [rax+0x1a28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ad1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ad4 mov esi,0x8000
            rsi = (0x8000u64) as u32 as u64;
            // e0ad9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0adb mov rdi,rbx
            rdi = rbx;
            // e0ade call QWORD PTR [rax+0x1a98]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a98)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ae4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ae7 mov rax,QWORD PTR [rax+0x1aa8]
            rax = m.r64(rax.wrapping_add(0x1aa8)) as u64;
            // e0aee mov esi,0x10000
            rsi = (0x10000u64) as u32 as u64;
            // e0af3 mov rdi,rbx
            rdi = rbx;
            // e0af6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e0afa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0afb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0afc jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
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

/// parity: P2v @0xea42 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ea42<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xea42;
    loop {
        match pc {
        0xea42 => {
            // block ea42
            #[cfg(feature = "trace")] eprintln!("f_0000ea42 pc ea42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea42 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea43 mov rbp,rsp
            rbp = rsp;
            // ea46 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea48 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea4a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ea4c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea4d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ea51 mov r14,rdx
            r14 = rdx;
            // ea54 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // ea56 mov r12,rdi
            r12 = rdi;
            // ea59 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // ea5e call 0xd2f1a
            { let r = { let t = m.env.code_va(0xd2f1a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea63 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // ea65 je 0xeab2
            if fl.e() { pc = 0xeab2; continue; }
            pc = 0xea67;
        }
        0xea67 => {
            // block ea67
            #[cfg(feature = "trace")] eprintln!("f_0000ea42 pc ea67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea67 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // ea6e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ea72 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // ea74 lea rdx,[rip+0xf0cd9] # 0xff754
            rdx = m.env.data_va(81697);
            // ea7b mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // ea7e lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // ea82 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // ea85 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ea8a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ea8d mov rdi,r12
            rdi = r12;
            // ea90 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea93 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // ea95 je 0xeab5
            if fl.e() { pc = 0xeab5; continue; }
            pc = 0xea97;
        }
        0xea97 => {
            // block ea97
            #[cfg(feature = "trace")] eprintln!("f_0000ea42 pc ea97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea97 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // ea9a test ah,0x20
            fl = logic::<8>(((rax >> 8) as u8 as u64) & 0x20u64);
            // ea9d jne 0xeaaa
            if fl.ne() { pc = 0xeaaa; continue; }
            pc = 0xea9f;
        }
        0xea9f => {
            // block ea9f
            #[cfg(feature = "trace")] eprintln!("f_0000ea42 pc ea9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea9f shr eax,0xc
            { let (r, f) = shr::<32>((rax as u32 as u64), 0xcu64, fl); rax = (r) as u32 as u64; fl = f; }
            // eaa2 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eaa5 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // eaa7 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xeaaa;
        }
        0xeaaa => {
            // block eaaa
            #[cfg(feature = "trace")] eprintln!("f_0000ea42 pc eaaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaa mov DWORD PTR [r14],r15d
            m.w32(r14, ((r15 as u32 as u64)) as u32);
            // eaad mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // eab0 jmp 0xeab5
            pc = 0xeab5; continue;
        }
        0xeab2 => {
            // block eab2
            #[cfg(feature = "trace")] eprintln!("f_0000ea42 pc eab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab2 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xeab5;
        }
        0xeab5 => {
            // block eab5
            #[cfg(feature = "trace")] eprintln!("f_0000ea42 pc eab5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab5 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // eab8 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // eabc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eabd pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eabf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eac1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eac3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eac4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x580dc size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000580dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x580dc;
    loop {
        match pc {
        0x580dc => {
            // block 580dc
            #[cfg(feature = "trace")] eprintln!("f_000580dc pc 580dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 580dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 580dd mov rbp,rsp
            rbp = rsp;
            // 580e0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 580e2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 580e4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 580e5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 580e6 mov rbx,rdi
            rbx = rdi;
            // 580e9 mov rax,QWORD PTR [rip+0xe82c0] # 0x1403b0
            rax = m.r64(m.env.data_va(347005)) as u64;
            // 580f0 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 580f4 mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // 580f7 mov r14,rbx
            r14 = rbx;
            // 580fa sub r14,0xffffffffffffff80
            { let (r, f) = sub::<64>(r14, 0xffff_ffff_ffff_ff80u64);
              r14 = r;
              fl = f; }
            // 580fe lea rcx,[rax+0x68]
            rcx = rax.wrapping_add(0x68);
            // 58102 mov QWORD PTR [rbx+0x80],rcx
            m.w64(rbx.wrapping_add(0x80), (rcx) as u64);
            // 58109 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 5810d mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 58111 lea r15,[rbx+0x18]
            r15 = rbx.wrapping_add(0x18);
            // 58115 mov rax,QWORD PTR [rip+0xe828c] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 5811c add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 58120 mov QWORD PTR [rbx+0x18],rax
            m.w64(rbx.wrapping_add(0x18), (rax) as u64);
            // 58124 lea rdi,[rbx+0x58]
            rdi = rbx.wrapping_add(0x58);
            // 58128 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5812d mov rdi,r15
            rdi = r15;
            // 58130 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58135 mov rsi,QWORD PTR [rip+0xe80bc] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // 5813c add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 58140 mov rdi,rbx
            rdi = rbx;
            // 58143 call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58148 mov rdi,r14
            rdi = r14;
            // 5814b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5814f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58150 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58152 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58154 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58155 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
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

/// parity: P2v @0xdb438 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000db438<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdb438;
    loop {
        match pc {
        0xdb438 => {
            // block db438
            #[cfg(feature = "trace")] eprintln!("f_000db438 pc db438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db438 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // db439 mov rbp,rsp
            rbp = rsp;
            // db43c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // db43e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // db440 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // db442 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // db443 mov r14,rdx
            r14 = rdx;
            // db446 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // db449 mov rbx,rdi
            rbx = rdi;
            // db44c lea r15,[rbx+0x6a0]
            r15 = rbx.wrapping_add(0x6a0);
            // db453 mov rax,QWORD PTR [rbx+0x6a0]
            rax = m.r64(rbx.wrapping_add(0x6a0)) as u64;
            // db45a cmp QWORD PTR [rbx+0x6a8],rax
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x6a8)) as u64, rax).1;
            // db461 je 0xdb46a
            if fl.e() { pc = 0xdb46a; continue; }
            pc = 0xdb463;
        }
        0xdb463 => {
            // block db463
            #[cfg(feature = "trace")] eprintln!("f_000db438 pc db463 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db463 mov QWORD PTR [rbx+0x6a8],rax
            m.w64(rbx.wrapping_add(0x6a8), (rax) as u64);
            pc = 0xdb46a;
        }
        0xdb46a => {
            // block db46a
            #[cfg(feature = "trace")] eprintln!("f_000db438 pc db46a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db46a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // db46c mov rdi,r15
            rdi = r15;
            // db46f call 0xebed0
            { let r = { let t = m.env.stub_va(0x69d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db474 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // db477 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // db47a cmp r12d,0x200
            fl = sub::<32>((r12 as u32 as u64), 0x200u64).1;
            // db481 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // db484 and cl,al
            { let r = (rcx as u8 as u64) & (rax as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // db486 lea rdi,[rbx+0x380]
            rdi = rbx.wrapping_add(0x380);
            // db48d movzx r8d,cl
            r8 = ((rcx as u8 as u64)) as u32 as u64;
            // db491 movzx edx,r12w
            rdx = ((r12 as u16 as u64)) as u32 as u64;
            // db495 mov rsi,r15
            rsi = r15;
            // db498 mov rcx,r14
            rcx = r14;
            // db49b call 0xe9740
            { let r = { let t = m.env.code_va(0xe9740); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db4a0 mov eax,DWORD PTR [rbx+0x6a8]
            rax = (m.r32(rbx.wrapping_add(0x6a8)) as u64) as u32 as u64;
            // db4a6 sub eax,DWORD PTR [rbx+0x6a0]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbx.wrapping_add(0x6a0)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // db4ac mov DWORD PTR [rbx+0x338],eax
            m.w32(rbx.wrapping_add(0x338), ((rax as u32 as u64)) as u32);
            // db4b2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // db4b3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // db4b5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // db4b7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // db4b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // db4ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd34c2 size=133
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d34c2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd34c2;
    loop {
        match pc {
        0xd34c2 => {
            // block d34c2
            #[cfg(feature = "trace")] eprintln!("f_000d34c2 pc d34c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d34c2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d34c3 mov rbp,rsp
            rbp = rsp;
            // d34c6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d34c8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d34c9 mov r14,rsi
            r14 = rsi;
            // d34cc mov rbx,rdi
            rbx = rdi;
            // d34cf lea rdi,[rip+0x657fa] # 0x138cd0
            rdi = m.env.data_va(316573);
            // d34d6 call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d34db mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // d34dd lea rdi,[rip+0x657a1] # 0x138c85
            rdi = m.env.data_va(316498);
            // d34e4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d34e6 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d34eb mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // d34ee lea rdi,[rip+0x657a7] # 0x138c9c
            rdi = m.env.data_va(316521);
            // d34f5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d34f7 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d34fc mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // d34ff lea rdi,[rip+0x657a4] # 0x138caa
            rdi = m.env.data_va(316535);
            // d3506 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3508 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d350d lea rdi,[rip+0x657d2] # 0x138ce6
            rdi = m.env.data_va(316595);
            // d3514 call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3519 lea rsi,[rbx+0x40]
            rsi = rbx.wrapping_add(0x40);
            // d351d lea rdi,[rip+0x65797] # 0x138cbb
            rdi = m.env.data_va(316552);
            // d3524 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3526 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d352b test r14,r14
            fl = logic::<64>(r14 & r14);
            // d352e je 0xd3542
            if fl.e() { pc = 0xd3542; continue; }
            pc = 0xd3530;
        }
        0xd3530 => {
            // block d3530
            #[cfg(feature = "trace")] eprintln!("f_000d34c2 pc d3530 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3530 cmp DWORD PTR [rbx+0x4],0x1
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x4)) as u64, 0x1u64).1;
            // d3534 jne 0xd3542
            if fl.ne() { pc = 0xd3542; continue; }
            pc = 0xd3536;
        }
        0xd3536 => {
            // block d3536
            #[cfg(feature = "trace")] eprintln!("f_000d34c2 pc d3536 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3536 mov rdi,r14
            rdi = r14;
            // d3539 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d353a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d353c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d353d jmp 0xd3446
            { let r = { let t = m.env.code_va(0xd3446); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xd3542 => {
            // block d3542
            #[cfg(feature = "trace")] eprintln!("f_000d34c2 pc d3542 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3542 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3543 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3545 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3546 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4bc0 size=134
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004bc0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4bc0;
    loop {
        match pc {
        0x4bc0 => {
            // block 4bc0
            #[cfg(feature = "trace")] eprintln!("f_00004bc0 pc 4bc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bc0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4bc1 mov rbp,rsp
            rbp = rsp;
            // 4bc4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4bc6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4bc8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4bc9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4bca mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 4bcd mov r14,rsi
            r14 = rsi;
            // 4bd0 mov rbx,rdi
            rbx = rdi;
            // 4bd3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4bd6 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 4bd9 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bdf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4be1 je 0x4be8
            if fl.e() { pc = 0x4be8; continue; }
            pc = 0x4be3;
        }
        0x4be3 => {
            // block 4be3
            #[cfg(feature = "trace")] eprintln!("f_00004bc0 pc 4be3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4be3 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 4be6 jmp 0x4c37
            pc = 0x4c37; continue;
        }
        0x4be8 => {
            // block 4be8
            #[cfg(feature = "trace")] eprintln!("f_00004bc0 pc 4be8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4be8 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 4bef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4bf2 mov rdi,rbx
            rdi = rbx;
            // 4bf5 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bfb mov r9,QWORD PTR [rbx]
            r9 = m.r64(rbx) as u64;
            // 4bfe mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 4c01 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 4c04 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4c06 cmove rcx,r15
            if fl.e() { rcx = r15; }
            // 4c0a lea rax,[rip+0xfa3df] # 0xfeff0
            rax = m.env.data_va(79805);
            // 4c11 mov esi,DWORD PTR [rax+rcx*4]
            rsi = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4c14 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 4c18 mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 4c1d mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 4c23 mov rdi,rbx
            rdi = rbx;
            // 4c26 call QWORD PTR [r9+0x38]
            { let r = { let t = m.r64(r9.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c2a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4c2c je 0x4c37
            if fl.e() { pc = 0x4c37; continue; }
            pc = 0x4c2e;
        }
        0x4c2e => {
            // block 4c2e
            #[cfg(feature = "trace")] eprintln!("f_00004bc0 pc 4c2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c2e mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 4c31 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 4c34 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x4c37;
        }
        0x4c37 => {
            // block 4c37
            #[cfg(feature = "trace")] eprintln!("f_00004bc0 pc 4c37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c37 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 4c3a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4c3e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4c3f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4c41 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4c43 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4c44 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdae4 size=134
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000dae4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdae4;
    loop {
        match pc {
        0xdae4 => {
            // block dae4
            #[cfg(feature = "trace")] eprintln!("f_0000dae4 pc dae4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dae4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dae5 mov rbp,rsp
            rbp = rsp;
            // dae8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // daea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // daeb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // daef mov r14,rdx
            r14 = rdx;
            // daf2 mov rbx,rdi
            rbx = rdi;
            // daf5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // daf8 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dafe test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db00 je 0xdb06
            if fl.e() { pc = 0xdb06; continue; }
            pc = 0xdb02;
        }
        0xdb02 => {
            // block db02
            #[cfg(feature = "trace")] eprintln!("f_0000dae4 pc db02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db02 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db04 jmp 0xdb61
            pc = 0xdb61; continue;
        }
        0xdb06 => {
            // block db06
            #[cfg(feature = "trace")] eprintln!("f_0000dae4 pc db06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db06 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // db0d mov DWORD PTR [rbp-0x18],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0x0u64) as u32);
            // db14 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db17 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // db1b mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // db20 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // db25 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // db2b mov rdi,rbx
            rdi = rbx;
            // db2e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db31 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db34 lea rdx,[rbp-0x18]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // db38 mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // db3d mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // db42 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // db48 mov rdi,rbx
            rdi = rbx;
            // db4b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db4e cmp DWORD PTR [rbp-0x14],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x1u64).1;
            // db52 sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // db55 cmp DWORD PTR [rbp-0x18],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64, 0x0u64).1;
            // db59 sete dl
            rdx = (rdx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // db5c and dl,cl
            { let r = (rdx as u8 as u64) & (rcx as u8 as u64); let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // db5e mov BYTE PTR [r14],dl
            m.w8(r14, ((rdx as u8 as u64)) as u8);
            pc = 0xdb61;
        }
        0xdb61 => {
            // block db61
            #[cfg(feature = "trace")] eprintln!("f_0000dae4 pc db61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db61 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // db65 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // db66 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // db68 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // db69 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe686 size=134
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e686<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe686;
    loop {
        match pc {
        0xe686 => {
            // block e686
            #[cfg(feature = "trace")] eprintln!("f_0000e686 pc e686 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e686 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e687 mov rbp,rsp
            rbp = rsp;
            // e68a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e68c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e68e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e68f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e690 mov r14,rdx
            r14 = rdx;
            // e693 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e696 mov rbx,rdi
            rbx = rdi;
            // e699 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // e69c call 0xcb584
            { let r = { let t = m.env.code_va(0xcb584); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e6a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e6a3 je 0xe6ff
            if fl.e() { pc = 0xe6ff; continue; }
            pc = 0xe6a5;
        }
        0xe6a5 => {
            // block e6a5
            #[cfg(feature = "trace")] eprintln!("f_0000e686 pc e6a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6a5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e6a8 movzx r15d,r15w
            r15 = ((r15 as u16 as u64)) as u32 as u64;
            // e6ac mov rdi,rbx
            rdi = rbx;
            // e6af mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e6b2 call QWORD PTR [rax+0x1d38]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e6b8 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e6ba xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6bc cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // e6c0 ja 0xe701
            if fl.a() { pc = 0xe701; continue; }
            pc = 0xe6c2;
        }
        0xe6c2 => {
            // block e6c2
            #[cfg(feature = "trace")] eprintln!("f_0000e686 pc e6c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6c2 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e6c5 je 0xe701
            if fl.e() { pc = 0xe701; continue; }
            pc = 0xe6c7;
        }
        0xe6c7 => {
            // block e6c7
            #[cfg(feature = "trace")] eprintln!("f_0000e686 pc e6c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6c7 lea rax,[rip+0xf13b2] # 0xffa80
            rax = m.env.data_va(82509);
            // e6ce mov esi,DWORD PTR [rax+r15*4]
            rsi = (m.r32(rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e6d2 lea rax,[rip+0xf13c7] # 0xffaa0
            rax = m.env.data_va(82541);
            // e6d9 mov ecx,DWORD PTR [rax+r15*4]
            rcx = (m.r32(rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e6dd lea rax,[rip+0xf13dc] # 0xffac0
            rax = m.env.data_va(82573);
            // e6e4 mov r8d,DWORD PTR [rax+r15*4]
            r8 = (m.r32(rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e6e8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e6eb lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e6ef mov rdi,rbx
            rdi = rbx;
            // e6f2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e6f5 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // e6f9 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            // e6fd jmp 0xe701
            pc = 0xe701; continue;
        }
        0xe6ff => {
            // block e6ff
            #[cfg(feature = "trace")] eprintln!("f_0000e686 pc e6ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6ff xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe701;
        }
        0xe701 => {
            // block e701
            #[cfg(feature = "trace")] eprintln!("f_0000e686 pc e701 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e701 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e705 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e706 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e708 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e70a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e70b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29f28 size=134
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029f28<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x29f28;
    loop {
        match pc {
        0x29f28 => {
            // block 29f28
            #[cfg(feature = "trace")] eprintln!("f_00029f28 pc 29f28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29f28 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29f29 mov rbp,rsp
            rbp = rsp;
            // 29f2c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29f2d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 29f2e mov rbx,rdi
            rbx = rdi;
            // 29f31 movabs rax,0x0
            rax = 0x0u64;
            // 29f3b mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 29f3e mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 29f46 movabs rax,0x800000114
            rax = 0x0_0008_0000_0114u64;
            // 29f50 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 29f54 lea rdi,[rbx+0x34]
            rdi = rbx.wrapping_add(0x34);
            // 29f58 mov DWORD PTR [rbx+0x30],0x0
            m.w32(rbx.wrapping_add(0x30), (0x0u64) as u32);
            // 29f5f mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 29f67 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 29f6f mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 29f77 call 0x2946a
            { let r = { let t = m.env.code_va(0x2946a); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29f7c mov DWORD PTR [rbx+0x10c],0x0
            m.w32(rbx.wrapping_add(0x10c), (0x0u64) as u32);
            // 29f86 mov QWORD PTR [rbx+0x104],0x0
            m.w64(rbx.wrapping_add(0x104), (0x0u64) as u64);
            // 29f91 mov QWORD PTR [rbx+0xfc],0x0
            m.w64(rbx.wrapping_add(0xfc), (0x0u64) as u64);
            // 29f9c mov DWORD PTR [rbx+0x110],0x0
            m.w32(rbx.wrapping_add(0x110), (0x0u64) as u32);
            // 29fa6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 29faa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29fab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29fac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29fae size=134
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029fae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x29fae;
    loop {
        match pc {
        0x29fae => {
            // block 29fae
            #[cfg(feature = "trace")] eprintln!("f_00029fae pc 29fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29fae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29faf mov rbp,rsp
            rbp = rsp;
            // 29fb2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29fb3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 29fb4 mov rbx,rdi
            rbx = rdi;
            // 29fb7 movabs rax,0x0
            rax = 0x0u64;
            // 29fc1 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 29fc4 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 29fcc movabs rax,0x800000114
            rax = 0x0_0008_0000_0114u64;
            // 29fd6 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 29fda lea rdi,[rbx+0x34]
            rdi = rbx.wrapping_add(0x34);
            // 29fde mov DWORD PTR [rbx+0x30],0x0
            m.w32(rbx.wrapping_add(0x30), (0x0u64) as u32);
            // 29fe5 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 29fed mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 29ff5 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 29ffd call 0x2946a
            { let r = { let t = m.env.code_va(0x2946a); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2a002 mov DWORD PTR [rbx+0x10c],0x0
            m.w32(rbx.wrapping_add(0x10c), (0x0u64) as u32);
            // 2a00c mov QWORD PTR [rbx+0x104],0x0
            m.w64(rbx.wrapping_add(0x104), (0x0u64) as u64);
            // 2a017 mov QWORD PTR [rbx+0xfc],0x0
            m.w64(rbx.wrapping_add(0xfc), (0x0u64) as u64);
            // 2a022 mov DWORD PTR [rbx+0x110],0x0
            m.w32(rbx.wrapping_add(0x110), (0x0u64) as u32);
            // 2a02c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2a030 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2a031 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2a032 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5d4ac size=134
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005d4ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5d4ac;
    loop {
        match pc {
        0x5d4ac => {
            // block 5d4ac
            #[cfg(feature = "trace")] eprintln!("f_0005d4ac pc 5d4ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d4ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5d4ad mov rbp,rsp
            rbp = rsp;
            // 5d4b0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5d4b1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5d4b2 mov rbx,rdi
            rbx = rdi;
            // 5d4b5 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 5d4b8 jne 0x5d4da
            if fl.ne() { pc = 0x5d4da; continue; }
            pc = 0x5d4ba;
        }
        0x5d4ba => {
            // block 5d4ba
            #[cfg(feature = "trace")] eprintln!("f_0005d4ac pc 5d4ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d4ba lea rax,[rip+0xce903] # 0x12bdc4
            rax = m.env.data_va(263569);
            // 5d4c1 lea rsi,[rip+0xce90d] # 0x12bdd5
            rsi = m.env.data_va(263586);
            // 5d4c8 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d4ca cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d4ce mov eax,0x10
            rax = (0x10u64) as u32 as u64;
            // 5d4d3 mov edx,0x25
            rdx = (0x25u64) as u32 as u64;
            // 5d4d8 jmp 0x5d51c
            pc = 0x5d51c; continue;
        }
        0x5d4da => {
            // block 5d4da
            #[cfg(feature = "trace")] eprintln!("f_0005d4ac pc 5d4da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d4da test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 5d4dc jne 0x5d4fe
            if fl.ne() { pc = 0x5d4fe; continue; }
            pc = 0x5d4de;
        }
        0x5d4de => {
            // block 5d4de
            #[cfg(feature = "trace")] eprintln!("f_0005d4ac pc 5d4de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d4de lea rax,[rip+0xce8a5] # 0x12bd8a
            rax = m.env.data_va(263511);
            // 5d4e5 lea rsi,[rip+0xce8b4] # 0x12bda0
            rsi = m.env.data_va(263533);
            // 5d4ec test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d4ee cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d4f2 mov eax,0x15
            rax = (0x15u64) as u32 as u64;
            // 5d4f7 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 5d4fc jmp 0x5d51c
            pc = 0x5d51c; continue;
        }
        0x5d4fe => {
            // block 5d4fe
            #[cfg(feature = "trace")] eprintln!("f_0005d4ac pc 5d4fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d4fe lea rax,[rip+0xce6e3] # 0x12bbe8
            rax = m.env.data_va(263093);
            // 5d505 lea rsi,[rip+0xce8ef] # 0x12bdfb
            rsi = m.env.data_va(263624);
            // 5d50c test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d50e cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d512 mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5d517 mov edx,0x21
            rdx = (0x21u64) as u32 as u64;
            pc = 0x5d51c;
        }
        0x5d51c => {
            // block 5d51c
            #[cfg(feature = "trace")] eprintln!("f_0005d4ac pc 5d51c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d51c cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 5d520 mov rdi,rbx
            rdi = rbx;
            // 5d523 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d528 mov rax,rbx
            rax = rbx;
            // 5d52b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5d52f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5d530 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5d531 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3547 size=135
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3547<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd3547;
    loop {
        match pc {
        0xd3547 => {
            // block d3547
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3547 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3548 mov rbp,rsp
            rbp = rsp;
            // d354b mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // d354d cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d3550 jne 0xd3562
            if fl.ne() { pc = 0xd3562; continue; }
            pc = 0xd3552;
        }
        0xd3552 => {
            // block d3552
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3552 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3552 cmp DWORD PTR [rdi+0x4],esi
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, (rsi as u32 as u64)).1;
            // d3555 jne 0xd35ca
            if fl.ne() { pc = 0xd35ca; continue; }
            pc = 0xd3557;
        }
        0xd3557 => {
            // block d3557
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3557 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3557 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d3559 lea rcx,[rip+0xea030] # 0x1bd590
            rcx = m.env.data_va(859485);
            // d3560 jmp 0xd358b
            pc = 0xd358b; continue;
        }
        0xd3562 => {
            // block d3562
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3562 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d3565 jne 0xd357d
            if fl.ne() { pc = 0xd357d; continue; }
            pc = 0xd3567;
        }
        0xd3567 => {
            // block d3567
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3567 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3567 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // d356c cmp esi,0x14
            fl = sub::<32>((rsi as u32 as u64), 0x14u64).1;
            // d356f je 0xd35a4
            if fl.e() { pc = 0xd35a4; continue; }
            pc = 0xd3571;
        }
        0xd3571 => {
            // block d3571
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3571 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3571 cmp esi,0x15
            fl = sub::<32>((rsi as u32 as u64), 0x15u64).1;
            // d3574 jne 0xd35a2
            if fl.ne() { pc = 0xd35a2; continue; }
            pc = 0xd3576;
        }
        0xd3576 => {
            // block d3576
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3576 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3576 mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // d357b jmp 0xd35a4
            pc = 0xd35a4; continue;
        }
        0xd357d => {
            // block d357d
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d357d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d357d cmp DWORD PTR [rdi+0x4],esi
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, (rsi as u32 as u64)).1;
            // d3580 jne 0xd35ca
            if fl.ne() { pc = 0xd35ca; continue; }
            pc = 0xd3582;
        }
        0xd3582 => {
            // block d3582
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3582 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3582 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d3584 lea rcx,[rip+0xea0c5] # 0x1bd650
            rcx = m.env.data_va(859677);
            pc = 0xd358b;
        }
        0xd358b => {
            // block d358b
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d358b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d358b mov rsi,QWORD PTR [rcx+rax*8]
            rsi = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // d358f mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // d3592 add rdi,0x40
            { let (r, f) = add::<64>(rdi, 0x40u64);
              rdi = r;
              fl = f; }
            // d3596 call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d359b test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d359d sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d35a0 jmp 0xd35cc
            pc = 0xd35cc; continue;
        }
        0xd35a2 => {
            // block d35a2
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35a2 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            pc = 0xd35a4;
        }
        0xd35a4 => {
            // block d35a4
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35a4 cmp eax,DWORD PTR [rdi+0x4]
            fl = sub::<32>((rax as u32 as u64), m.r32(rdi.wrapping_add(0x4)) as u64).1;
            // d35a7 jne 0xd35ca
            if fl.ne() { pc = 0xd35ca; continue; }
            pc = 0xd35a9;
        }
        0xd35a9 => {
            // block d35a9
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35a9 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d35ab lea rcx,[rip+0xe9f1e] # 0x1bd4d0
            rcx = m.env.data_va(859293);
            // d35b2 mov rsi,QWORD PTR [rcx+rax*8]
            rsi = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // d35b6 mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // d35b9 add rdi,0x40
            { let (r, f) = add::<64>(rdi, 0x40u64);
              rdi = r;
              fl = f; }
            // d35bd call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d35c2 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d35c4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d35c6 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d35c8 je 0xd35cc
            if fl.e() { pc = 0xd35cc; continue; }
            pc = 0xd35ca;
        }
        0xd35ca => {
            // block d35ca
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35ca xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd35cc;
        }
        0xd35cc => {
            // block d35cc
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35cc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d35cd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d388 size=136
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d388<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2d388;
    loop {
        match pc {
        0x2d388 => {
            // block 2d388
            #[cfg(feature = "trace")] eprintln!("f_0002d388 pc 2d388 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d388 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d389 mov rbp,rsp
            rbp = rsp;
            // 2d38c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d38e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d390 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d391 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2d392 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2d395 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2d399 lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // 2d39d mov rcx,QWORD PTR [rip+0x11301c] # 0x1403c0
            rcx = m.r64(m.env.data_va(347021)) as u64;
            // 2d3a4 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // 2d3a8 mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // 2d3ac lea r14,[rdi+rax*1+0x70]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x70);
            // 2d3b1 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 2d3b5 mov QWORD PTR [rdi+rax*1+0x70],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x70), (rcx) as u64);
            // 2d3ba lea rbx,[rdi+rax*1+0x8]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x8);
            // 2d3bf mov rcx,QWORD PTR [rip+0x112fe2] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // 2d3c6 add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // 2d3ca mov QWORD PTR [rdi+rax*1+0x8],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x8), (rcx) as u64);
            // 2d3cf lea rdi,[rdi+rax*1+0x48]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x48);
            // 2d3d4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d3d9 mov rdi,rbx
            rdi = rbx;
            // 2d3dc call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d3e1 mov rsi,QWORD PTR [rip+0x112e20] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 2d3e8 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 2d3ec mov rdi,r15
            rdi = r15;
            // 2d3ef call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d3f4 mov rdi,r14
            rdi = r14;
            // 2d3f7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d3fc mov rdi,r15
            rdi = r15;
            // 2d3ff add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2d403 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d404 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d406 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d408 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d409 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba510 size=136
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba510<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba510;
    loop {
        match pc {
        0xba510 => {
            // block ba510
            #[cfg(feature = "trace")] eprintln!("f_000ba510 pc ba510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba510 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba511 mov rbp,rsp
            rbp = rsp;
            // ba514 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba516 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba518 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba519 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ba51a mov rbx,rdi
            rbx = rdi;
            // ba51d mov rax,QWORD PTR [rip+0x85e8c] # 0x1403b0
            rax = m.r64(m.env.data_va(347005)) as u64;
            // ba524 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // ba528 mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // ba52b mov r14,rbx
            r14 = rbx;
            // ba52e sub r14,0xffffffffffffff80
            { let (r, f) = sub::<64>(r14, 0xffff_ffff_ffff_ff80u64);
              r14 = r;
              fl = f; }
            // ba532 lea rcx,[rax+0x68]
            rcx = rax.wrapping_add(0x68);
            // ba536 mov QWORD PTR [rbx+0x80],rcx
            m.w64(rbx.wrapping_add(0x80), (rcx) as u64);
            // ba53d add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // ba541 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // ba545 lea r15,[rbx+0x18]
            r15 = rbx.wrapping_add(0x18);
            // ba549 mov rax,QWORD PTR [rip+0x85e58] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // ba550 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // ba554 mov QWORD PTR [rbx+0x18],rax
            m.w64(rbx.wrapping_add(0x18), (rax) as u64);
            // ba558 lea rdi,[rbx+0x58]
            rdi = rbx.wrapping_add(0x58);
            // ba55c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba561 mov rdi,r15
            rdi = r15;
            // ba564 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba569 mov rsi,QWORD PTR [rip+0x85c88] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // ba570 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ba574 mov rdi,rbx
            rdi = rbx;
            // ba577 call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba57c mov rdi,r14
            rdi = r14;
            // ba57f call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba584 mov rdi,rbx
            rdi = rbx;
            // ba587 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ba58b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba58c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba58e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba590 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba591 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb40 size=138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000eb40<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeb40;
    loop {
        match pc {
        0xeb40 => {
            // block eb40
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb40 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb41 mov rbp,rsp
            rbp = rsp;
            // eb44 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb46 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb48 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb49 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // eb4a mov r14,rdx
            r14 = rdx;
            // eb4d mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // eb4f mov r15,rdi
            r15 = rdi;
            // eb52 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // eb56 call 0xd2f1a
            { let r = { let t = m.env.code_va(0xd2f1a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb5d je 0xebbc
            if fl.e() { pc = 0xebbc; continue; }
            pc = 0xeb5f;
        }
        0xeb5f => {
            // block eb5f
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb5f mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // eb66 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // eb69 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // eb6b lea rdx,[rip+0xf0be2] # 0xff754
            rdx = m.env.data_va(81697);
            // eb72 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // eb75 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // eb79 mov ecx,0x70000
            rcx = (0x70000u64) as u32 as u64;
            // eb7e mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // eb84 mov rdi,r15
            rdi = r15;
            // eb87 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb8a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb8c je 0xebbc
            if fl.e() { pc = 0xebbc; continue; }
            pc = 0xeb8e;
        }
        0xeb8e => {
            // block eb8e
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb8e mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // eb91 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // eb94 je 0xeba4
            if fl.e() { pc = 0xeba4; continue; }
            pc = 0xeb96;
        }
        0xeb96 => {
            // block eb96
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb96 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // eb99 jne 0xebad
            if fl.ne() { pc = 0xebad; continue; }
            pc = 0xeb9b;
        }
        0xeb9b => {
            // block eb9b
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb9b mov DWORD PTR [r14],0x4b00
            m.w32(r14, (0x4b00u64) as u32);
            // eba2 jmp 0xebb8
            pc = 0xebb8; continue;
        }
        0xeba4 => {
            // block eba4
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eba4 mov DWORD PTR [r14],0x2580
            m.w32(r14, (0x2580u64) as u32);
            // ebab jmp 0xebb8
            pc = 0xebb8; continue;
        }
        0xebad => {
            // block ebad
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebad test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // ebaf jne 0xebbc
            if fl.ne() { pc = 0xebbc; continue; }
            pc = 0xebb1;
        }
        0xebb1 => {
            // block ebb1
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebb1 mov DWORD PTR [r14],0x9600
            m.w32(r14, (0x9600u64) as u32);
            pc = 0xebb8;
        }
        0xebb8 => {
            // block ebb8
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebb8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ebba jmp 0xebbe
            pc = 0xebbe; continue;
        }
        0xebbc => {
            // block ebbc
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebbc xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xebbe;
        }
        0xebbe => {
            // block ebbe
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebbe add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ebc2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ebc3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ebc5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ebc7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ebc8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bab0 size=138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bab0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2bab0;
    loop {
        match pc {
        0x2bab0 => {
            // block 2bab0
            #[cfg(feature = "trace")] eprintln!("f_0002bab0 pc 2bab0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bab0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bab1 mov rbp,rsp
            rbp = rsp;
            // 2bab4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bab6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bab7 mov r14,rsi
            r14 = rsi;
            // 2baba mov rbx,rdi
            rbx = rdi;
            // 2babd mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 2bac1 sub rax,QWORD PTR [r14]
            { let (r, f) = sub::<64>(rax, m.r64(r14) as u64);
              rax = r;
              fl = f; }
            // 2bac4 mov rcx,rax
            rcx = rax;
            // 2bac7 shr rcx,0x4
            { let (r, f) = shr::<64>(rcx, 0x4u64, fl); rcx = r; fl = f; }
            // 2bacb mov DWORD PTR [rbx+0x20],ecx
            m.w32(rbx.wrapping_add(0x20), ((rcx as u32 as u64)) as u32);
            // 2bace mov DWORD PTR [rbx+0x4c],0x0
            m.w32(rbx.wrapping_add(0x4c), (0x0u64) as u32);
            // 2bad5 lea rdi,[rbx+0x24]
            rdi = rbx.wrapping_add(0x24);
            // 2bad9 shr rax,0x2
            { let (r, f) = shr::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 2badd movabs rsi,0x3fffffffc
            rsi = 0x0_0003_ffff_fffcu64;
            // 2bae7 and rsi,rax
            { let r = rsi & rax; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 2baea call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2baef lea rdi,[rbx+0x50]
            rdi = rbx.wrapping_add(0x50);
            // 2baf3 mov esi,DWORD PTR [rbx+0x20]
            rsi = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2baf6 shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 2bafa call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2baff lea rdi,[rbx+0x78]
            rdi = rbx.wrapping_add(0x78);
            // 2bb03 mov esi,DWORD PTR [rbx+0x20]
            rsi = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2bb06 shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 2bb0a call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bb0f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 2bb12 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 2bb16 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 2bb19 je 0x2bb33
            if fl.e() { pc = 0x2bb33; continue; }
            pc = 0x2bb1b;
        }
        0x2bb1b => {
            // block 2bb1b
            #[cfg(feature = "trace")] eprintln!("f_0002bab0 pc 2bb1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb1b mov rdx,QWORD PTR [rbx+0x24]
            rdx = m.r64(rbx.wrapping_add(0x24)) as u64;
            // 2bb1f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bb21;
        }
        0x2bb21 => {
            // block 2bb21
            #[cfg(feature = "trace")] eprintln!("f_0002bab0 pc 2bb21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb21 mov edi,DWORD PTR [rax]
            rdi = (m.r32(rax) as u64) as u32 as u64;
            // 2bb23 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 2bb25 inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 2bb27 mov DWORD PTR [rdx+rbx*4],edi
            m.w32(rdx.wrapping_add(rbx.wrapping_mul(4)), ((rdi as u32 as u64)) as u32);
            // 2bb2a add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 2bb2e cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 2bb31 jne 0x2bb21
            if fl.ne() { pc = 0x2bb21; continue; }
            pc = 0x2bb33;
        }
        0x2bb33 => {
            // block 2bb33
            #[cfg(feature = "trace")] eprintln!("f_0002bab0 pc 2bb33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb33 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2bb35 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bb36 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bb38 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bb39 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f1f8 size=138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f1f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2f1f8;
    loop {
        match pc {
        0x2f1f8 => {
            // block 2f1f8
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f1f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f1f9 mov rbp,rsp
            rbp = rsp;
            // 2f1fc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f1fe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f1ff sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f203 mov rbx,rdi
            rbx = rdi;
            // 2f206 lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 2f20a mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f20e mov rdi,r14
            rdi = r14;
            // 2f211 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f217 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2f219 je 0x2f279
            if fl.e() { pc = 0x2f279; continue; }
            pc = 0x2f21b;
        }
        0x2f21b => {
            // block 2f21b
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f21b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f21b mov rdi,r14
            rdi = r14;
            // 2f21e call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f223 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2f226 cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 2f22b jne 0x2f279
            if fl.ne() { pc = 0x2f279; continue; }
            pc = 0x2f22d;
        }
        0x2f22d => {
            // block 2f22d
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f22d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f22d mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f231 mov esi,DWORD PTR [rbx+0x2b4]
            rsi = (m.r32(rbx.wrapping_add(0x2b4)) as u64) as u32 as u64;
            // 2f237 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2f239 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f23e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f241 mov rdi,r14
            rdi = r14;
            // 2f244 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f247 mov rdi,rbx
            rdi = rbx;
            // 2f24a call 0x31164
            { let r = { let t = m.env.code_va(0x31164); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f24f mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // 2f253 lea rsi,[rbp-0x11]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 2f257 mov rdi,rbx
            rdi = rbx;
            // 2f25a call 0x311bc
            { let r = { let t = m.env.code_va(0x311bc); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f25f movzx esi,BYTE PTR [rbp-0x11]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u32 as u64;
            // 2f263 or esi,0x80
            { let r = (rsi as u32 as u64) | 0x80u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2f269 mov rdi,rbx
            rdi = rbx;
            // 2f26c add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f270 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f271 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f273 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f274 jmp 0x31368
            { let r = { let t = m.env.code_va(0x31368); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0x2f279 => {
            // block 2f279
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f279 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f27d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f27e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f280 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f281 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd58e size=140
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d58e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd58e;
    loop {
        match pc {
        0xd58e => {
            // block d58e
            #[cfg(feature = "trace")] eprintln!("f_0000d58e pc d58e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d58e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d58f mov rbp,rsp
            rbp = rsp;
            // d592 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d594 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d596 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d598 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d599 mov r14,rdx
            r14 = rdx;
            // d59c mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d59f mov r12,rdi
            r12 = rdi;
            // d5a2 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d5a7 call 0xd0fb9
            { let r = { let t = m.env.code_va(0xd0fb9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5ac movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d5af cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // d5b2 jbe 0xd60c
            if fl.be() { pc = 0xd60c; continue; }
            pc = 0xd5b4;
        }
        0xd5b4 => {
            // block d5b4
            #[cfg(feature = "trace")] eprintln!("f_0000d58e pc d5b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b4 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d5b8 test r15w,r15w
            fl = logic::<16>((r15 as u16 as u64) & (r15 as u16 as u64));
            // d5bc mov ecx,0x70
            rcx = (0x70u64) as u32 as u64;
            // d5c1 mov esi,0xfe
            rsi = (0xfeu64) as u32 as u64;
            // d5c6 cmove esi,ecx
            if fl.e() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // d5c9 mov edx,DWORD PTR [r14+0x4]
            rdx = (m.r32(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // d5cd xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d5cf mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5d4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5d7 mov rdi,r12
            rdi = r12;
            // d5da call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5dd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5df je 0xd60e
            if fl.e() { pc = 0xd60e; continue; }
            pc = 0xd5e1;
        }
        0xd5e1 => {
            // block d5e1
            #[cfg(feature = "trace")] eprintln!("f_0000d58e pc d5e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5e1 test r15w,r15w
            fl = logic::<16>((r15 as u16 as u64) & (r15 as u16 as u64));
            // d5e5 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d5e9 mov ecx,0x71
            rcx = (0x71u64) as u32 as u64;
            // d5ee mov esi,0xff
            rsi = (0xffu64) as u32 as u64;
            // d5f3 cmove esi,ecx
            if fl.e() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // d5f6 mov edx,DWORD PTR [r14+0x8]
            rdx = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // d5fa mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5ff xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d602 mov rdi,r12
            rdi = r12;
            // d605 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d608 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d60a jmp 0xd60e
            pc = 0xd60e; continue;
        }
        0xd60c => {
            // block d60c
            #[cfg(feature = "trace")] eprintln!("f_0000d58e pc d60c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d60c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd60e;
        }
        0xd60e => {
            // block d60e
            #[cfg(feature = "trace")] eprintln!("f_0000d58e pc d60e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d60e mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d610 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d611 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d613 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d615 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d617 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d618 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x447f0 size=140
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000447f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x447f0;
    loop {
        match pc {
        0x447f0 => {
            // block 447f0
            #[cfg(feature = "trace")] eprintln!("f_000447f0 pc 447f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 447f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 447f1 mov rbp,rsp
            rbp = rsp;
            // 447f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 447f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 447f8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 447fa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 447fb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 447ff mov r14,rdx
            r14 = rdx;
            // 44802 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 44804 mov r15,rdi
            r15 = rdi;
            // 44807 mov BYTE PTR [r14],0x1
            m.w8(r14, (0x1u64) as u8);
            // 4480b cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 4480e ja 0x4486d
            if fl.a() { pc = 0x4486d; continue; }
            pc = 0x44810;
        }
        0x44810 => {
            // block 44810
            #[cfg(feature = "trace")] eprintln!("f_000447f0 pc 44810 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44810 movzx r12d,bx
            r12 = ((rbx as u16 as u64)) as u32 as u64;
            // 44814 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 44818 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4481d movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 44820 cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // 44823 jae 0x4486d
            if fl.ae() { pc = 0x4486d; continue; }
            pc = 0x44825;
        }
        0x44825 => {
            // block 44825
            #[cfg(feature = "trace")] eprintln!("f_000447f0 pc 44825 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44825 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 4482c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 4482f mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 44831 lea rdx,[rip+0xbbe68] # 0x1006a0
            rdx = m.env.data_va(85613);
            // 44838 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4483b and ebx,0x1
            { let r = (rbx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 4483e mov edx,0x8000
            rdx = (0x8000u64) as u32 as u64;
            // 44843 mov ecx,0x2000
            rcx = (0x2000u64) as u32 as u64;
            // 44848 cmovne ecx,edx
            if fl.ne() { rcx = ((rdx as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 4484b add ebx,ebx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rbx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 4484d or ebx,0xd
            { let r = (rbx as u32 as u64) | 0xdu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 44850 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 44854 mov rdi,r15
            rdi = r15;
            // 44857 mov r8d,ebx
            r8 = ((rbx as u32 as u64)) as u32 as u64;
            // 4485a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4485d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4485f je 0x4486d
            if fl.e() { pc = 0x4486d; continue; }
            pc = 0x44861;
        }
        0x44861 => {
            // block 44861
            #[cfg(feature = "trace")] eprintln!("f_000447f0 pc 44861 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44861 cmp DWORD PTR [rbp-0x24],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64, 0x0u64).1;
            // 44865 sete BYTE PTR [r14]
            m.w8(r14, ((fl.e() as u64)) as u8);
            // 44869 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4486b jmp 0x4486f
            pc = 0x4486f; continue;
        }
        0x4486d => {
            // block 4486d
            #[cfg(feature = "trace")] eprintln!("f_000447f0 pc 4486d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4486d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4486f;
        }
        0x4486f => {
            // block 4486f
            #[cfg(feature = "trace")] eprintln!("f_000447f0 pc 4486f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4486f add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44873 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44874 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 44876 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44878 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4487a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4487b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2cf8 size=140
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2cf8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2cf8;
    loop {
        match pc {
        0xe2cf8 => {
            // block e2cf8
            #[cfg(feature = "trace")] eprintln!("f_000e2cf8 pc e2cf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2cf8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2cf9 mov rbp,rsp
            rbp = rsp;
            // e2cfc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2cfe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2d00 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e2d02 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2d04 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2d05 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e2d06 mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // e2d09 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // e2d0c mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // e2d0f mov r14,rdi
            r14 = rdi;
            // e2d12 mov rdi,QWORD PTR [r14]
            rdi = m.r64(r14) as u64;
            // e2d15 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2d18 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e2d1c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2d1e mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2d23 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2d28 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2d2b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2d2e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2d30 je 0xe2d72
            if fl.e() { pc = 0xe2d72; continue; }
            pc = 0xe2d32;
        }
        0xe2d32 => {
            // block e2d32
            #[cfg(feature = "trace")] eprintln!("f_000e2cf8 pc e2d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2d32 add r13d,r13d
            { let (r, f) = add::<32>((r13 as u32 as u64), (r13 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // e2d35 movzx eax,r12b
            rax = ((r12 as u8 as u64)) as u32 as u64;
            // e2d39 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e2d3b movzx esi,r15b
            rsi = ((r15 as u8 as u64)) as u32 as u64;
            // e2d3f or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e2d41 mov cl,r13b
            rcx = (rcx & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // e2d44 shl esi,cl
            { let (r, f) = shl::<32>((rsi as u32 as u64), (rcx as u8 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e2d46 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // e2d4b mov cl,r13b
            rcx = (rcx & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // e2d4e shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e2d50 not edx
            rdx = (!(rdx as u32 as u64)) as u32 as u64;
            // e2d52 and edx,DWORD PTR [rbp-0x2c]
            { let r = (rdx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e2d55 or edx,esi
            { let r = (rdx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e2d57 mov DWORD PTR [rbp-0x2c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rdx as u32 as u64)) as u32);
            // e2d5a mov rdi,QWORD PTR [r14]
            rdi = m.r64(r14) as u64;
            // e2d5d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2d60 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2d65 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2d6a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2d6d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2d70 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xe2d72;
        }
        0xe2d72 => {
            // block e2d72
            #[cfg(feature = "trace")] eprintln!("f_000e2cf8 pc e2d72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2d72 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2d74 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e2d78 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2d79 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2d7b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e2d7d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2d7f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2d81 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2d82 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe957c size=140
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e957c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe957c;
    loop {
        match pc {
        0xe957c => {
            // block e957c
            #[cfg(feature = "trace")] eprintln!("f_000e957c pc e957c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e957c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e957d mov rbp,rsp
            rbp = rsp;
            // e9580 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9581 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9588 mov rbx,rsi
            rbx = rsi;
            // e958b mov BYTE PTR [rbp-0xa0],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u8);
            // e9592 mov WORD PTR [rbp-0x9e],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff62), (0x0u64) as u16);
            // e959b mov DWORD PTR [rbp-0x9c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff64), (0x0u64) as u32);
            // e95a5 mov DWORD PTR [rbp-0x98],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u32);
            // e95af mov BYTE PTR [rbp-0x10],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u8);
            // e95b3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e95b6 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // e95bd call QWORD PTR [rax+0x98]
            { let r = { let t = m.r64(rax.wrapping_add(0x98)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e95c3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e95c5 je 0xe95fe
            if fl.e() { pc = 0xe95fe; continue; }
            pc = 0xe95c7;
        }
        0xe95c7 => {
            // block e95c7
            #[cfg(feature = "trace")] eprintln!("f_000e957c pc e95c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e95c7 mov ecx,DWORD PTR [rbp-0xa0]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64) as u32 as u64;
            // e95cd mov BYTE PTR [rbx],cl
            m.w8(rbx, ((rcx as u8 as u64)) as u8);
            // e95cf mov edx,DWORD PTR [rbp-0x98]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64) as u32 as u64;
            // e95d5 mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // e95d8 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e95db mov WORD PTR [rbx+0x2],cx
            m.w16(rbx.wrapping_add(0x2), ((rcx as u16 as u64)) as u16);
            // e95df mov rcx,QWORD PTR [rbp-0x90]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // e95e6 mov rdx,QWORD PTR [rbp-0x88]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // e95ed mov QWORD PTR [rbx+0x18],rdx
            m.w64(rbx.wrapping_add(0x18), (rdx) as u64);
            // e95f1 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // e95f5 mov cl,BYTE PTR [rbp-0x10]
            rcx = (rcx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u8 as u64);
            // e95f8 mov BYTE PTR [rbx+0x90],cl
            m.w8(rbx.wrapping_add(0x90), ((rcx as u8 as u64)) as u8);
            pc = 0xe95fe;
        }
        0xe95fe => {
            // block e95fe
            #[cfg(feature = "trace")] eprintln!("f_000e957c pc e95fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e95fe add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9605 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9606 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9607 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8fb8 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008fb8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x8fb8;
    loop {
        match pc {
        0x8fb8 => {
            // block 8fb8
            #[cfg(feature = "trace")] eprintln!("f_00008fb8 pc 8fb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8fb8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8fb9 mov rbp,rsp
            rbp = rsp;
            // 8fbc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8fbe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8fbf sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 8fc3 mov rbx,rsi
            rbx = rsi;
            // 8fc6 mov r14,rdi
            r14 = rdi;
            // 8fc9 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 8fcb je 0x8fdd
            if fl.e() { pc = 0x8fdd; continue; }
            pc = 0x8fcd;
        }
        0x8fcd => {
            // block 8fcd
            #[cfg(feature = "trace")] eprintln!("f_00008fb8 pc 8fcd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8fcd mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 8fd0 mov rdi,r14
            rdi = r14;
            // 8fd3 call QWORD PTR [rax+0x1108]
            { let r = { let t = m.r64(rax.wrapping_add(0x1108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8fd9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8fdb je 0x9038
            if fl.e() { pc = 0x9038; continue; }
            pc = 0x8fdd;
        }
        0x8fdd => {
            // block 8fdd
            #[cfg(feature = "trace")] eprintln!("f_00008fb8 pc 8fdd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8fdd xorps xmm0,xmm0
            x0 = 0;
            // 8fe0 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 8fe4 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 8fec mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8fef mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // 8ff3 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 8ff7 mov rdi,rbx
            rdi = rbx;
            // 8ffa call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8ffc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8ffe je 0x902b
            if fl.e() { pc = 0x902b; continue; }
            pc = 0x9000;
        }
        0x9000 => {
            // block 9000
            #[cfg(feature = "trace")] eprintln!("f_00008fb8 pc 9000 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9000 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 9003 mov rax,QWORD PTR [rax+0xca0]
            rax = m.r64(rax.wrapping_add(0xca0)) as u64;
            // 900a lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 900e mov rdi,r14
            rdi = r14;
            // 9011 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9013 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 9015 jmp 0x902d
            pc = 0x902d; continue;
        }
        0x9017 => {
            // block 9017
            #[cfg(feature = "trace")] eprintln!("f_00008fb8 pc 9017 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9017 mov rbx,rax
            rbx = rax;
            // 901a lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 901e call 0xebb7c
            { let r = { let t = m.env.stub_va(0x349); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9023 mov rdi,rbx
            rdi = rbx;
            // 9026 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x902b;
        }
        0x902b => {
            // block 902b
            #[cfg(feature = "trace")] eprintln!("f_00008fb8 pc 902b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 902b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x902d;
        }
        0x902d => {
            // block 902d
            #[cfg(feature = "trace")] eprintln!("f_00008fb8 pc 902d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 902d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 9031 call 0xebb7c
            { let r = { let t = m.env.stub_va(0x349); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9036 jmp 0x903a
            pc = 0x903a; continue;
        }
        0x9038 => {
            // block 9038
            #[cfg(feature = "trace")] eprintln!("f_00008fb8 pc 9038 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9038 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x903a;
        }
        0x903a => {
            // block 903a
            #[cfg(feature = "trace")] eprintln!("f_00008fb8 pc 903a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 903a mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 903c add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 9040 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9041 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9043 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9044 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15f84 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015f84<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15f84;
    loop {
        match pc {
        0x15f84 => {
            // block 15f84
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15f84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f84 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15f85 mov rbp,rsp
            rbp = rsp;
            // 15f88 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15f8a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15f8c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15f8e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15f90 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15f91 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15f92 mov r14,rdx
            r14 = rdx;
            // 15f95 mov rbx,rsi
            rbx = rsi;
            // 15f98 mov r15,rdi
            r15 = rdi;
            // 15f9b mov r13,r14
            r13 = r14;
            // 15f9e sub r13,rbx
            { let (r, f) = sub::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 15fa1 cmp r13,0xffffffffffffffef
            fl = sub::<64>(r13, 0xffff_ffff_ffff_ffefu64).1;
            // 15fa5 jbe 0x15fb1
            if fl.be() { pc = 0x15fb1; continue; }
            pc = 0x15fa7;
        }
        0x15fa7 => {
            // block 15fa7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fa7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fa7 mov rdi,r15
            rdi = r15;
            // 15faa call 0xebfc0
            { let r = { let t = m.env.stub_va(0x78d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15faf jmp 0x15fc7
            pc = 0x15fc7; continue;
        }
        0x15fb1 => {
            // block 15fb1
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fb1 cmp r13,0x16
            fl = sub::<64>(r13, 0x16u64).1;
            // 15fb5 ja 0x15fc7
            if fl.a() { pc = 0x15fc7; continue; }
            pc = 0x15fb7;
        }
        0x15fb7 => {
            // block 15fb7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fb7 lea rax,[r13+r13*1+0x0]
            rax = r13.wrapping_add(r13);
            // 15fbc mov BYTE PTR [r15],al
            m.w8(r15, ((rax as u8 as u64)) as u8);
            // 15fbf inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 15fc2 mov rax,r15
            rax = r15;
            // 15fc5 jmp 0x15fe6
            pc = 0x15fe6; continue;
        }
        0x15fc7 => {
            // block 15fc7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fc7 lea r12,[r13+0x10]
            r12 = r13.wrapping_add(0x10);
            // 15fcb and r12,0xfffffffffffffff0
            { let r = r12 & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              r12 = r;
              fl = f; }
            // 15fcf mov rdi,r12
            rdi = r12;
            // 15fd2 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15fd7 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 15fdb or r12,0x1
            { let r = r12 | 0x1u64; let f = logic::<64>(r);
              r12 = r;
              fl = f; }
            // 15fdf mov QWORD PTR [r15],r12
            m.w64(r15, (r12) as u64);
            // 15fe2 mov QWORD PTR [r15+0x8],r13
            m.w64(r15.wrapping_add(0x8), (r13) as u64);
            pc = 0x15fe6;
        }
        0x15fe6 => {
            // block 15fe6
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fe6 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 15fe9 je 0x16000
            if fl.e() { pc = 0x16000; continue; }
            pc = 0x15feb;
        }
        0x15feb => {
            // block 15feb
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15feb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15feb mov rcx,rax
            rcx = rax;
            pc = 0x15fee;
        }
        0x15fee => {
            // block 15fee
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fee mov dl,BYTE PTR [rbx]
            rdx = (rdx & !0xff) | ((m.r8(rbx) as u64) as u8 as u64);
            // 15ff0 mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // 15ff2 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 15ff5 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 15ff8 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 15ffb jne 0x15fee
            if fl.ne() { pc = 0x15fee; continue; }
            pc = 0x15ffd;
        }
        0x15ffd => {
            // block 15ffd
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15ffd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ffd add rax,r13
            { let (r, f) = add::<64>(rax, r13);
              rax = r;
              fl = f; }
            pc = 0x16000;
        }
        0x16000 => {
            // block 16000
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 16000 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16000 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 16003 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 16007 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 16008 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1600a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1600c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1600e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 16010 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 16011 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41d78 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041d78<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41d78;
    loop {
        match pc {
        0x41d78 => {
            // block 41d78
            #[cfg(feature = "trace")] eprintln!("f_00041d78 pc 41d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41d78 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41d79 mov rbp,rsp
            rbp = rsp;
            // 41d7c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 41d7e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 41d80 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 41d81 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 41d82 mov rbx,rdi
            rbx = rdi;
            // 41d85 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // 41d88 ja 0x41df8
            if fl.a() { pc = 0x41df8; continue; }
            pc = 0x41d8a;
        }
        0x41d8a => {
            // block 41d8a
            #[cfg(feature = "trace")] eprintln!("f_00041d78 pc 41d8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41d8a mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 41d8c lea rcx,[rip+0xbe62d] # 0x1003c0
            rcx = m.env.data_va(84877);
            // 41d93 mov r14d,DWORD PTR [rcx+rax*4]
            r14 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 41d97 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // 41d9a je 0x41dbb
            if fl.e() { pc = 0x41dbb; continue; }
            pc = 0x41d9c;
        }
        0x41d9c => {
            // block 41d9c
            #[cfg(feature = "trace")] eprintln!("f_00041d78 pc 41d9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41d9c cmp esi,0x10
            fl = sub::<32>((rsi as u32 as u64), 0x10u64).1;
            // 41d9f jne 0x41db6
            if fl.ne() { pc = 0x41db6; continue; }
            pc = 0x41da1;
        }
        0x41da1 => {
            // block 41da1
            #[cfg(feature = "trace")] eprintln!("f_00041d78 pc 41da1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41da1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 41da4 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 41da9 mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 41dae mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 41db4 jmp 0x41ded
            pc = 0x41ded; continue;
        }
        0x41db6 => {
            // block 41db6
            #[cfg(feature = "trace")] eprintln!("f_00041d78 pc 41db6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41db6 cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // 41db9 jne 0x41df8
            if fl.ne() { pc = 0x41df8; continue; }
            pc = 0x41dbb;
        }
        0x41dbb => {
            // block 41dbb
            #[cfg(feature = "trace")] eprintln!("f_00041d78 pc 41dbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41dbb cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // 41dbe sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 41dc2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 41dc5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 41dc7 mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 41dcc mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 41dd2 mov rdi,rbx
            rdi = rbx;
            // 41dd5 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 41dd8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41ddb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 41dde movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // 41de2 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 41de7 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            pc = 0x41ded;
        }
        0x41ded => {
            // block 41ded
            #[cfg(feature = "trace")] eprintln!("f_00041d78 pc 41ded rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41ded mov rdi,rbx
            rdi = rbx;
            // 41df0 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 41df3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41df6 jmp 0x41dfa
            pc = 0x41dfa; continue;
        }
        0x41df8 => {
            // block 41df8
            #[cfg(feature = "trace")] eprintln!("f_00041d78 pc 41df8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41df8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x41dfa;
        }
        0x41dfa => {
            // block 41dfa
            #[cfg(feature = "trace")] eprintln!("f_00041d78 pc 41dfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41dfa add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 41dfe pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 41dff pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 41e01 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 41e03 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41e04 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2df4 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2df4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2df4;
    loop {
        match pc {
        0xe2df4 => {
            // block e2df4
            #[cfg(feature = "trace")] eprintln!("f_000e2df4 pc e2df4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2df4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2df5 mov rbp,rsp
            rbp = rsp;
            // e2df8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2dfa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2dfc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e2dfe push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2e00 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2e01 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e2e02 mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // e2e05 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // e2e08 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // e2e0b mov r14,rdi
            r14 = rdi;
            // e2e0e mov rdi,QWORD PTR [r14]
            rdi = m.r64(r14) as u64;
            // e2e11 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2e14 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e2e18 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2e1a mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2e1f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2e24 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2e27 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2e2a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2e2c je 0xe2e71
            if fl.e() { pc = 0xe2e71; continue; }
            pc = 0xe2e2e;
        }
        0xe2e2e => {
            // block e2e2e
            #[cfg(feature = "trace")] eprintln!("f_000e2df4 pc e2e2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2e2e add r13d,r13d
            { let (r, f) = add::<32>((r13 as u32 as u64), (r13 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // e2e31 movzx eax,r12b
            rax = ((r12 as u8 as u64)) as u32 as u64;
            // e2e35 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e2e37 movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // e2e3b or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e2e3d mov cl,r13b
            rcx = (rcx & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // e2e40 shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e2e42 mov eax,0x300
            rax = (0x300u64) as u32 as u64;
            // e2e47 mov cl,r13b
            rcx = (rcx & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // e2e4a shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e2e4c not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // e2e4e and eax,DWORD PTR [rbp-0x2c]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e2e51 shl edx,0x8
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x8u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e2e54 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e2e56 mov DWORD PTR [rbp-0x2c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rdx as u32 as u64)) as u32);
            // e2e59 mov rdi,QWORD PTR [r14]
            rdi = m.r64(r14) as u64;
            // e2e5c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2e5f mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2e64 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2e69 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2e6c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2e6f mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xe2e71;
        }
        0xe2e71 => {
            // block e2e71
            #[cfg(feature = "trace")] eprintln!("f_000e2df4 pc e2e71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2e71 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2e73 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e2e77 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2e78 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2e7a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e2e7c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2e7e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2e80 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2e81 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x58f47 size=143
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00058f47<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x58f47;
    loop {
        match pc {
        0x58f47 => {
            // block 58f47
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f47 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 58f48 mov rbp,rsp
            rbp = rsp;
            // 58f4b push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58f4d push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58f4f push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 58f51 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 58f53 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58f54 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 58f58 mov r15,r8
            r15 = r8;
            // 58f5b mov r13d,edx
            r13 = ((rdx as u32 as u64)) as u32 as u64;
            // 58f5e mov rbx,rdi
            rbx = rdi;
            // 58f61 lea r14d,[r13+r13*1+0x0]
            r14 = (r13.wrapping_add(r13)) as u32 as u64;
            // 58f66 mov DWORD PTR [rbp-0x30],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((r14 as u32 as u64)) as u32);
            // 58f6a cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 58f6d je 0x58f7a
            if fl.e() { pc = 0x58f7a; continue; }
            pc = 0x58f6f;
        }
        0x58f6f => {
            // block 58f6f
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f6f cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 58f72 jne 0x58f83
            if fl.ne() { pc = 0x58f83; continue; }
            pc = 0x58f74;
        }
        0x58f74 => {
            // block 58f74
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f74 imul r14d,esi
            { let (r, f) = imul::<32>((r14 as u32 as u64), (rsi as u32 as u64)); r14 = (r) as u32 as u64; fl = f; }
            // 58f78 jmp 0x58f91
            pc = 0x58f91; continue;
        }
        0x58f7a => {
            // block 58f7a
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f7a imul r14d,esi
            { let (r, f) = imul::<32>((r14 as u32 as u64), (rsi as u32 as u64)); r14 = (r) as u32 as u64; fl = f; }
            // 58f7e add r14d,r13d
            { let (r, f) = add::<32>((r14 as u32 as u64), (r13 as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 58f81 jmp 0x58f8e
            pc = 0x58f8e; continue;
        }
        0x58f83 => {
            // block 58f83
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f83 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 58f86 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 58f89 jne 0x58f91
            if fl.ne() { pc = 0x58f91; continue; }
            pc = 0x58f8b;
        }
        0x58f8b => {
            // block 58f8b
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f8b mov r14d,r13d
            r14 = ((r13 as u32 as u64)) as u32 as u64;
            pc = 0x58f8e;
        }
        0x58f8e => {
            // block 58f8e
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f8e sub r14d,r9d
            { let (r, f) = sub::<32>((r14 as u32 as u64), (r9 as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x58f91;
        }
        0x58f91 => {
            // block 58f91
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f91 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 58f93 je 0x58fc7
            if fl.e() { pc = 0x58fc7; continue; }
            pc = 0x58f95;
        }
        0x58f95 => {
            // block 58f95
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f95 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // 58f98 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 58f9c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x58f9f;
        }
        0x58f9f => {
            // block 58f9f
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58f9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f9f mov DWORD PTR [rbp-0x2c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u32 as u64)) as u32);
            // 58fa2 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // 58fa5 add rdi,r15
            { let (r, f) = add::<64>(rdi, r15);
              rdi = r;
              fl = f; }
            // 58fa8 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 58fab add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 58fae mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 58fb2 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58fb7 mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 58fba mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 58fbd add r14d,ecx
            { let (r, f) = add::<32>((r14 as u32 as u64), (rcx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 58fc0 add r12d,r13d
            { let (r, f) = add::<32>((r12 as u32 as u64), (r13 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // 58fc3 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 58fc5 jne 0x58f9f
            if fl.ne() { pc = 0x58f9f; continue; }
            pc = 0x58fc7;
        }
        0x58fc7 => {
            // block 58fc7
            #[cfg(feature = "trace")] eprintln!("f_00058f47 pc 58fc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58fc7 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 58fcb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58fcc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 58fce pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 58fd0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58fd2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58fd4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58fd5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc3d0 size=144
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dc3d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdc3d0;
    loop {
        match pc {
        0xdc3d0 => {
            // block dc3d0
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc3d1 mov rbp,rsp
            rbp = rsp;
            // dc3d4 cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // dc3d7 jb 0xdc3ff
            if fl.b() { pc = 0xdc3ff; continue; }
            pc = 0xdc3d9;
        }
        0xdc3d9 => {
            // block dc3d9
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3d9 movzx eax,si
            rax = ((rsi as u16 as u64)) as u32 as u64;
            // dc3dc add eax,0xfffffff8
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffff8u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // dc3df cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // dc3e2 ja 0xdc41c
            if fl.a() { pc = 0xdc41c; continue; }
            pc = 0xdc3e4;
        }
        0xdc3e4 => {
            // block dc3e4
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3e4 lea rcx,[rip+0x61] # 0xdc44c
            rcx = m.env.code_va(0xdc44c);
            // dc3eb movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // dc3ef add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // dc3f2 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xdc3f4 => {
            // block dc3f4
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3f4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc3f7 call QWORD PTR [rax+0x1980]
            { let r = { let t = m.r64(rax.wrapping_add(0x1980)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc3fd jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc3ff => {
            // block dc3ff
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3ff mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc402 movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // dc405 imul rcx,rcx,0x608
            { let (r, f) = imul::<64>(rcx, 0x608u64); rcx = r; fl = f; }
            // dc40c lea rsi,[rip+0xe138d] # 0x1bd7a0
            rsi = m.env.data_va(860013);
            // dc413 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // dc416 call QWORD PTR [rax+0x19f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19f8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdc41c;
        }
        0xdc41c => {
            // block dc41c
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc41c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc41c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // dc41e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc41f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdc420 => {
            // block dc420
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc420 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc423 call QWORD PTR [rax+0x1988]
            { let r = { let t = m.r64(rax.wrapping_add(0x1988)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc429 jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc42b => {
            // block dc42b
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc42b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc42b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc42e call QWORD PTR [rax+0x1990]
            { let r = { let t = m.r64(rax.wrapping_add(0x1990)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc434 jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc436 => {
            // block dc436
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc436 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc439 call QWORD PTR [rax+0x19a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19a8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc43f jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc441 => {
            // block dc441
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc441 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc444 call QWORD PTR [rax+0x19b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19b0)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc44a jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb720 size=144
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eb720<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeb720;
    loop {
        match pc {
        0xeb720 => {
            // block eb720
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb720 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb720 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb721 mov rbp,rsp
            rbp = rsp;
            // eb724 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb726 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb728 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eb72a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb72b mov r15,rsi
            r15 = rsi;
            // eb72e mov r14,rdi
            r14 = rdi;
            // eb731 mov r12,QWORD PTR [r14]
            r12 = m.r64(r14) as u64;
            // eb734 mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // eb738 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // eb73b je 0xeb767
            if fl.e() { pc = 0xeb767; continue; }
            pc = 0xeb73d;
        }
        0xeb73d => {
            // block eb73d
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb73d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb73d mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0xeb741;
        }
        0xeb741 => {
            // block eb741
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb741 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb741 add rdi,0xffffffffffffffc8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffc8u64);
              rdi = r;
              fl = f; }
            // eb745 add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            // eb749 mov rsi,rbx
            rsi = rbx;
            // eb74c call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb751 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb755 add rdi,0xffffffffffffffc8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffc8u64);
              rdi = r;
              fl = f; }
            // eb759 mov QWORD PTR [r15+0x8],rdi
            m.w64(r15.wrapping_add(0x8), (rdi) as u64);
            // eb75d cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // eb760 jne 0xeb741
            if fl.ne() { pc = 0xeb741; continue; }
            pc = 0xeb762;
        }
        0xeb762 => {
            // block eb762
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb762 mov r12,QWORD PTR [r14]
            r12 = m.r64(r14) as u64;
            // eb765 jmp 0xeb76b
            pc = 0xeb76b; continue;
        }
        0xeb767 => {
            // block eb767
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb767 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0xeb76b;
        }
        0xeb76b => {
            // block eb76b
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb76b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb76b mov QWORD PTR [r14],rdi
            m.w64(r14, (rdi) as u64);
            // eb76e mov QWORD PTR [r15+0x8],r12
            m.w64(r15.wrapping_add(0x8), (r12) as u64);
            // eb772 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // eb776 mov rcx,QWORD PTR [r15+0x10]
            rcx = m.r64(r15.wrapping_add(0x10)) as u64;
            // eb77a mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // eb77e mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // eb782 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // eb786 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // eb78a mov QWORD PTR [r14+0x10],rcx
            m.w64(r14.wrapping_add(0x10), (rcx) as u64);
            // eb78e mov QWORD PTR [r15+0x18],rax
            m.w64(r15.wrapping_add(0x18), (rax) as u64);
            // eb792 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb796 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // eb799 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb79a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eb79c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb79e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb7a0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb7a1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14aa size=146
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000014aa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x14aa;
    loop {
        match pc {
        0x14aa => {
            // block 14aa
            #[cfg(feature = "trace")] eprintln!("f_000014aa pc 14aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14aa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14ab mov rbp,rsp
            rbp = rsp;
            // 14ae push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 14b0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 14b2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 14b4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 14b6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14b7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 14b8 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 14bb mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 14be mov rbx,rdi
            rbx = rdi;
            // 14c1 mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // 14c4 and r13d,0x7
            { let r = (r13 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 14c8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 14cb call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14d1 mov r9,QWORD PTR [rbx]
            r9 = m.r64(rbx) as u64;
            // 14d4 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 14d7 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 14da test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 14dc cmove rcx,r12
            if fl.e() { rcx = r12; }
            // 14e0 lea rax,[rip+0xfdb09] # 0xfeff0
            rax = m.env.data_va(79805);
            // 14e7 mov r15d,DWORD PTR [rax+rcx*4]
            r15 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 14eb mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // 14f0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 14f3 mov rdi,rbx
            rdi = rbx;
            // 14f6 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 14f9 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // 14fc call QWORD PTR [r9+0x30]
            { let r = { let t = m.r64(r9.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1500 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1502 je 0x1529
            if fl.e() { pc = 0x1529; continue; }
            pc = 0x1504;
        }
        0x1504 => {
            // block 1504
            #[cfg(feature = "trace")] eprintln!("f_000014aa pc 1504 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1504 shr r14d,0x3
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x3u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 1508 and r14d,0x1
            { let r = (r14 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 150c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 150f mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 1514 mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 151a mov rdi,rbx
            rdi = rbx;
            // 151d mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 1520 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 1523 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1526 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x1529;
        }
        0x1529 => {
            // block 1529
            #[cfg(feature = "trace")] eprintln!("f_000014aa pc 1529 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1529 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 152c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1530 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1531 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1533 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1535 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1537 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1539 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 153a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43e4a size=146
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043e4a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43e4a;
    loop {
        match pc {
        0x43e4a => {
            // block 43e4a
            #[cfg(feature = "trace")] eprintln!("f_00043e4a pc 43e4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e4a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43e4b mov rbp,rsp
            rbp = rsp;
            // 43e4e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43e50 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43e52 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 43e54 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43e55 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43e59 mov r14,rcx
            r14 = rcx;
            // 43e5c mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 43e5e mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 43e61 mov r12,rdi
            r12 = rdi;
            // 43e64 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43e69 call 0xcc811
            { let r = { let t = m.env.code_va(0xcc811); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43e6e mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43e70 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43e72 cmp ebx,0x3f
            fl = sub::<32>((rbx as u32 as u64), 0x3fu64).1;
            // 43e75 ja 0x43ecf
            if fl.a() { pc = 0x43ecf; continue; }
            pc = 0x43e77;
        }
        0x43e77 => {
            // block 43e77
            #[cfg(feature = "trace")] eprintln!("f_00043e4a pc 43e77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e77 cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // 43e7b ja 0x43ecf
            if fl.a() { pc = 0x43ecf; continue; }
            pc = 0x43e7d;
        }
        0x43e7d => {
            // block 43e7d
            #[cfg(feature = "trace")] eprintln!("f_00043e4a pc 43e7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e7d test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 43e7f je 0x43ecf
            if fl.e() { pc = 0x43ecf; continue; }
            pc = 0x43e81;
        }
        0x43e81 => {
            // block 43e81
            #[cfg(feature = "trace")] eprintln!("f_00043e4a pc 43e81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e81 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 43e88 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 43e8c mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 43e8e mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 43e91 lea rdx,[rdx+rdx*2]
            rdx = rdx.wrapping_add(rdx.wrapping_mul(2));
            // 43e95 shl rdx,0x5
            { let (r, f) = shl::<64>(rdx, 0x5u64, fl); rdx = r; fl = f; }
            // 43e99 lea rdi,[rip+0xbc220] # 0x1000c0
            rdi = m.env.data_va(84109);
            // 43ea0 add rdi,rdx
            { let (r, f) = add::<64>(rdi, rdx);
              rdi = r;
              fl = f; }
            // 43ea3 lea rdx,[rcx+rcx*2]
            rdx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 43ea7 mov esi,DWORD PTR [rdi+rdx*4]
            rsi = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43eaa mov ecx,DWORD PTR [rdi+rdx*4+0x4]
            rcx = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4)) as u64) as u32 as u64;
            // 43eae mov r8d,DWORD PTR [rdi+rdx*4+0x8]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // 43eb3 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 43eb7 mov rdi,r12
            rdi = r12;
            // 43eba call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43ebd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43ebf je 0x43ecd
            if fl.e() { pc = 0x43ecd; continue; }
            pc = 0x43ec1;
        }
        0x43ec1 => {
            // block 43ec1
            #[cfg(feature = "trace")] eprintln!("f_00043e4a pc 43ec1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43ec1 cmp DWORD PTR [rbp-0x24],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64, 0x0u64).1;
            // 43ec5 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            // 43ec9 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 43ecb jmp 0x43ecf
            pc = 0x43ecf; continue;
        }
        0x43ecd => {
            // block 43ecd
            #[cfg(feature = "trace")] eprintln!("f_00043e4a pc 43ecd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43ecd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x43ecf;
        }
        0x43ecf => {
            // block 43ecf
            #[cfg(feature = "trace")] eprintln!("f_00043e4a pc 43ecf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43ecf add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43ed3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43ed4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 43ed6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43ed8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43eda pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43edb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x448ea size=146
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000448ea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x448ea;
    loop {
        match pc {
        0x448ea => {
            // block 448ea
            #[cfg(feature = "trace")] eprintln!("f_000448ea pc 448ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 448ea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 448eb mov rbp,rsp
            rbp = rsp;
            // 448ee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 448f0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 448f2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 448f4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 448f5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 448f9 mov r14,rdx
            r14 = rdx;
            // 448fc mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 448fe mov r15,rdi
            r15 = rdi;
            // 44901 mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // 44905 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 44908 ja 0x4496a
            if fl.a() { pc = 0x4496a; continue; }
            pc = 0x4490a;
        }
        0x4490a => {
            // block 4490a
            #[cfg(feature = "trace")] eprintln!("f_000448ea pc 4490a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4490a mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 4490e call 0xcf820
            { let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44913 movzx r12d,ax
            r12 = ((rax as u16 as u64)) as u32 as u64;
            // 44917 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 4491a mov rdi,r15
            rdi = r15;
            // 4491d call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44923 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 44926 add eax,r12d
            { let (r, f) = add::<32>((rax as u32 as u64), (r12 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 44929 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // 4492b jle 0x4496a
            if fl.le() { pc = 0x4496a; continue; }
            pc = 0x4492d;
        }
        0x4492d => {
            // block 4492d
            #[cfg(feature = "trace")] eprintln!("f_000448ea pc 4492d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4492d mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 44934 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 44937 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 44939 lea rdx,[rip+0xbbb30] # 0x100470
            rdx = m.env.data_va(85053);
            // 44940 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 44943 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 44947 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 44949 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4494e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 44951 mov rdi,r15
            rdi = r15;
            // 44954 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44957 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44959 je 0x4496c
            if fl.e() { pc = 0x4496c; continue; }
            pc = 0x4495b;
        }
        0x4495b => {
            // block 4495b
            #[cfg(feature = "trace")] eprintln!("f_000448ea pc 4495b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4495b mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 4495e shr eax,0x13
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x13u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 44961 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 44963 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // 44966 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 44968 jmp 0x4496c
            pc = 0x4496c; continue;
        }
        0x4496a => {
            // block 4496a
            #[cfg(feature = "trace")] eprintln!("f_000448ea pc 4496a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4496a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x4496c;
        }
        0x4496c => {
            // block 4496c
            #[cfg(feature = "trace")] eprintln!("f_000448ea pc 4496c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4496c mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 4496e add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44972 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44973 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 44975 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44977 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 44979 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4497a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb514e size=146
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b514e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb514e;
    loop {
        match pc {
        0xb514e => {
            // block b514e
            #[cfg(feature = "trace")] eprintln!("f_000b514e pc b514e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b514e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b514f mov rbp,rsp
            rbp = rsp;
            // b5152 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5154 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5156 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5158 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5159 mov rbx,rsi
            rbx = rsi;
            // b515c mov r14,rdi
            r14 = rdi;
            // b515f mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // b5164 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5169 mov r12,rax
            r12 = rax;
            // b516c mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // b516e mov DWORD PTR [r12+0x20],eax
            m.w32(r12.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // b5173 mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // b5177 mov QWORD PTR [r12+0x38],rax
            m.w64(r12.wrapping_add(0x38), (rax) as u64);
            // b517c mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5180 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b5184 mov QWORD PTR [r12+0x30],rcx
            m.w64(r12.wrapping_add(0x30), (rcx) as u64);
            // b5189 mov QWORD PTR [r12+0x28],rax
            m.w64(r12.wrapping_add(0x28), (rax) as u64);
            // b518e mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // b5196 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // b519e mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // b51a6 mov rdi,r14
            rdi = r14;
            // b51a9 mov rsi,r12
            rsi = r12;
            // b51ac call 0xebc60
            { let r = { let t = m.env.stub_va(0x42d); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b51b1 mov r14,rax
            r14 = rax;
            // b51b4 mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // b51b7 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // b51bb jne 0xb51d1
            if fl.ne() { pc = 0xb51d1; continue; }
            pc = 0xb51bd;
        }
        0xb51bd => {
            // block b51bd
            #[cfg(feature = "trace")] eprintln!("f_000b514e pc b51bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b51bd mov rdi,r12
            rdi = r12;
            // b51c0 add rdi,0x28
            { let (r, f) = add::<64>(rdi, 0x28u64);
              rdi = r;
              fl = f; }
            // b51c4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b51c9 mov rdi,r12
            rdi = r12;
            // b51cc call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb51d1;
        }
        0xb51d1 => {
            // block b51d1
            #[cfg(feature = "trace")] eprintln!("f_000b514e pc b51d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b51d1 mov rax,r14
            rax = r14;
            // b51d4 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b51d7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b51d8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b51da pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b51dc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b51de pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b51df ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc7f94 size=147
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c7f94<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc7f94;
    loop {
        match pc {
        0xc7f94 => {
            // block c7f94
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c7f94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7f94 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c7f95 mov rbp,rsp
            rbp = rsp;
            // c7f98 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c7f9a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c7f9c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c7f9e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c7fa0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c7fa1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c7fa2 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // c7fa5 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // c7fa7 mov r14,rsi
            r14 = rsi;
            // c7faa mov r12,rdi
            r12 = rdi;
            // c7fad cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // c7fb0 jne 0xc7fd1
            if fl.ne() { pc = 0xc7fd1; continue; }
            pc = 0xc7fb2;
        }
        0xc7fb2 => {
            // block c7fb2
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c7fb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7fb2 movzx ecx,r15b
            rcx = ((r15 as u8 as u64)) as u32 as u64;
            // c7fb6 mov rdi,r12
            rdi = r12;
            // c7fb9 mov rsi,r14
            rsi = r14;
            // c7fbc mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c7fbe add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c7fc2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7fc3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c7fc5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c7fc7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7fc9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c7fcb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7fcc jmp 0xc68b5
            { let r = { let t = m.env.code_va(0xc68b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xc7fd1 => {
            // block c7fd1
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c7fd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7fd1 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // c7fd4 jne 0xc8018
            if fl.ne() { pc = 0xc8018; continue; }
            pc = 0xc7fd6;
        }
        0xc7fd6 => {
            // block c7fd6
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c7fd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7fd6 lea edi,[rbx+rbx*1]
            rdi = (rbx.wrapping_add(rbx)) as u32 as u64;
            // c7fd9 add rdi,rdi
            { let (r, f) = add::<64>(rdi, rdi);
              rdi = r;
              fl = f; }
            // c7fdc call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7fe1 mov r13,rax
            r13 = rax;
            // c7fe4 mov rdi,r12
            rdi = r12;
            // c7fe7 mov rsi,r13
            rsi = r13;
            // c7fea mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c7fec call 0xc5cac
            { let r = { let t = m.env.code_va(0xc5cac); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7ff1 movzx ecx,r15b
            rcx = ((r15 as u8 as u64)) as u32 as u64;
            // c7ff5 mov rdi,r13
            rdi = r13;
            // c7ff8 mov rsi,r14
            rsi = r14;
            // c7ffb mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c7ffd call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c8002 mov rdi,r13
            rdi = r13;
            // c8005 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c8009 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c800a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c800c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c800e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8010 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8012 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8013 jmp 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc8018 => {
            // block c8018
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c8018 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8018 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c801c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c801d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c801f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c8021 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8023 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8025 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8026 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1c86 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001c86<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1c86;
    loop {
        match pc {
        0x1c86 => {
            // block 1c86
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1c86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1c86 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1c87 mov rbp,rsp
            rbp = rsp;
            // 1c8a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1c8c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1c8e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1c8f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1c90 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 1c93 mov r14,rsi
            r14 = rsi;
            // 1c96 mov rbx,rdi
            rbx = rdi;
            // 1c99 mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // 1c9d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 1ca0 call 0xcb6a1
            { let r = { let t = m.env.code_va(0xcb6a1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1ca5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1ca7 je 0x1cd2
            if fl.e() { pc = 0x1cd2; continue; }
            pc = 0x1ca9;
        }
        0x1ca9 => {
            // block 1ca9
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1ca9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ca9 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 1cb0 lea eax,[r15-0x4]
            rax = (r15.wrapping_add(0xffff_ffff_ffff_fffc)) as u32 as u64;
            // 1cb4 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 1cb7 jae 0x1cd6
            if fl.ae() { pc = 0x1cd6; continue; }
            pc = 0x1cb9;
        }
        0x1cb9 => {
            // block 1cb9
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1cb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1cb9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1cbc lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 1cc0 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1cc5 mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 1cca mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 1cd0 jmp 0x1cf3
            pc = 0x1cf3; continue;
        }
        0x1cd2 => {
            // block 1cd2
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1cd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1cd2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 1cd4 jmp 0x1d0a
            pc = 0x1d0a; continue;
        }
        0x1cd6 => {
            // block 1cd6
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1cd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1cd6 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 1cda ja 0x1d15
            if fl.a() { pc = 0x1d15; continue; }
            pc = 0x1cdc;
        }
        0x1cdc => {
            // block 1cdc
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1cdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1cdc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1cdf lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 1ce3 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1ce8 mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 1ced mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            pc = 0x1cf3;
        }
        0x1cf3 => {
            // block 1cf3
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1cf3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1cf3 mov rdi,rbx
            rdi = rbx;
            // 1cf6 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1cf9;
        }
        0x1cf9 => {
            // block 1cf9
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1cf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1cf9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1cfb setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 1cfe cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 1d02 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 1d05 and cl,al
            { let r = (rcx as u8 as u64) & (rax as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 1d07 mov BYTE PTR [r14],cl
            m.w8(r14, ((rcx as u8 as u64)) as u8);
            pc = 0x1d0a;
        }
        0x1d0a => {
            // block 1d0a
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1d0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1d0a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1d0e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1d0f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1d11 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1d13 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1d14 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1d15 => {
            // block 1d15
            #[cfg(feature = "trace")] eprintln!("f_00001c86 pc 1d15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1d15 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 1d17 jmp 0x1cf9
            pc = 0x1cf9; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2522 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002522<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2522;
    loop {
        match pc {
        0x2522 => {
            // block 2522
            #[cfg(feature = "trace")] eprintln!("f_00002522 pc 2522 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2522 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2523 mov rbp,rsp
            rbp = rsp;
            // 2526 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2528 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 252a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 252c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 252d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2531 mov r14,rdx
            r14 = rdx;
            // 2534 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2537 mov rbx,rdi
            rbx = rdi;
            // 253a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 253d call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2543 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2545 je 0x254b
            if fl.e() { pc = 0x254b; continue; }
            pc = 0x2547;
        }
        0x2547 => {
            // block 2547
            #[cfg(feature = "trace")] eprintln!("f_00002522 pc 2547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2547 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2549 jmp 0x25a9
            pc = 0x25a9; continue;
        }
        0x254b => {
            // block 254b
            #[cfg(feature = "trace")] eprintln!("f_00002522 pc 254b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 254b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 254e mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 2551 lea rdx,[rip+0xfcb28] # 0xff080
            rdx = m.env.data_va(79949);
            // 2558 mov r15d,DWORD PTR [rdx+rcx*4]
            r15 = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 255c lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 2560 mov ecx,0x1e
            rcx = (0x1eu64) as u32 as u64;
            // 2565 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 256b mov rdi,rbx
            rdi = rbx;
            // 256e mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2571 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2574 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2577 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 257a lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 257e mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // 2583 mov r8d,0x6
            r8 = (0x6u64) as u32 as u64;
            // 2589 mov rdi,rbx
            rdi = rbx;
            // 258c mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 258f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2592 and al,r12b
            { let r = (rax as u8 as u64) & (r12 as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 2595 mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 2598 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 259b mov edx,DWORD PTR [rbp-0x28]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 259e shl edx,0x4
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x4u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 25a1 and edx,0x10
            { let r = (rdx as u32 as u64) & 0x10u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 25a4 or edx,ecx
            { let r = (rdx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 25a6 mov DWORD PTR [r14],edx
            m.w32(r14, ((rdx as u32 as u64)) as u32);
            pc = 0x25a9;
        }
        0x25a9 => {
            // block 25a9
            #[cfg(feature = "trace")] eprintln!("f_00002522 pc 25a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25a9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 25ad pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25ae pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 25b0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25b2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 25b4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25b5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4b6de size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004b6de<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4b6de;
    loop {
        match pc {
        0x4b6de => {
            // block 4b6de
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b6de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b6de push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4b6df mov rbp,rsp
            rbp = rsp;
            // 4b6e2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4b6e4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4b6e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4b6e7 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 4b6eb mov rbx,rdi
            rbx = rdi;
            // 4b6ee mov r15,QWORD PTR [rip+0xf4ce3] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 4b6f5 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 4b6f8 mov QWORD PTR [rbp-0x20],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (r15) as u64);
            // 4b6fc mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 4b704 mov DWORD PTR [rbp-0x34],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x1u64) as u32);
            // 4b70b movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // 4b70e mov QWORD PTR [rbp-0x28],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rax) as u64);
            // 4b712 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b715 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 4b718 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b71a call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b720 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4b722 je 0x4b758
            if fl.e() { pc = 0x4b758; continue; }
            pc = 0x4b724;
        }
        0x4b724 => {
            // block 4b724
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b724 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b727 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b729 mov rdi,rbx
            rdi = rbx;
            // 4b72c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b732 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 4b736 lea r8,[rbp-0x30]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 4b73a lea r9,[rbp-0x34]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 4b73e mov esi,0x1a
            rsi = (0x1au64) as u32 as u64;
            // 4b743 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 4b748 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4b74a call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b74f cmp QWORD PTR [rbp-0x30],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x0u64).1;
            // 4b754 setne r14b
            r14 = (r14 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x4b758;
        }
        0x4b758 => {
            // block 4b758
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b758 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b758 cmp r15,QWORD PTR [rbp-0x20]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 4b75c jne 0x4b76c
            if fl.ne() { pc = 0x4b76c; continue; }
            pc = 0x4b75e;
        }
        0x4b75e => {
            // block 4b75e
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b75e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b75e mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 4b761 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 4b765 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4b766 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4b768 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4b76a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4b76b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4b76c => {
            // block 4b76c
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b76c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b76c call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x62ef8 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00062ef8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x62ef8;
    loop {
        match pc {
        0x62ef8 => {
            // block 62ef8
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62ef8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62ef8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 62ef9 mov rbp,rsp
            rbp = rsp;
            // 62efc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 62efd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 62efe mov rbx,rdi
            rbx = rdi;
            // 62f01 cmp esi,0x2
            fl = sub::<32>((rsi as u32 as u64), 0x2u64).1;
            // 62f04 je 0x62f1b
            if fl.e() { pc = 0x62f1b; continue; }
            pc = 0x62f06;
        }
        0x62f06 => {
            // block 62f06
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62f06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f06 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 62f09 jne 0x62f3b
            if fl.ne() { pc = 0x62f3b; continue; }
            pc = 0x62f0b;
        }
        0x62f0b => {
            // block 62f0b
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62f0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f0b lea rax,[rip+0xce46d] # 0x13137f
            rax = m.env.data_va(285516);
            // 62f12 lea rsi,[rip+0xce474] # 0x13138d
            rsi = m.env.data_va(285530);
            // 62f19 jmp 0x62f4d
            pc = 0x62f4d; continue;
        }
        0x62f1b => {
            // block 62f1b
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62f1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f1b lea rax,[rip+0xce485] # 0x1313a7
            rax = m.env.data_va(285556);
            // 62f22 lea rsi,[rip+0xce488] # 0x1313b1
            rsi = m.env.data_va(285566);
            // 62f29 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 62f2b cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 62f2f mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // 62f34 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 62f39 jmp 0x62f5d
            pc = 0x62f5d; continue;
        }
        0x62f3b => {
            // block 62f3b
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62f3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f3b test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 62f3d jne 0x62f6b
            if fl.ne() { pc = 0x62f6b; continue; }
            pc = 0x62f3f;
        }
        0x62f3f => {
            // block 62f3f
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62f3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f3f lea rax,[rip+0xce411] # 0x131357
            rax = m.env.data_va(285476);
            // 62f46 lea rsi,[rip+0xce418] # 0x131365
            rsi = m.env.data_va(285490);
            pc = 0x62f4d;
        }
        0x62f4d => {
            // block 62f4d
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62f4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f4d test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 62f4f cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 62f53 mov eax,0xd
            rax = (0xdu64) as u32 as u64;
            // 62f58 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            pc = 0x62f5d;
        }
        0x62f5d => {
            // block 62f5d
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62f5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f5d cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 62f61 mov rdi,rbx
            rdi = rbx;
            // 62f64 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 62f69 jmp 0x62f82
            pc = 0x62f82; continue;
        }
        0x62f6b => {
            // block 62f6b
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62f6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f6b mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 62f73 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 62f7b mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            pc = 0x62f82;
        }
        0x62f82 => {
            // block 62f82
            #[cfg(feature = "trace")] eprintln!("f_00062ef8 pc 62f82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f82 mov rax,rbx
            rax = rbx;
            // 62f85 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 62f89 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 62f8a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 62f8b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb922e size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b922e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb922e;
    loop {
        match pc {
        0xb922e => {
            // block b922e
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b922e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b922e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b922f mov rbp,rsp
            rbp = rsp;
            // b9232 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9234 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9236 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9237 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9238 mov r14,rdx
            r14 = rdx;
            // b923b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // b923e mov rbx,rdi
            rbx = rdi;
            // b9241 mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // b9245 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9248 call 0xcbc54
            { let r = { let t = m.env.code_va(0xcbc54); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b924d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b924f je 0xb92b0
            if fl.e() { pc = 0xb92b0; continue; }
            pc = 0xb9251;
        }
        0xb9251 => {
            // block b9251
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b9251 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9251 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9254 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9259 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b925b je 0xb92b0
            if fl.e() { pc = 0xb92b0; continue; }
            pc = 0xb925d;
        }
        0xb925d => {
            // block b925d
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b925d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b925d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9260 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9265 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // b9269 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b926c cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b926e jbe 0xb92b0
            if fl.be() { pc = 0xb92b0; continue; }
            pc = 0xb9270;
        }
        0xb9270 => {
            // block b9270
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b9270 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9270 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // b9277 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b927a lea rdx,[rip+0x496cf] # 0x102950
            rdx = m.env.data_va(94493);
            // b9281 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b9284 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // b9288 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // b928b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b9290 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9293 mov rdi,rbx
            rdi = rbx;
            // b9296 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9299 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b929b je 0xb92b3
            if fl.e() { pc = 0xb92b3; continue; }
            pc = 0xb929d;
        }
        0xb929d => {
            // block b929d
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b929d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b929d mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // b92a0 shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // b92a3 not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // b92a5 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b92a8 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // b92ab mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // b92ae jmp 0xb92b3
            pc = 0xb92b3; continue;
        }
        0xb92b0 => {
            // block b92b0
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b92b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92b0 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xb92b3;
        }
        0xb92b3 => {
            // block b92b3
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b92b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92b3 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b92b6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b92ba pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b92bb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b92bd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b92bf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b92c0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb92c2 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b92c2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb92c2;
    loop {
        match pc {
        0xb92c2 => {
            // block b92c2
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b92c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92c2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b92c3 mov rbp,rsp
            rbp = rsp;
            // b92c6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b92c8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b92ca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b92cb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b92cc mov r14,rdx
            r14 = rdx;
            // b92cf mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // b92d2 mov rbx,rdi
            rbx = rdi;
            // b92d5 mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // b92d9 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b92dc call 0xcc8c5
            { let r = { let t = m.env.code_va(0xcc8c5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b92e1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b92e3 je 0xb9344
            if fl.e() { pc = 0xb9344; continue; }
            pc = 0xb92e5;
        }
        0xb92e5 => {
            // block b92e5
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b92e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92e5 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b92e8 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b92ed test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b92ef je 0xb9344
            if fl.e() { pc = 0xb9344; continue; }
            pc = 0xb92f1;
        }
        0xb92f1 => {
            // block b92f1
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b92f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92f1 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b92f4 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b92f9 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // b92fd movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b9300 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b9302 jbe 0xb9344
            if fl.be() { pc = 0xb9344; continue; }
            pc = 0xb9304;
        }
        0xb9304 => {
            // block b9304
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b9304 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9304 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // b930b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b930e lea rdx,[rip+0x4966b] # 0x102980
            rdx = m.env.data_va(94541);
            // b9315 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b9318 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // b931c xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // b931f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b9324 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9327 mov rdi,rbx
            rdi = rbx;
            // b932a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b932d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b932f je 0xb9347
            if fl.e() { pc = 0xb9347; continue; }
            pc = 0xb9331;
        }
        0xb9331 => {
            // block b9331
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b9331 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9331 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // b9334 shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // b9337 not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // b9339 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b933c mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // b933f mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // b9342 jmp 0xb9347
            pc = 0xb9347; continue;
        }
        0xb9344 => {
            // block b9344
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b9344 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9344 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9347;
        }
        0xb9347 => {
            // block b9347
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b9347 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9347 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b934a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b934e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b934f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9351 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9353 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9354 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4244 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4244<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4244;
    loop {
        match pc {
        0xc4244 => {
            // block c4244
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c4244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4244 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4245 mov rbp,rsp
            rbp = rsp;
            // c4248 mov r8,QWORD PTR [rdi]
            r8 = m.r64(rdi) as u64;
            // c424b mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c424f cmp rdx,r8
            fl = sub::<64>(rdx, r8).1;
            // c4252 je 0xc42a2
            if fl.e() { pc = 0xc42a2; continue; }
            pc = 0xc4254;
        }
        0xc4254 => {
            // block c4254
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c4254 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4254 mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            pc = 0xc4258;
        }
        0xc4258 => {
            // block c4258
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c4258 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4258 mov rax,QWORD PTR [rdx-0x8]
            rax = m.r64(rdx.wrapping_add(0xffff_ffff_ffff_fff8)) as u64;
            // c425c mov QWORD PTR [rcx-0x8],rax
            m.w64(rcx.wrapping_add(0xffff_ffff_ffff_fff8), (rax) as u64);
            // c4260 mov r9,QWORD PTR [rdx-0x18]
            r9 = m.r64(rdx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // c4264 mov rax,QWORD PTR [rdx-0x10]
            rax = m.r64(rdx.wrapping_add(0xffff_ffff_ffff_fff0)) as u64;
            // c4268 mov QWORD PTR [rcx-0x10],rax
            m.w64(rcx.wrapping_add(0xffff_ffff_ffff_fff0), (rax) as u64);
            // c426c mov QWORD PTR [rcx-0x18],r9
            m.w64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8), (r9) as u64);
            // c4270 mov QWORD PTR [rdx-0x8],0x0
            m.w64(rdx.wrapping_add(0xffff_ffff_ffff_fff8), (0x0u64) as u64);
            // c4278 mov QWORD PTR [rdx-0x10],0x0
            m.w64(rdx.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u64);
            // c4280 mov QWORD PTR [rdx-0x18],0x0
            m.w64(rdx.wrapping_add(0xffff_ffff_ffff_ffe8), (0x0u64) as u64);
            // c4288 lea rdx,[rdx-0x18]
            rdx = rdx.wrapping_add(0xffff_ffff_ffff_ffe8);
            // c428c mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c4290 add rcx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rcx, 0xffff_ffff_ffff_ffe8u64);
              rcx = r;
              fl = f; }
            // c4294 mov QWORD PTR [rsi+0x8],rcx
            m.w64(rsi.wrapping_add(0x8), (rcx) as u64);
            // c4298 cmp r8,rdx
            fl = sub::<64>(r8, rdx).1;
            // c429b jne 0xc4258
            if fl.ne() { pc = 0xc4258; continue; }
            pc = 0xc429d;
        }
        0xc429d => {
            // block c429d
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c429d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c429d mov r8,QWORD PTR [rdi]
            r8 = m.r64(rdi) as u64;
            // c42a0 jmp 0xc42a6
            pc = 0xc42a6; continue;
        }
        0xc42a2 => {
            // block c42a2
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c42a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c42a2 mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            pc = 0xc42a6;
        }
        0xc42a6 => {
            // block c42a6
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c42a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c42a6 mov QWORD PTR [rdi],rcx
            m.w64(rdi, (rcx) as u64);
            // c42a9 mov QWORD PTR [rsi+0x8],r8
            m.w64(rsi.wrapping_add(0x8), (r8) as u64);
            // c42ad mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c42b1 mov rcx,QWORD PTR [rsi+0x10]
            rcx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c42b5 mov QWORD PTR [rdi+0x8],rcx
            m.w64(rdi.wrapping_add(0x8), (rcx) as u64);
            // c42b9 mov QWORD PTR [rsi+0x10],rax
            m.w64(rsi.wrapping_add(0x10), (rax) as u64);
            // c42bd mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // c42c1 mov rcx,QWORD PTR [rsi+0x18]
            rcx = m.r64(rsi.wrapping_add(0x18)) as u64;
            // c42c5 mov QWORD PTR [rdi+0x10],rcx
            m.w64(rdi.wrapping_add(0x10), (rcx) as u64);
            // c42c9 mov QWORD PTR [rsi+0x18],rax
            m.w64(rsi.wrapping_add(0x18), (rax) as u64);
            // c42cd mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c42d1 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c42d4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c42d5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd374e size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d374e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd374e;
    loop {
        match pc {
        0xd374e => {
            // block d374e
            #[cfg(feature = "trace")] eprintln!("f_000d374e pc d374e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d374e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d374f mov rbp,rsp
            rbp = rsp;
            // d3752 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3754 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d3756 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3757 sub rsp,0xc8
            { let (r, f) = sub::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // d375e mov r15,rsi
            r15 = rsi;
            // d3761 mov rbx,rdi
            rbx = rdi;
            // d3764 test r15,r15
            fl = logic::<64>(r15 & r15);
            // d3767 je 0xd37d1
            if fl.e() { pc = 0xd37d1; continue; }
            pc = 0xd3769;
        }
        0xd3769 => {
            // block d3769
            #[cfg(feature = "trace")] eprintln!("f_000d374e pc d3769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3769 xorps xmm0,xmm0
            x0 = 0;
            // d376c movaps XMMWORD PTR [rbp-0xe0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff20), x0);
            // d3773 mov QWORD PTR [rbp-0xd0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (0x0u64) as u64);
            // d377e movabs rax,0xc800000003
            rax = 0x0_00c8_0000_0003u64;
            // d3788 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // d378f lea r14,[rbp-0xc8]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // d3796 mov edx,0xb0
            rdx = (0xb0u64) as u32 as u64;
            // d379b mov rdi,r14
            rdi = r14;
            // d379e mov rsi,r15
            rsi = r15;
            // d37a1 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d37a6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d37a9 lea rsi,[rbp-0xe0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // d37b0 mov rdi,rbx
            rdi = rbx;
            // d37b3 call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d37b9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d37bb je 0xd37d1
            if fl.e() { pc = 0xd37d1; continue; }
            pc = 0xd37bd;
        }
        0xd37bd => {
            // block d37bd
            #[cfg(feature = "trace")] eprintln!("f_000d374e pc d37bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d37bd mov edx,0xb0
            rdx = (0xb0u64) as u32 as u64;
            // d37c2 mov rdi,r15
            rdi = r15;
            // d37c5 mov rsi,r14
            rsi = r14;
            // d37c8 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d37cd mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d37cf jmp 0xd37d3
            pc = 0xd37d3; continue;
        }
        0xd37d1 => {
            // block d37d1
            #[cfg(feature = "trace")] eprintln!("f_000d374e pc d37d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d37d1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd37d3;
        }
        0xd37d3 => {
            // block d37d3
            #[cfg(feature = "trace")] eprintln!("f_000d374e pc d37d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d37d3 add rsp,0xc8
            { let (r, f) = add::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // d37da pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d37db pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d37dd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d37df pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d37e0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2c64 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2c64<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2c64;
    loop {
        match pc {
        0xe2c64 => {
            // block e2c64
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c64 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2c65 mov rbp,rsp
            rbp = rsp;
            // e2c68 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2c6a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2c6c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2c6d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e2c6e mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // e2c71 mov r15,rdi
            r15 = rdi;
            // e2c74 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e2c7b mov ebx,0x43864
            rbx = (0x43864u64) as u32 as u64;
            // e2c80 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // e2c83 ja 0xe2c9c
            if fl.a() { pc = 0xe2c9c; continue; }
            pc = 0xe2c85;
        }
        0xe2c85 => {
            // block e2c85
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c85 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // e2c88 ja 0xe2c9c
            if fl.a() { pc = 0xe2c9c; continue; }
            pc = 0xe2c8a;
        }
        0xe2c8a => {
            // block e2c8a
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c8a cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // e2c8d ja 0xe2c9c
            if fl.a() { pc = 0xe2c9c; continue; }
            pc = 0xe2c8f;
        }
        0xe2c8f => {
            // block e2c8f
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c8f lea eax,[rdx+rsi*4]
            rax = (rdx.wrapping_add(rsi.wrapping_mul(4))) as u32 as u64;
            // e2c92 lea eax,[rcx+rax*4]
            rax = (rcx.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e2c95 lea ebx,[rax+rax*2+0x43864]
            rbx = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x43864)) as u32 as u64;
            pc = 0xe2c9c;
        }
        0xe2c9c => {
            // block e2c9c
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c9c mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e2c9f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2ca2 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e2ca6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2cab xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2cae mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2cb0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2cb3 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // e2cb7 je 0xe2cec
            if fl.e() { pc = 0xe2cec; continue; }
            pc = 0xe2cb9;
        }
        0xe2cb9 => {
            // block e2cb9
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2cb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2cb9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e2cbc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2cbf mov edx,0x80000000
            rdx = (0x80000000u64) as u32 as u64;
            // e2cc4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2cc9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2ccc mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2cce call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2cd1 movzx eax,r14b
            rax = ((r14 as u8 as u64)) as u32 as u64;
            // e2cd5 lea edx,[rax+rax*1+0x1]
            rdx = (rax.wrapping_add(rax).wrapping_add(0x1)) as u32 as u64;
            // e2cd9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e2cdc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2cdf mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2ce4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2ce7 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2ce9 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2cec;
        }
        0xe2cec => {
            // block e2cec
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2cec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2cec add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e2cf0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2cf1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2cf3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2cf5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2cf6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x595ae size=149
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000595ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x595ae;
    loop {
        match pc {
        0x595ae => {
            // block 595ae
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 595ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 595ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 595af mov rbp,rsp
            rbp = rsp;
            // 595b2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 595b4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 595b6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 595b8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 595ba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 595bb sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 595bf mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // 595c2 mov DWORD PTR [rbp-0x6c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rdx as u32 as u64)) as u32);
            // 595c5 mov rbx,rdi
            rbx = rdi;
            // 595c8 movzx eax,BYTE PTR [rbp+0x10]
            rax = (m.r8(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 595cc movzx ecx,r8b
            rcx = ((r8 as u8 as u64)) as u32 as u64;
            // 595d0 movzx r8d,r9b
            r8 = ((r9 as u8 as u64)) as u32 as u64;
            // 595d4 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 595d8 mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 595db call 0x32c5c
            { let r = { let t = m.env.code_va(0x32c5c); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 595e0 cmp DWORD PTR [rbp-0x68],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, 0x0u64).1;
            // 595e4 je 0x59634
            if fl.e() { pc = 0x59634; continue; }
            pc = 0x595e6;
        }
        0x595e6 => {
            // block 595e6
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 595e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 595e6 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 595e9 and eax,0xffffff
            { let r = (rax as u32 as u64) & 0xffffffu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 595ee mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 595f0 shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 595f3 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 595f6 mov r14w,0x1
            r14 = (r14 & !0xffff) | ((0x1u64) as u16 as u64);
            // 595fb movzx r12d,cl
            r12 = ((rcx as u8 as u64)) as u32 as u64;
            // 595ff movzx r15d,r15b
            r15 = ((r15 as u8 as u64)) as u32 as u64;
            // 59603 movzx r13d,al
            r13 = ((rax as u8 as u64)) as u32 as u64;
            pc = 0x59607;
        }
        0x59607 => {
            // block 59607
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 59607 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59607 mov r8d,DWORD PTR [rbp-0x64]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 5960b mov rdi,rbx
            rdi = rbx;
            // 5960e mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 59611 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 59614 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // 59617 mov r9d,DWORD PTR [rbp-0x6c]
            r9 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // 5961b call 0x59541
            { let r = { let t = m.env.code_va(0x59541); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59620 mov eax,DWORD PTR [rbp-0x60]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 59623 lea rbx,[rbx+rax*4]
            rbx = rbx.wrapping_add(rax.wrapping_mul(4));
            // 59627 movzx eax,r14w
            rax = ((r14 as u16 as u64)) as u32 as u64;
            // 5962b lea r14d,[r14+0x1]
            r14 = (r14.wrapping_add(0x1)) as u32 as u64;
            // 5962f cmp eax,DWORD PTR [rbp-0x68]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 59632 jb 0x59607
            if fl.b() { pc = 0x59607; continue; }
            pc = 0x59634;
        }
        0x59634 => {
            // block 59634
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 59634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59634 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 59638 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 59639 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5963b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5963d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5963f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 59641 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 59642 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8d2c size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008d2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x8d2c;
    loop {
        match pc {
        0x8d2c => {
            // block 8d2c
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8d2d mov rbp,rsp
            rbp = rsp;
            // 8d30 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8d32 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8d34 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 8d36 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8d37 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8d3b mov r14,rdx
            r14 = rdx;
            // 8d3e mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 8d40 mov r12,rdi
            r12 = rdi;
            // 8d43 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 8d48 call 0xceac8
            { let r = { let t = m.env.code_va(0xceac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8d4d mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // 8d50 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 8d57 mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            // 8d5e mov DWORD PTR [r14],0xff
            m.w32(r14, (0xffu64) as u32);
            // 8d65 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 8d69 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 8d6d mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 8d6f call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8d74 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8d76 je 0x8db2
            if fl.e() { pc = 0x8db2; continue; }
            pc = 0x8d78;
        }
        0x8d78 => {
            // block 8d78
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d78 mov esi,DWORD PTR [rbp-0x24]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 8d7b test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 8d7d je 0x8db2
            if fl.e() { pc = 0x8db2; continue; }
            pc = 0x8d7f;
        }
        0x8d7f => {
            // block 8d7f
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d7f mov edx,DWORD PTR [rbp-0x28]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 8d82 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8d84 cmp esi,r15d
            fl = sub::<32>((rsi as u32 as u64), (r15 as u32 as u64)).1;
            // 8d87 ja 0x8db4
            if fl.a() { pc = 0x8db4; continue; }
            pc = 0x8d89;
        }
        0x8d89 => {
            // block 8d89
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d89 cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // 8d8c ja 0x8db4
            if fl.a() { pc = 0x8db4; continue; }
            pc = 0x8d8e;
        }
        0x8d8e => {
            // block 8d8e
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d8e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 8d92 lea rcx,[rip+0xf68f7] # 0xff690
            rcx = m.env.data_va(81501);
            // 8d99 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8d9c lea rdi,[rip+0xf68fd] # 0xff6a0
            rdi = m.env.data_va(81517);
            // 8da3 mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8da7 mov rdi,r12
            rdi = r12;
            // 8daa mov rdx,r14
            rdx = r14;
            // 8dad call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8db0 jmp 0x8db4
            pc = 0x8db4; continue;
        }
        0x8db2 => {
            // block 8db2
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8db2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8db2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x8db4;
        }
        0x8db4 => {
            // block 8db4
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8db4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8db4 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8db8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8db9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 8dbb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8dbd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8dbf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8dc0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1729c size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001729c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1729c;
    loop {
        match pc {
        0x1729c => {
            // block 1729c
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 1729c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1729c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1729d mov rbp,rsp
            rbp = rsp;
            // 172a0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 172a2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 172a3 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 172a7 mov r14,rsi
            r14 = rsi;
            // 172aa mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // 172ac lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 172b0 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 172b5 call 0x16080
            { let r = { let t = m.env.code_va(0x16080); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 172ba mov rax,QWORD PTR [rbp-0x28]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 172be mov rcx,QWORD PTR [rbp-0x20]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 172c2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 172c5 je 0x172d6
            if fl.e() { pc = 0x172d6; continue; }
            pc = 0x172c7;
        }
        0x172c7 => {
            // block 172c7
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172c7 cmp DWORD PTR [rax],ebx
            fl = sub::<32>(m.r32(rax) as u64, (rbx as u32 as u64)).1;
            // 172c9 je 0x172da
            if fl.e() { pc = 0x172da; continue; }
            pc = 0x172cb;
        }
        0x172cb => {
            // block 172cb
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172cb add rax,0xf8
            { let (r, f) = add::<64>(rax, 0xf8u64);
              rax = r;
              fl = f; }
            // 172d1 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 172d4 jne 0x172c7
            if fl.ne() { pc = 0x172c7; continue; }
            pc = 0x172d6;
        }
        0x172d6 => {
            // block 172d6
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172d6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 172d8 jmp 0x172f3
            pc = 0x172f3; continue;
        }
        0x172da => {
            // block 172da
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172da mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 172dd mov rbx,QWORD PTR [rcx+0x10]
            rbx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 172e1 movzx esi,WORD PTR [rax+0x4]
            rsi = (m.r16(rax.wrapping_add(0x4)) as u64) as u32 as u64;
            // 172e5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 172e7 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 172e9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 172ec mov rdi,r14
            rdi = r14;
            // 172ef call rbx
            { let r = { let t = rbx; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 172f1 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x172f3;
        }
        0x172f3 => {
            // block 172f3
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172f3 lea rax,[rip+0x12a11e] # 0x141418
            rax = m.env.data_va(351205);
            // 172fa mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 172fe lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17302 call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17307 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 17309 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 1730d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1730e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17310 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17311 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x17312 => {
            // block 17312
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 17312 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17312 mov rbx,rax
            rbx = rax;
            // 17315 lea rax,[rip+0x12a0fc] # 0x141418
            rax = m.env.data_va(351205);
            // 1731c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17320 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17324 call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17329 mov rdi,rbx
            rdi = rbx;
            // 1732c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b3ec size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002b3ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2b3ec;
    loop {
        match pc {
        0x2b3ec => {
            // block 2b3ec
            #[cfg(feature = "trace")] eprintln!("f_0002b3ec pc 2b3ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b3ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b3ed mov rbp,rsp
            rbp = rsp;
            // 2b3f0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b3f1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2b3f2 mov rbx,rdi
            rbx = rdi;
            // 2b3f5 lea rdi,[rbx+0x270]
            rdi = rbx.wrapping_add(0x270);
            // 2b3fc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b3fe xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b400 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b405 lea rdi,[rbx+0x218]
            rdi = rbx.wrapping_add(0x218);
            // 2b40c call 0x2ace2
            { let r = { let t = m.env.code_va(0x2ace2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b411 lea rdi,[rbx+0x220]
            rdi = rbx.wrapping_add(0x220);
            // 2b418 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b41a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b41c call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b421 lea rdi,[rbx+0x16c]
            rdi = rbx.wrapping_add(0x16c);
            // 2b428 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b42a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b42c call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b431 lea rdi,[rbx+0xc0]
            rdi = rbx.wrapping_add(0xc0);
            // 2b438 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b43a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b43c call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b441 lea rdi,[rbx+0x98]
            rdi = rbx.wrapping_add(0x98);
            // 2b448 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b44a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b44c call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b451 lea rdi,[rbx+0x70]
            rdi = rbx.wrapping_add(0x70);
            // 2b455 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b457 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b459 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b45e lea rdi,[rbx+0x48]
            rdi = rbx.wrapping_add(0x48);
            // 2b462 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b464 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b466 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b46b add rbx,0x20
            { let (r, f) = add::<64>(rbx, 0x20u64);
              rbx = r;
              fl = f; }
            // 2b46f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b471 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b473 mov rdi,rbx
            rdi = rbx;
            // 2b476 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2b47a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b47b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b47c jmp 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68346 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068346<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68346;
    loop {
        match pc {
        0x68346 => {
            // block 68346
            #[cfg(feature = "trace")] eprintln!("f_00068346 pc 68346 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68346 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68347 mov rbp,rsp
            rbp = rsp;
            // 6834a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6834c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6834e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 68350 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 68352 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68353 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 68354 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 68357 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // 6835a mov rbx,rdi
            rbx = rdi;
            // 6835d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 68360 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 68365 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 68368 call QWORD PTR [rax+0xbe0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbe0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6836e mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 68371 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 68378 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6837b mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // 6837e lea rcx,[rip+0x999cb] # 0x101d50
            rcx = m.env.data_va(91421);
            // 68385 mov esi,DWORD PTR [rcx+r13*4]
            rsi = (m.r32(rcx.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 68389 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 6838d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 68392 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 68395 mov rdi,rbx
            rdi = rbx;
            // 68398 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6839b mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 6839e lea rcx,[rip+0x999db] # 0x101d80
            rcx = m.env.data_va(91469);
            // 683a5 mov cl,BYTE PTR [rcx+r13*4]
            rcx = (rcx & !0xff) | ((m.r8(rcx.wrapping_add(r13.wrapping_mul(4))) as u64) as u8 as u64);
            // 683a9 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 683ab and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 683ae cmp eax,r12d
            fl = sub::<32>((rax as u32 as u64), (r12 as u32 as u64)).1;
            // 683b1 je 0x683ca
            if fl.e() { pc = 0x683ca; continue; }
            pc = 0x683b3;
        }
        0x683b3 => {
            // block 683b3
            #[cfg(feature = "trace")] eprintln!("f_00068346 pc 683b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 683b3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 683b6 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 683bb mov rdi,rbx
            rdi = rbx;
            // 683be mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 683c1 call QWORD PTR [rax+0xbe0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbe0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 683c7 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x683ca;
        }
        0x683ca => {
            // block 683ca
            #[cfg(feature = "trace")] eprintln!("f_00068346 pc 683ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 683ca mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 683cd add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 683d1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 683d2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 683d4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 683d6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 683d8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 683da pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 683db ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba598 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba598<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba598;
    loop {
        match pc {
        0xba598 => {
            // block ba598
            #[cfg(feature = "trace")] eprintln!("f_000ba598 pc ba598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba598 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba599 mov rbp,rsp
            rbp = rsp;
            // ba59c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba59e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba5a0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba5a1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ba5a2 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ba5a5 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // ba5a9 lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // ba5ad mov rcx,QWORD PTR [rip+0x85dfc] # 0x1403b0
            rcx = m.r64(m.env.data_va(347005)) as u64;
            // ba5b4 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // ba5b8 mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // ba5bc lea r14,[rdi+rax*1+0x80]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x80);
            // ba5c4 lea rdx,[rcx+0x68]
            rdx = rcx.wrapping_add(0x68);
            // ba5c8 mov QWORD PTR [rdi+rax*1+0x80],rdx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x80), (rdx) as u64);
            // ba5d0 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // ba5d4 mov QWORD PTR [rdi+rax*1+0x10],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x10), (rcx) as u64);
            // ba5d9 lea rbx,[rdi+rax*1+0x18]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x18);
            // ba5de mov rcx,QWORD PTR [rip+0x85dc3] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // ba5e5 add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // ba5e9 mov QWORD PTR [rdi+rax*1+0x18],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x18), (rcx) as u64);
            // ba5ee lea rdi,[rdi+rax*1+0x58]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x58);
            // ba5f3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba5f8 mov rdi,rbx
            rdi = rbx;
            // ba5fb call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba600 mov rsi,QWORD PTR [rip+0x85bf1] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // ba607 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ba60b mov rdi,r15
            rdi = r15;
            // ba60e call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba613 mov rdi,r14
            rdi = r14;
            // ba616 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba61b mov rdi,r15
            rdi = r15;
            // ba61e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ba622 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba623 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba625 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba627 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba628 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc623e size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c623e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc623e;
    loop {
        match pc {
        0xc623e => {
            // block c623e
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c623e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c623e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c623f mov rbp,rsp
            rbp = rsp;
            // c6242 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c6244 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c6246 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c6248 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c624a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c624b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c624c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // c624f mov rbx,rdi
            rbx = rdi;
            // c6252 lea rax,[rip+0x28c1] # 0xc8b1a
            rax = m.env.code_va(0xc8b1a);
            // c6259 lea r12,[rip+0x293d] # 0xc8b9d
            r12 = m.env.code_va(0xc8b9d);
            // c6260 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c6263 cmovne r12,rax
            if fl.ne() { r12 = rax; }
            // c6267 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c626a jle 0xc62c5
            if fl.le() { pc = 0xc62c5; continue; }
            pc = 0xc626c;
        }
        0xc626c => {
            // block c626c
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c626c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c626c add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c626e and ecx,0xfffffffc
            { let r = (rcx as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c6271 movsxd rax,ecx
            rax = sx::<32>((rcx as u32 as u64)) as u64;
            // c6274 lea r13,[rsi+rax*2]
            r13 = rsi.wrapping_add(rax.wrapping_mul(2));
            // c6278 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc627b;
        }
        0xc627b => {
            // block c627b
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c627b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c627b mov rdi,rbx
            rdi = rbx;
            // c627e lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // c6282 call r12
            { let r = { let t = r12; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c6285 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // c6289 jne 0xc62ac
            if fl.ne() { pc = 0xc62ac; continue; }
            pc = 0xc628b;
        }
        0xc628b => {
            // block c628b
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c628b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c628b mov ax,WORD PTR [rbp-0x30]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u16 as u64);
            // c628f mov WORD PTR [r13+0x0],ax
            m.w16(r13, ((rax as u16 as u64)) as u16);
            // c6294 mov ax,WORD PTR [rbp-0x2e]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2)) as u64) as u16 as u64);
            // c6298 mov WORD PTR [r13+0x2],ax
            m.w16(r13.wrapping_add(0x2), ((rax as u16 as u64)) as u16);
            // c629d mov ax,WORD PTR [rbp-0x2c]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u16 as u64);
            // c62a1 mov WORD PTR [r13+0x4],ax
            m.w16(r13.wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // c62a6 add r13,0x6
            { let (r, f) = add::<64>(r13, 0x6u64);
              r13 = r;
              fl = f; }
            // c62aa jmp 0xc62b9
            pc = 0xc62b9; continue;
        }
        0xc62ac => {
            // block c62ac
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c62ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62ac mov ax,WORD PTR [rbp-0x2e]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2)) as u64) as u16 as u64);
            // c62b0 mov WORD PTR [r13+0x0],ax
            m.w16(r13, ((rax as u16 as u64)) as u16);
            // c62b5 add r13,0x2
            { let (r, f) = add::<64>(r13, 0x2u64);
              r13 = r;
              fl = f; }
            pc = 0xc62b9;
        }
        0xc62b9 => {
            // block c62b9
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c62b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62b9 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // c62bd inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // c62c0 cmp r14d,r15d
            fl = sub::<32>((r14 as u32 as u64), (r15 as u32 as u64)).1;
            // c62c3 jne 0xc627b
            if fl.ne() { pc = 0xc627b; continue; }
            pc = 0xc62c5;
        }
        0xc62c5 => {
            // block c62c5
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c62c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62c5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c62c9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c62ca pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c62cc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c62ce pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c62d0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c62d2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c62d3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd38e0 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d38e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd38e0;
    loop {
        match pc {
        0xd38e0 => {
            // block d38e0
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d38e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d38e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d38e1 mov rbp,rsp
            rbp = rsp;
            // d38e4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d38e6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d38e8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d38e9 sub rsp,0x488
            { let (r, f) = sub::<64>(rsp, 0x488u64);
              rsp = r;
              fl = f; }
            // d38f0 mov r15,rsi
            r15 = rsi;
            // d38f3 mov rbx,rdi
            rbx = rdi;
            // d38f6 test r15,r15
            fl = logic::<64>(r15 & r15);
            // d38f9 je 0xd3966
            if fl.e() { pc = 0xd3966; continue; }
            pc = 0xd38fb;
        }
        0xd38fb => {
            // block d38fb
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d38fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d38fb xorps xmm0,xmm0
            x0 = 0;
            // d38fe movaps XMMWORD PTR [rbp-0x4a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb60), x0);
            // d3905 mov QWORD PTR [rbp-0x490],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (0x0u64) as u64);
            // d3910 mov DWORD PTR [rbp-0x4a0],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb60), (0x5u64) as u32);
            // d391a mov DWORD PTR [rbp-0x49c],0x480
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb64), (0x480u64) as u32);
            // d3924 lea r14,[rbp-0x488]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // d392b mov edx,0x468
            rdx = (0x468u64) as u32 as u64;
            // d3930 mov rdi,r14
            rdi = r14;
            // d3933 mov rsi,r15
            rsi = r15;
            // d3936 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d393b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d393e lea rsi,[rbp-0x4a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // d3945 mov rdi,rbx
            rdi = rbx;
            // d3948 call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d394e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3950 je 0xd3966
            if fl.e() { pc = 0xd3966; continue; }
            pc = 0xd3952;
        }
        0xd3952 => {
            // block d3952
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d3952 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3952 mov edx,0x468
            rdx = (0x468u64) as u32 as u64;
            // d3957 mov rdi,r15
            rdi = r15;
            // d395a mov rsi,r14
            rsi = r14;
            // d395d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3962 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d3964 jmp 0xd3968
            pc = 0xd3968; continue;
        }
        0xd3966 => {
            // block d3966
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d3966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3966 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3968;
        }
        0xd3968 => {
            // block d3968
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d3968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3968 add rsp,0x488
            { let (r, f) = add::<64>(rsp, 0x488u64);
              rsp = r;
              fl = f; }
            // d396f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3970 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3972 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3974 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3975 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3976 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3976<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd3976;
    loop {
        match pc {
        0xd3976 => {
            // block d3976
            #[cfg(feature = "trace")] eprintln!("f_000d3976 pc d3976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3976 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3977 mov rbp,rsp
            rbp = rsp;
            // d397a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d397c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d397e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d397f sub rsp,0xa48
            { let (r, f) = sub::<64>(rsp, 0xa48u64);
              rsp = r;
              fl = f; }
            // d3986 mov r15,rsi
            r15 = rsi;
            // d3989 mov rbx,rdi
            rbx = rdi;
            // d398c test r15,r15
            fl = logic::<64>(r15 & r15);
            // d398f je 0xd39fc
            if fl.e() { pc = 0xd39fc; continue; }
            pc = 0xd3991;
        }
        0xd3991 => {
            // block d3991
            #[cfg(feature = "trace")] eprintln!("f_000d3976 pc d3991 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3991 xorps xmm0,xmm0
            x0 = 0;
            // d3994 movaps XMMWORD PTR [rbp-0xa60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f5a0), x0);
            // d399b mov QWORD PTR [rbp-0xa50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5b0), (0x0u64) as u64);
            // d39a6 mov DWORD PTR [rbp-0xa60],0x6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f5a0), (0x6u64) as u32);
            // d39b0 mov DWORD PTR [rbp-0xa5c],0xa40
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f5a4), (0xa40u64) as u32);
            // d39ba lea r14,[rbp-0xa48]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_f5b8);
            // d39c1 mov edx,0xa28
            rdx = (0xa28u64) as u32 as u64;
            // d39c6 mov rdi,r14
            rdi = r14;
            // d39c9 mov rsi,r15
            rsi = r15;
            // d39cc call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d39d1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d39d4 lea rsi,[rbp-0xa60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f5a0);
            // d39db mov rdi,rbx
            rdi = rbx;
            // d39de call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d39e4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d39e6 je 0xd39fc
            if fl.e() { pc = 0xd39fc; continue; }
            pc = 0xd39e8;
        }
        0xd39e8 => {
            // block d39e8
            #[cfg(feature = "trace")] eprintln!("f_000d3976 pc d39e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d39e8 mov edx,0xa28
            rdx = (0xa28u64) as u32 as u64;
            // d39ed mov rdi,r15
            rdi = r15;
            // d39f0 mov rsi,r14
            rsi = r14;
            // d39f3 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d39f8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d39fa jmp 0xd39fe
            pc = 0xd39fe; continue;
        }
        0xd39fc => {
            // block d39fc
            #[cfg(feature = "trace")] eprintln!("f_000d3976 pc d39fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d39fc xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd39fe;
        }
        0xd39fe => {
            // block d39fe
            #[cfg(feature = "trace")] eprintln!("f_000d3976 pc d39fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d39fe add rsp,0xa48
            { let (r, f) = add::<64>(rsp, 0xa48u64);
              rsp = r;
              fl = f; }
            // d3a05 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3a06 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3a08 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3a0a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3a0b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe59da size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e59da<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe59da;
    loop {
        match pc {
        0xe59da => {
            // block e59da
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e59da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e59da push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e59db mov rbp,rsp
            rbp = rsp;
            // e59de push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e59e0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e59e1 mov r11b,BYTE PTR [rbp+0x18]
            r11 = (r11 & !0xff) | ((m.r8(rbp.wrapping_add(0x18)) as u64) as u8 as u64);
            // e59e5 mov r10b,BYTE PTR [rbp+0x10]
            r10 = (r10 & !0xff) | ((m.r8(rbp.wrapping_add(0x10)) as u64) as u8 as u64);
            // e59e9 lea ebx,[r9-0x1]
            rbx = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // e59ed mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            // e59f2 cmp ebx,0x8
            fl = sub::<32>((rbx as u32 as u64), 0x8u64).1;
            // e59f5 ja 0xe5a04
            if fl.a() { pc = 0xe5a04; continue; }
            pc = 0xe59f7;
        }
        0xe59f7 => {
            // block e59f7
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e59f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e59f7 movsxd rax,ebx
            rax = sx::<32>((rbx as u32 as u64)) as u64;
            // e59fa lea rbx,[rip+0x442af] # 0x129cb0
            rbx = m.env.data_va(255101);
            // e5a01 mov eax,DWORD PTR [rbx+rax*4]
            rax = (m.r32(rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe5a04;
        }
        0xe5a04 => {
            // block e5a04
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a04 add r9d,0xfffffffd
            { let (r, f) = add::<32>((r9 as u32 as u64), 0xfffffffdu64);
              r9 = (r) as u32 as u64;
              fl = f; }
            // e5a08 cmp r9d,0x3
            fl = sub::<32>((r9 as u32 as u64), 0x3u64).1;
            // e5a0c ja 0xe5a38
            if fl.a() { pc = 0xe5a38; continue; }
            pc = 0xe5a0e;
        }
        0xe5a0e => {
            // block e5a0e
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a0e lea r14,[rip+0x4b] # 0xe5a60
            r14 = m.env.code_va(0xe5a60);
            // e5a15 movsxd rbx,DWORD PTR [r14+r9*4]
            rbx = sx::<32>(m.r32(r14.wrapping_add(r9.wrapping_mul(4))) as u64) as u64;
            // e5a19 add rbx,r14
            { let (r, f) = add::<64>(rbx, r14);
              rbx = r;
              fl = f; }
            // e5a1c jmp rbx
            pc = (rbx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe5a1e => {
            // block e5a1e
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a1e test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // e5a21 je 0xe5a38
            if fl.e() { pc = 0xe5a38; continue; }
            pc = 0xe5a23;
        }
        0xe5a23 => {
            // block e5a23
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a23 jmp 0xe5a3d
            pc = 0xe5a3d; continue;
        }
        0xe5a25 => {
            // block e5a25
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a25 mov r9d,0x6
            r9 = (0x6u64) as u32 as u64;
            // e5a2b jmp 0xe5a33
            pc = 0xe5a33; continue;
        }
        0xe5a2d => {
            // block e5a2d
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a2d mov r9d,0x5
            r9 = (0x5u64) as u32 as u64;
            pc = 0xe5a33;
        }
        0xe5a33 => {
            // block e5a33
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a33 test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // e5a36 jne 0xe5a57
            if fl.ne() { pc = 0xe5a57; continue; }
            pc = 0xe5a38;
        }
        0xe5a38 => {
            // block e5a38
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a38 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e5a3b jne 0xe5a47
            if fl.ne() { pc = 0xe5a47; continue; }
            pc = 0xe5a3d;
        }
        0xe5a3d => {
            // block e5a3d
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a3d movzx r9d,r10b
            r9 = ((r10 as u8 as u64)) as u32 as u64;
            // e5a41 add r9d,0x7
            { let (r, f) = add::<32>((r9 as u32 as u64), 0x7u64);
              r9 = (r) as u32 as u64;
              fl = f; }
            // e5a45 jmp 0xe5a57
            pc = 0xe5a57; continue;
        }
        0xe5a47 => {
            // block e5a47
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a47 cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e5a4a mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // e5a4d jne 0xe5a57
            if fl.ne() { pc = 0xe5a57; continue; }
            pc = 0xe5a4f;
        }
        0xe5a4f => {
            // block e5a4f
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a4f movzx r9d,r10b
            r9 = ((r10 as u8 as u64)) as u32 as u64;
            // e5a53 add r9d,0x3
            { let (r, f) = add::<32>((r9 as u32 as u64), 0x3u64);
              r9 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe5a57;
        }
        0xe5a57 => {
            // block e5a57
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a57 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e5a58 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e5a5a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5a5b jmp 0xe4cd0
            { let r = { let t = m.env.code_va(0xe4cd0); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68a10 size=152
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068a10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68a10;
    loop {
        match pc {
        0x68a10 => {
            // block 68a10
            #[cfg(feature = "trace")] eprintln!("f_00068a10 pc 68a10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68a10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68a11 mov rbp,rsp
            rbp = rsp;
            // 68a14 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68a15 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 68a16 mov rbx,rdi
            rbx = rdi;
            // 68a19 lea rdi,[rbx+0x1f8]
            rdi = rbx.wrapping_add(0x1f8);
            // 68a20 mov rsi,QWORD PTR [rbx+0x200]
            rsi = m.r64(rbx.wrapping_add(0x200)) as u64;
            // 68a27 call 0xebc48
            { let r = { let t = m.env.stub_va(0x415); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a2c lea rdi,[rbx+0x1e0]
            rdi = rbx.wrapping_add(0x1e0);
            // 68a33 mov rsi,QWORD PTR [rbx+0x1e8]
            rsi = m.r64(rbx.wrapping_add(0x1e8)) as u64;
            // 68a3a call 0xebc1e
            { let r = { let t = m.env.stub_va(0x3eb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a3f lea rdi,[rbx+0x1c8]
            rdi = rbx.wrapping_add(0x1c8);
            // 68a46 mov rsi,QWORD PTR [rbx+0x1d0]
            rsi = m.r64(rbx.wrapping_add(0x1d0)) as u64;
            // 68a4d call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a52 lea rdi,[rbx+0x1b0]
            rdi = rbx.wrapping_add(0x1b0);
            // 68a59 mov rsi,QWORD PTR [rbx+0x1b8]
            rsi = m.r64(rbx.wrapping_add(0x1b8)) as u64;
            // 68a60 call 0xebc36
            { let r = { let t = m.env.stub_va(0x403); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a65 lea rdi,[rbx+0x198]
            rdi = rbx.wrapping_add(0x198);
            // 68a6c mov rsi,QWORD PTR [rbx+0x1a0]
            rsi = m.r64(rbx.wrapping_add(0x1a0)) as u64;
            // 68a73 call 0xebc36
            { let r = { let t = m.env.stub_va(0x403); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a78 lea rdi,[rbx+0x180]
            rdi = rbx.wrapping_add(0x180);
            // 68a7f mov rsi,QWORD PTR [rbx+0x188]
            rsi = m.r64(rbx.wrapping_add(0x188)) as u64;
            // 68a86 call 0xebc8a
            { let r = { let t = m.env.stub_va(0x457); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a8b mov rsi,QWORD PTR [rbx+0x170]
            rsi = m.r64(rbx.wrapping_add(0x170)) as u64;
            // 68a92 add rbx,0x168
            { let (r, f) = add::<64>(rbx, 0x168u64);
              rbx = r;
              fl = f; }
            // 68a99 mov rdi,rbx
            rdi = rbx;
            // 68a9c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 68aa0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68aa1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68aa2 jmp 0xebc6c
            { let r = { let t = m.env.stub_va(0x439); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4598 size=152
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4598<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd4598;
    loop {
        match pc {
        0xd4598 => {
            // block d4598
            #[cfg(feature = "trace")] eprintln!("f_000d4598 pc d4598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4598 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4599 mov rbp,rsp
            rbp = rsp;
            // d459c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d459e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d45a0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d45a2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d45a3 sub rsp,0x80
            { let (r, f) = sub::<64>(rsp, 0x80u64);
              rsp = r;
              fl = f; }
            // d45aa mov r15,rcx
            r15 = rcx;
            // d45ad mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d45b0 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // d45b3 mov rbx,rdi
            rbx = rdi;
            // d45b6 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // d45b9 mov QWORD PTR [r15+0x20],rax
            m.w64(r15.wrapping_add(0x20), (rax) as u64);
            // d45bd mov DWORD PTR [r15+0x28],r14d
            m.w32(r15.wrapping_add(0x28), ((r14 as u32 as u64)) as u32);
            // d45c1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d45c4 mov rsi,r15
            rsi = r15;
            // d45c7 call QWORD PTR [rax+0xd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xd0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d45cd mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d45cf mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d45d1 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d45d3 jne 0xd461f
            if fl.ne() { pc = 0xd461f; continue; }
            pc = 0xd45d5;
        }
        0xd45d5 => {
            // block d45d5
            #[cfg(feature = "trace")] eprintln!("f_000d4598 pc d45d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d45d5 mov DWORD PTR [rbp-0x24],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x12u64) as u32);
            // d45dc lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // d45e0 mov rdi,rbx
            rdi = rbx;
            // d45e3 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // d45e6 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d45eb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d45ed je 0xd461d
            if fl.e() { pc = 0xd461d; continue; }
            pc = 0xd45ef;
        }
        0xd45ef => {
            // block d45ef
            #[cfg(feature = "trace")] eprintln!("f_000d4598 pc d45ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d45ef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d45f2 mov esi,DWORD PTR [rbp-0x24]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // d45f5 lea rcx,[rbp-0x98]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // d45fc mov rdi,rbx
            rdi = rbx;
            // d45ff mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d4602 call QWORD PTR [rax+0xc10]
            { let r = { let t = m.r64(rax.wrapping_add(0xc10)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4608 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d460a je 0xd461d
            if fl.e() { pc = 0xd461d; continue; }
            pc = 0xd460c;
        }
        0xd460c => {
            // block d460c
            #[cfg(feature = "trace")] eprintln!("f_000d4598 pc d460c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d460c lea rsi,[rbp-0x98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // d4613 mov rdi,r15
            rdi = r15;
            // d4616 call 0x29928
            { let r = { let t = m.env.code_va(0x29928); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d461b jmp 0xd461f
            pc = 0xd461f; continue;
        }
        0xd461d => {
            // block d461d
            #[cfg(feature = "trace")] eprintln!("f_000d4598 pc d461d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d461d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd461f;
        }
        0xd461f => {
            // block d461f
            #[cfg(feature = "trace")] eprintln!("f_000d4598 pc d461f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d461f add rsp,0x80
            { let (r, f) = add::<64>(rsp, 0x80u64);
              rsp = r;
              fl = f; }
            // d4626 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4627 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4629 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d462b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d462d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d462e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2ef8 size=152
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2ef8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2ef8;
    loop {
        match pc {
        0xe2ef8 => {
            // block e2ef8
            #[cfg(feature = "trace")] eprintln!("f_000e2ef8 pc e2ef8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2ef8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2ef9 mov rbp,rsp
            rbp = rsp;
            // e2efc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2efe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2f00 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2f02 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2f03 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2f07 mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // e2f0a mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // e2f0d mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e2f0f mov r12,rdi
            r12 = rdi;
            // e2f12 mov rdi,QWORD PTR [r12]
            rdi = m.r64(r12) as u64;
            // e2f16 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2f19 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // e2f1d lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // e2f21 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // e2f24 je 0xe2f2d
            if fl.e() { pc = 0xe2f2d; continue; }
            pc = 0xe2f26;
        }
        0xe2f26 => {
            // block e2f26
            #[cfg(feature = "trace")] eprintln!("f_000e2ef8 pc e2f26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2f26 mov esi,0x4001c
            rsi = (0x4001cu64) as u32 as u64;
            // e2f2b jmp 0xe2f32
            pc = 0xe2f32; continue;
        }
        0xe2f2d => {
            // block e2f2d
            #[cfg(feature = "trace")] eprintln!("f_000e2ef8 pc e2f2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2f2d mov esi,0x4001b
            rsi = (0x4001bu64) as u32 as u64;
            pc = 0xe2f32;
        }
        0xe2f32 => {
            // block e2f32
            #[cfg(feature = "trace")] eprintln!("f_000e2ef8 pc e2f32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2f32 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2f37 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2f3a call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2f3c shl ebx,0x3
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x3u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e2f3f mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // e2f44 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2f46 shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e2f48 not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // e2f4a and eax,DWORD PTR [rbp-0x24]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e2f4d movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // e2f51 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2f53 shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e2f55 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e2f57 mov DWORD PTR [rbp-0x24],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rdx as u32 as u64)) as u32);
            // e2f5a mov rdi,QWORD PTR [r12]
            rdi = m.r64(r12) as u64;
            // e2f5e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2f61 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e2f65 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // e2f68 je 0xe2f71
            if fl.e() { pc = 0xe2f71; continue; }
            pc = 0xe2f6a;
        }
        0xe2f6a => {
            // block e2f6a
            #[cfg(feature = "trace")] eprintln!("f_000e2ef8 pc e2f6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2f6a mov esi,0x4001c
            rsi = (0x4001cu64) as u32 as u64;
            // e2f6f jmp 0xe2f76
            pc = 0xe2f76; continue;
        }
        0xe2f71 => {
            // block e2f71
            #[cfg(feature = "trace")] eprintln!("f_000e2ef8 pc e2f71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2f71 mov esi,0x4001b
            rsi = (0x4001bu64) as u32 as u64;
            pc = 0xe2f76;
        }
        0xe2f76 => {
            // block e2f76
            #[cfg(feature = "trace")] eprintln!("f_000e2ef8 pc e2f76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2f76 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2f7b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2f7e call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2f80 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e2f82 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2f86 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2f87 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2f89 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2f8b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2f8d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2f8e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x17202 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017202<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x17202;
    loop {
        match pc {
        0x17202 => {
            // block 17202
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 17202 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17202 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17203 mov rbp,rsp
            rbp = rsp;
            // 17206 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 17208 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 17209 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 1720d mov r14,rsi
            r14 = rsi;
            // 17210 mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // 17212 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 17216 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 1721b call 0x16080
            { let r = { let t = m.env.code_va(0x16080); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17220 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 17222 mov rcx,QWORD PTR [rbp-0x20]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 17226 sub rcx,QWORD PTR [rbp-0x28]
            { let (r, f) = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64);
              rcx = r;
              fl = f; }
            // 1722a sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 1722e movabs rdx,0xef7bdef7bdef7bdf
            rdx = 0xef7b_def7_bdef_7bdfu64;
            // 17238 imul rdx,rcx
            { let (r, f) = imul::<64>(rdx, rcx); rdx = r; fl = f; }
            // 1723c cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 1723f jae 0x1727a
            if fl.ae() { pc = 0x1727a; continue; }
            pc = 0x17241;
        }
        0x17241 => {
            // block 17241
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 17241 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17241 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 17244 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 17248 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // 1724b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1724d xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 1724f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 17252 mov rdi,r14
            rdi = r14;
            // 17255 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17257 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17259 jmp 0x1727c
            pc = 0x1727c; continue;
        }
        0x1725b => {
            // block 1725b
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 1725b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1725b mov rbx,rax
            rbx = rax;
            // 1725e lea rax,[rip+0x12a1b3] # 0x141418
            rax = m.env.data_va(351205);
            // 17265 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17269 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 1726d call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17272 mov rdi,rbx
            rdi = rbx;
            // 17275 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1727a;
        }
        0x1727a => {
            // block 1727a
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 1727a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1727a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x1727c;
        }
        0x1727c => {
            // block 1727c
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 1727c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1727c lea rax,[rip+0x12a195] # 0x141418
            rax = m.env.data_va(351205);
            // 17283 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17287 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 1728b call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17290 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 17292 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 17296 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 17297 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17299 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1729a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64b68 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064b68<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x64b68;
    loop {
        match pc {
        0x64b68 => {
            // block 64b68
            #[cfg(feature = "trace")] eprintln!("f_00064b68 pc 64b68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64b68 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64b69 mov rbp,rsp
            rbp = rsp;
            // 64b6c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 64b6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 64b70 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 64b72 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 64b73 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 64b77 mov r12,rsi
            r12 = rsi;
            // 64b7a mov r14,rdi
            r14 = rdi;
            // 64b7d lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 64b81 mov rdx,r12
            rdx = r12;
            // 64b84 call 0xebb8e
            { let r = { let t = m.env.stub_va(0x35b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64b89 mov r15,rax
            r15 = rax;
            // 64b8c mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 64b8f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 64b92 jne 0x64bee
            if fl.ne() { pc = 0x64bee; continue; }
            pc = 0x64b94;
        }
        0x64b94 => {
            // block 64b94
            #[cfg(feature = "trace")] eprintln!("f_00064b68 pc 64b94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64b94 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // 64b99 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64b9e mov rbx,rax
            rbx = rax;
            // 64ba1 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 64ba6 mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // 64baa mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 64bae mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 64bb3 mov QWORD PTR [rbx+0x28],rcx
            m.w64(rbx.wrapping_add(0x28), (rcx) as u64);
            // 64bb7 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // 64bbb mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 64bc4 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 64bcd mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // 64bd5 mov DWORD PTR [rbx+0x38],0x0
            m.w32(rbx.wrapping_add(0x38), (0x0u64) as u32);
            // 64bdc mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 64be0 mov rdi,r14
            rdi = r14;
            // 64be3 mov rdx,r15
            rdx = r15;
            // 64be6 mov rcx,rbx
            rcx = rbx;
            // 64be9 call 0xeb930
            { let r = { let t = m.env.stub_va(0xfd); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x64bee;
        }
        0x64bee => {
            // block 64bee
            #[cfg(feature = "trace")] eprintln!("f_00064b68 pc 64bee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64bee add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 64bf2 mov rax,rbx
            rax = rbx;
            // 64bf5 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 64bf9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64bfa pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 64bfc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 64bfe pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64c00 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64c01 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3b36 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3b36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3b36;
    loop {
        match pc {
        0xc3b36 => {
            // block c3b36
            #[cfg(feature = "trace")] eprintln!("f_000c3b36 pc c3b36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3b36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3b37 mov rbp,rsp
            rbp = rsp;
            // c3b3a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3b3c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3b3e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3b40 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3b41 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3b45 mov r12,rsi
            r12 = rsi;
            // c3b48 mov r14,rdi
            r14 = rdi;
            // c3b4b lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3b4f mov rdx,r12
            rdx = r12;
            // c3b52 call 0xebd3e
            { let r = { let t = m.env.stub_va(0x50b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b57 mov r15,rax
            r15 = rax;
            // c3b5a mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3b5d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3b60 jne 0xc3bbc
            if fl.ne() { pc = 0xc3bbc; continue; }
            pc = 0xc3b62;
        }
        0xc3b62 => {
            // block c3b62
            #[cfg(feature = "trace")] eprintln!("f_000c3b36 pc c3b62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3b62 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c3b67 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b6c mov rbx,rax
            rbx = rax;
            // c3b6f mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // c3b74 mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // c3b78 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // c3b7c mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // c3b81 mov QWORD PTR [rbx+0x28],rcx
            m.w64(rbx.wrapping_add(0x28), (rcx) as u64);
            // c3b85 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // c3b89 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // c3b92 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // c3b9b mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // c3ba3 mov DWORD PTR [rbx+0x38],0x0
            m.w32(rbx.wrapping_add(0x38), (0x0u64) as u32);
            // c3baa mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3bae mov rdi,r14
            rdi = r14;
            // c3bb1 mov rdx,r15
            rdx = r15;
            // c3bb4 mov rcx,rbx
            rcx = rbx;
            // c3bb7 call 0xebe64
            { let r = { let t = m.env.stub_va(0x631); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3bbc;
        }
        0xc3bbc => {
            // block c3bbc
            #[cfg(feature = "trace")] eprintln!("f_000c3b36 pc c3bbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3bbc add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // c3bc0 mov rax,rbx
            rax = rbx;
            // c3bc3 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3bc7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3bc8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3bca pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3bcc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3bce pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3bcf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3cbc size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3cbc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3cbc;
    loop {
        match pc {
        0xc3cbc => {
            // block c3cbc
            #[cfg(feature = "trace")] eprintln!("f_000c3cbc pc c3cbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3cbc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3cbd mov rbp,rsp
            rbp = rsp;
            // c3cc0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3cc2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3cc4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3cc6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3cc7 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3ccb mov r12,rsi
            r12 = rsi;
            // c3cce mov r14,rdi
            r14 = rdi;
            // c3cd1 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3cd5 mov rdx,r12
            rdx = r12;
            // c3cd8 call 0xebd56
            { let r = { let t = m.env.stub_va(0x523); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3cdd mov r15,rax
            r15 = rax;
            // c3ce0 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3ce3 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3ce6 jne 0xc3d42
            if fl.ne() { pc = 0xc3d42; continue; }
            pc = 0xc3ce8;
        }
        0xc3ce8 => {
            // block c3ce8
            #[cfg(feature = "trace")] eprintln!("f_000c3cbc pc c3ce8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3ce8 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c3ced call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3cf2 mov rbx,rax
            rbx = rax;
            // c3cf5 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // c3cfa mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // c3cfe mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // c3d02 mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // c3d07 mov QWORD PTR [rbx+0x28],rcx
            m.w64(rbx.wrapping_add(0x28), (rcx) as u64);
            // c3d0b mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // c3d0f mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // c3d18 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // c3d21 mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // c3d29 mov DWORD PTR [rbx+0x38],0x0
            m.w32(rbx.wrapping_add(0x38), (0x0u64) as u32);
            // c3d30 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3d34 mov rdi,r14
            rdi = r14;
            // c3d37 mov rdx,r15
            rdx = r15;
            // c3d3a mov rcx,rbx
            rcx = rbx;
            // c3d3d call 0xebe7c
            { let r = { let t = m.env.stub_va(0x649); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3d42;
        }
        0xc3d42 => {
            // block c3d42
            #[cfg(feature = "trace")] eprintln!("f_000c3cbc pc c3d42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3d42 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // c3d46 mov rax,rbx
            rax = rbx;
            // c3d49 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3d4d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3d4e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3d50 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3d52 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3d54 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3d55 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdedb8 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dedb8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdedb8;
    loop {
        match pc {
        0xdedb8 => {
            // block dedb8
            #[cfg(feature = "trace")] eprintln!("f_000dedb8 pc dedb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dedb8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dedb9 mov rbp,rsp
            rbp = rsp;
            // dedbc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dedbe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dedc0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dedc2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dedc3 mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // dedc6 mov r14,rdx
            r14 = rdx;
            // dedc9 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // dedcc mov rbx,rdi
            rbx = rdi;
            // dedcf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dedd2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dedd4 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dedd9 call QWORD PTR [rax+0x368]
            { let r = { let t = m.r64(rax.wrapping_add(0x368)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // deddf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dede2 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dede7 mov rdi,rbx
            rdi = rbx;
            // dedea mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // deded mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // dedf0 call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dedf6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dedf9 mov rdi,rbx
            rdi = rbx;
            // dedfc mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // dedff call QWORD PTR [rax+0x1908]
            { let r = { let t = m.r64(rax.wrapping_add(0x1908)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee05 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee08 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // dee0d mov rdi,rbx
            rdi = rbx;
            // dee10 call QWORD PTR [rax+0x1940]
            { let r = { let t = m.r64(rax.wrapping_add(0x1940)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee16 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee19 mov rdi,rbx
            rdi = rbx;
            // dee1c mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // dee1f call QWORD PTR [rax+0x18e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x18e8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee25 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee28 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dee2a mov rdi,rbx
            rdi = rbx;
            // dee2d mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // dee30 call QWORD PTR [rax+0x4b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x4b0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee36 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee39 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dee3b mov rdi,rbx
            rdi = rbx;
            // dee3e mov rsi,r14
            rsi = r14;
            // dee41 call QWORD PTR [rax+0x19c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19c8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee47 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // dee49 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dee4a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dee4c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dee4e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dee50 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dee51 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
